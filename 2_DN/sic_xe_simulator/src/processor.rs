extern crate chrono;
extern crate timer;

use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use crate::{
    machine::{opcodes::Opcode, Machine},
    sic_xe::{
        get_format_sic_f3_f4_bits, get_r1_r2, i24_to_u8arr, is_format_f3, is_format_f4,
        is_format_sic, is_immediate, resolve_address, u8arr_to_i24, FormatSicF3F4Bits,
    },
};

const MAX_HZ: i64 = 1_000_000_000;

pub struct Processor {
    pub machine: Machine,

    /// speed in Hz
    speed: i64,

    timer: timer::Timer,
    guard: Option<timer::Guard>,
}

pub type ProcessorHandle = Arc<Mutex<Processor>>;

impl Processor {
    pub fn new_handle() -> ProcessorHandle { Arc::new(Mutex::new(Processor::new())) }
    fn new() -> Self {
        Self { machine: Machine::new(), speed: 100, timer: timer::Timer::new(), guard: None }
    }

    fn execute_instruction(&mut self) -> () {
        let byte = self.fetch();
        let opcode = match Opcode::from_byte(byte & 0xFC) {
            Some(opcode) => opcode,
            None => {
                Processor::invalid_opcode(byte);
                return;
            }
        };
        println!("\n{:?}", opcode);
        // println!("\nbyte1={:08b}", byte);

        if self.exec_f1(&opcode) {
            self.print_state();
            return;
        }

        let operand = self.fetch();
        // println!("byte2={:08b}", operand);
        if self.exec_f2(&opcode, &operand) {
            self.print_state();
            return;
        }

        let third_byte = self.fetch();
        // println!("byte3={:08b}", third_byte);
        if !self.exec_sic_f3_f4(&opcode, &byte, &operand, &third_byte) {
            panic!("??? exec failed. Looks like I forgot to add a opcode to arms!?");
        }
        self.print_state();
    }

    fn print_state(&self) -> () {
        println!("STATE:\n-------------------------");
        for i in 0..100 {
            print!("{:0>2x} ", self.machine.memory.get_byte(i));
        }
        println!("\na={}", self.machine.registers.get_a());
        println!("b={}", self.machine.registers.get_b());
        println!("x={}", self.machine.registers.get_x());
        println!("s={}", self.machine.registers.get_s());
        println!("t={}", self.machine.registers.get_t());
        println!("sw={}", self.machine.registers.get_sw());
        println!("-------------------------\n");
    }

    /// fetch next 8b and pc++
    fn fetch(&mut self) -> u8 {
        let pc = self.machine.registers.get_pc();
        // println!("pc={}", pc);
        self.machine.registers.set_pc(pc + 1);
        self.machine.memory.get_byte(pc.try_into().unwrap())
    }

    /// opcode: 8b
    /// return:
    /// \   true -> executed F1
    /// \   false -> not F1
    fn exec_f1(&mut self, opcode: &Opcode) -> bool {
        match opcode {
            Opcode::Float => {
                self.machine.registers.set_f(self.machine.registers.get_a().try_into().unwrap());
            }
            Opcode::Fix => Processor::not_implemented("FIX"),
            Opcode::Norm => Processor::not_implemented("NORM"),
            Opcode::Sio => Processor::not_implemented("SIO"),
            Opcode::Hio => Processor::not_implemented("HIO"),
            Opcode::Tio => Processor::not_implemented("TIO"),
            _ => return false,
        };

        true
    }
    /// opcode: 8b
    /// operand: 4b,4b == r1,r2
    /// return:
    /// \   true -> executed F2
    /// \   false -> not F2
    fn exec_f2(&mut self, opcode: &Opcode, operand: &u8) -> bool {
        // make sure its one of the opcodes
        if !matches!(
            opcode,
            Opcode::Addr
                | Opcode::Subr
                | Opcode::Mulr
                | Opcode::Divr
                | Opcode::Compr
                | Opcode::Shiftl
                | Opcode::Shiftr
                | Opcode::Rmo
                | Opcode::Clear
                | Opcode::Tixr
                | Opcode::Svc
        ) {
            return false;
        };

        let (r1, r2) = get_r1_r2(&operand);
        let r1_val = self.machine.registers.get_reg(r1.try_into().unwrap());
        let r2_val = self.machine.registers.get_reg(r2.try_into().unwrap());

        match opcode {
            Opcode::Addr => self.machine.registers.set_reg(r2.try_into().unwrap(), r2_val + r1_val),
            Opcode::Subr => self.machine.registers.set_reg(r2.try_into().unwrap(), r2_val - r1_val),
            Opcode::Mulr => self.machine.registers.set_reg(r2.try_into().unwrap(), r2_val * r1_val),
            Opcode::Divr => self.machine.registers.set_reg(r2.try_into().unwrap(), r2_val / r1_val),
            Opcode::Compr => self.machine.registers.set_sw(match r1_val.cmp(&r2_val) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            }),
            Opcode::Shiftl => {
                self.machine.registers.set_reg(r1.try_into().unwrap(), r1_val << r2_val);
            }
            Opcode::Shiftr => {
                self.machine.registers.set_reg(r1.try_into().unwrap(), r1_val >> r2_val);
            }
            Opcode::Rmo => self.machine.registers.set_reg(r2.try_into().unwrap(), r1_val),
            Opcode::Clear => self.machine.registers.set_reg(r1.try_into().unwrap(), 0),
            Opcode::Tixr => {
                self.machine.registers.set_x(self.machine.registers.get_x() + 1);
                self.machine.registers.set_sw(match self.machine.registers.get_x().cmp(&r1_val) {
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                });
            }
            Opcode::Svc => Processor::not_implemented("SVC"),
            _ => return false,
        };

        true
    }
    /// opcode: 6b
    /// ni: 1b,1b == n,i
    /// \   0, 0 -> SIC
    /// \   else -> F3 or F4
    /// operand: 16b or 24b
    /// \   SIC  -> 1b,15b == x,addr
    /// \   F3   -> 1b,1b,1b,1b,12b == x,b,p,e,offset
    /// \   F4   -> 1b,1b,1b,1b,20b == x,b,p,e,addr
    fn exec_sic_f3_f4(
        &mut self,
        opcode: &Opcode,
        first_byte: &u8,
        second_byte: &u8,
        third_byte: &u8,
    ) -> bool {
        let bits = get_format_sic_f3_f4_bits(&first_byte, &second_byte);
        let addr = {
            if is_format_sic(&bits) {
                ((second_byte & 0x7F) as u32) << 8 | *third_byte as u32
            } else if is_format_f3(&bits) {
                ((second_byte & 0x0F) as u32) << 8 | *third_byte as u32
            } else if is_format_f4(&bits) {
                let fourth_byte = self.fetch();
                // println!("byte4={:08b}\n", fourth_byte);
                ((second_byte & 0x0F) as u32) << 16 | (*third_byte as u32) << 8 | fourth_byte as u32
            } else {
                panic!("INVALID STATE");
            }
        } as usize;
        println!("bits={}", bits);

        match opcode {
            // ***** immediate addressing not possible *****
            // stores
            Opcode::Sta => {
                Processor::store_word(
                    &bits,
                    addr,
                    self.machine.registers.get_a_as_bytes(),
                    &mut self.machine,
                );
            }
            Opcode::Stx => {
                Processor::store_word(
                    &bits,
                    addr,
                    self.machine.registers.get_x_as_bytes(),
                    &mut self.machine,
                );
            }
            Opcode::Stl => {
                Processor::store_word(
                    &bits,
                    addr,
                    self.machine.registers.get_l_as_bytes(),
                    &mut self.machine,
                );
            }
            Opcode::Stch => {
                Processor::store_byte(
                    &bits,
                    addr,
                    self.machine.registers.get_a_as_bytes()[2],
                    &mut self.machine,
                );
            }
            Opcode::Stb => {
                Processor::store_word(
                    &bits,
                    addr,
                    self.machine.registers.get_b_as_bytes(),
                    &mut self.machine,
                );
            }
            Opcode::Sts => {
                Processor::store_word(
                    &bits,
                    addr,
                    self.machine.registers.get_s_as_bytes(),
                    &mut self.machine,
                );
            }
            Opcode::Stf => Processor::not_implemented("STF"),
            Opcode::Stt => {
                Processor::store_word(
                    &bits,
                    addr,
                    self.machine.registers.get_t_as_bytes(),
                    &mut self.machine,
                );
            }
            Opcode::Stsw => {
                Processor::store_word(
                    &bits,
                    addr,
                    self.machine.registers.get_sw_as_bytes(),
                    &mut self.machine,
                );
            }

            // jumps
            Opcode::Jeq => {
                if self.machine.registers.get_sw() == 0 {
                    let address = resolve_address(&bits, addr, &mut self.machine) as i32;
                    self.machine.registers.set_pc(address);
                }
            }
            Opcode::Jgt => {
                if self.machine.registers.get_sw() == 1 {
                    let address = resolve_address(&bits, addr, &mut self.machine) as i32;
                    self.machine.registers.set_pc(address);
                }
            }
            Opcode::Jlt => {
                if self.machine.registers.get_sw() == -1 {
                    let address = resolve_address(&bits, addr, &mut self.machine) as i32;
                    self.machine.registers.set_pc(address);
                }
            }
            Opcode::J => {
                let address = resolve_address(&bits, addr, &mut self.machine) as i32;
                self.machine.registers.set_pc(address);
            }
            Opcode::Rsub => {
                self.machine.registers.set_pc(self.machine.registers.get_l());
            }
            Opcode::Jsub => {
                self.machine.registers.set_l(self.machine.registers.get_pc());
                self.machine.registers.set_pc(addr as i32);
            }

            // ***** immediate addressing possible *****
            // loads
            Opcode::Lda => {
                let word = Processor::load_word(&bits, addr, &mut self.machine);
                self.machine.registers.set_a_as_bytes(word);
                println!("a is now={}", self.machine.registers.get_a());
            }
            Opcode::Ldx => {
                let word = Processor::load_word(&bits, addr, &mut self.machine);
                self.machine.registers.set_x_as_bytes(word);
            }
            Opcode::Ldl => {
                let word = Processor::load_word(&bits, addr, &mut self.machine);
                self.machine.registers.set_l_as_bytes(word);
            }
            Opcode::Ldch => {
                let current_bytes = self.machine.registers.get_a_as_bytes();
                let byte = Processor::load_byte(&bits, addr, &mut self.machine);
                let new_bytes: [u8; 3] = [current_bytes[0], current_bytes[1], byte];
                self.machine.registers.set_a_as_bytes(new_bytes);
            }
            Opcode::Ldb => {
                let word = Processor::load_word(&bits, addr, &mut self.machine);
                self.machine.registers.set_b_as_bytes(word);
            }
            Opcode::Lds => {
                let word = Processor::load_word(&bits, addr, &mut self.machine);
                self.machine.registers.set_s_as_bytes(word);
            }
            Opcode::Ldf => Processor::not_implemented("LDF"),
            Opcode::Ldt => {
                let word = Processor::load_word(&bits, addr, &mut self.machine);
                self.machine.registers.set_t_as_bytes(word);
            }

            // arithmetic
            Opcode::Add => {
                let word = u8arr_to_i24(Processor::load_word(&bits, addr, &mut self.machine));
                println!("ADDING {} + {}", self.machine.registers.get_a(), word);
                self.machine.registers.set_a(self.machine.registers.get_a() + word);
            }
            Opcode::Sub => {
                let word = u8arr_to_i24(Processor::load_word(&bits, addr, &mut self.machine));
                self.machine.registers.set_a(self.machine.registers.get_a() - word);
            }
            Opcode::Mul => {
                let word = u8arr_to_i24(Processor::load_word(&bits, addr, &mut self.machine));
                self.machine.registers.set_a(self.machine.registers.get_a() * word);
            }
            Opcode::Div => {
                let word = u8arr_to_i24(Processor::load_word(&bits, addr, &mut self.machine));
                self.machine.registers.set_a(self.machine.registers.get_a() / word);
            }
            Opcode::And => {
                let word = u8arr_to_i24(Processor::load_word(&bits, addr, &mut self.machine));
                self.machine.registers.set_a(self.machine.registers.get_a() & word);
            }
            Opcode::Or => {
                let word = u8arr_to_i24(Processor::load_word(&bits, addr, &mut self.machine));
                self.machine.registers.set_a(self.machine.registers.get_a() | word);
            }
            Opcode::Comp => {
                let word = u8arr_to_i24(Processor::load_word(&bits, addr, &mut self.machine));
                self.machine.registers.set_sw(match self.machine.registers.get_a().cmp(&word) {
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                });
            }
            Opcode::Tix => {
                self.machine.registers.set_x(self.machine.registers.get_x() + 1);
                let word = u8arr_to_i24(Processor::load_word(&bits, addr, &mut self.machine));
                self.machine.registers.set_sw(match self.machine.registers.get_x().cmp(&word) {
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                });
            }

            // input/output
            Opcode::Rd => {
                let current_bytes = self.machine.registers.get_a_as_bytes();
                let address = resolve_address(&bits, addr, &mut self.machine);
                let new_bytes: [u8; 3] =
                    [current_bytes[0], current_bytes[1], self.machine.get_device(address).read()];
                self.machine.registers.set_a_as_bytes(new_bytes);
            }
            Opcode::Wd => {
                let address = resolve_address(&bits, addr, &mut self.machine);
                let val_a = self.machine.registers.get_a_as_bytes()[2];
                self.machine.get_device(address).write(val_a);
            }
            Opcode::Td => Processor::not_implemented("TD"),

            // floating point arithmetic
            Opcode::Addf => Processor::not_implemented("ADDF"),
            Opcode::Subf => Processor::not_implemented("SUBF"),
            Opcode::Mulf => Processor::not_implemented("MULF"),
            Opcode::Divf => Processor::not_implemented("DIVF"),
            Opcode::Compf => Processor::not_implemented("COMPF"),

            // others
            Opcode::Lps => Processor::not_implemented("LPS"),
            Opcode::Sti => Processor::not_implemented("STI"),
            Opcode::Ssk => Processor::not_implemented("SSK"),
            _ => return false,
        };

        true
    }

    // helpers
    fn store_word(
        bits: &FormatSicF3F4Bits,
        mut address: usize,
        word: [u8; 3],
        machine: &mut Machine,
    ) -> () {
        address = resolve_address(bits, address, machine);
        println!(
            "DOING STORE WORD at {} with word [{}, {}, {}]",
            address, word[0], word[1], word[2]
        );
        machine.memory.set_word(address, word);
    }

    fn store_byte(
        bits: &FormatSicF3F4Bits,
        mut address: usize,
        byte: u8,
        machine: &mut Machine,
    ) -> () {
        address = resolve_address(bits, address, machine);
        println!("DOING STORE BYTE at {} with word {}", address, byte);
        machine.memory.set_byte(address, byte);
    }

    fn load_word(bits: &FormatSicF3F4Bits, mut address: usize, machine: &mut Machine) -> [u8; 3] {
        if is_immediate(bits) {
            return i24_to_u8arr(address as i32);
        }

        println!("OG address={}", address);
        address = resolve_address(bits, address, machine);
        println!("resolved address={}", address);
        let word = machine.memory.get_word(address);
        println!("word={:2x},{:2x},{:2x}", word[0], word[1], word[2]);
        word
    }

    fn load_byte(bits: &FormatSicF3F4Bits, mut address: usize, machine: &mut Machine) -> u8 {
        if is_immediate(bits) {
            return (address & 0xFF) as u8;
        }

        address = resolve_address(bits, address, machine);
        machine.memory.get_byte(address)
    }

    // errors
    fn not_implemented(mnemonic: &str) -> () {
        panic!("{mnemonic}: NOT IMPLMENTED!");
    }
    fn invalid_opcode(invalid_opcode_byte: u8) -> () {
        panic!("{invalid_opcode_byte}: NOT VALID OPCODE!");
    }
}

// ProcessorHandle
// ================================================================================================

pub trait ProcessorExt {
    fn start(&self);
    fn stop(&self);
    fn is_running(&self) -> bool;

    fn get_speed(&self) -> i64;
    fn set_speed(&self, hz: i64);
}

impl ProcessorExt for ProcessorHandle {
    fn start(&self) -> () {
        let mut self_ = self.lock().unwrap();

        let interval = chrono::TimeDelta::nanoseconds(MAX_HZ / self_.speed);

        let guard = {
            // create new Arc smart pointer to be used by the timer thread
            let ptr: Arc<Mutex<Processor>> = Arc::clone(&self);
            self_.timer.schedule_repeating(interval, move || {
                let mut self__ = ptr.lock().unwrap();
                self__.execute_instruction();
            })
        };

        self_.guard = Some(guard);
    }
    fn stop(&self) -> () { self.lock().unwrap().guard = None; }
    fn is_running(&self) -> bool { self.lock().unwrap().guard.is_some() }

    fn get_speed(&self) -> i64 { self.lock().unwrap().speed }
    fn set_speed(&self, hz: i64) -> () { self.lock().unwrap().speed = hz.max(1).min(MAX_HZ); }
}

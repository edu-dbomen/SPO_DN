extern crate chrono;
extern crate timer;

use std::sync::{Arc, Mutex};

use crate::{
    machine::{opcodes::Opcode, Machine},
    sic_xe::{get_FormatSicF3F4Bits, get_r1_r2},
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
        let opcode = match Opcode::from_byte(byte) {
            Some(opcode) => opcode,
            None => {
                Processor::invalid_opcode(byte);
                return;
            }
        };

        if self.exec_f1(&opcode) {
            return;
        }

        let operand = self.fetch();
        if self.exec_f2(&opcode, &operand) {
            return;
        }

        let third_byte = self.fetch();
        if !self.exec_sic_f3_f4(&opcode, &byte, &operand, &third_byte) {
            panic!("??? exec failed. Looks like I forgot to add a opcode to arms!?");
        }
    }

    /// fetch next 8b and pc++
    fn fetch(&mut self) -> u8 {
        let pc = self.machine.registers.get_pc();
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
                self.machine.registers.set_f(self.machine.registers.get_a().try_into().unwrap())
            }
            Opcode::Fix => todo!("FIX"),
            Opcode::Norm => todo!("NORM"),
            Opcode::Sio => todo!("SIO"),
            Opcode::Hio => todo!("HIO"),
            Opcode::Tio => todo!("TIO"),
            _ => return false,
        };

        true
    }
    /// opcode: 8b
    /// operand: 4b,4b == r1,r2
    /// return:
    /// \   true -> executed F2
    /// \   false -> not F2
    fn exec_f2(&self, opcode: &Opcode, operand: &u8) -> bool {
        let (r1, r2) = get_r1_r2(&operand);

        match opcode {
            Opcode::Addr => todo!("ADDR"),
            Opcode::Subr => todo!("SUBR"),
            Opcode::Mulr => todo!("MULR"),
            Opcode::Divr => todo!("DIVR"),
            Opcode::Compr => todo!("COMPR"),
            Opcode::Shiftl => todo!("SHIFTL"),
            Opcode::Shiftr => todo!("SHIFTR"),
            Opcode::Rmo => todo!("RMO"),
            Opcode::Clear => todo!("CLEAR"),
            Opcode::Tixr => todo!("TIXR"),
            Opcode::Svc => todo!("SVC"),
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
    /// return:
    /// \   true -> executed F3
    /// \   false -> not F3
    fn exec_sic_f3_f4(
        &self,
        opcode: &Opcode,
        first_byte: &u8,
        second_byte: &u8,
        third_byte: &u8,
    ) -> bool {
        let bits = get_FormatSicF3F4Bits(&first_byte, &second_byte);

        match opcode {
            // ***** immediate addressing not possible *****
            // stores
            Opcode::Sta => 1 + 1, //todo!("STA"),
            Opcode::Stx => todo!("STX"),
            Opcode::Stl => todo!("STL"),
            Opcode::Stch => todo!("STCH"),
            Opcode::Stb => todo!("STB"),
            Opcode::Sts => todo!("STS"),
            Opcode::Stf => todo!("STF"),
            Opcode::Stt => todo!("STT"),
            Opcode::Stsw => todo!("STSW"),

            // jumps
            Opcode::Jeq => todo!("JEQ"),
            Opcode::Jgt => todo!("JGT"),
            Opcode::Jlt => todo!("JLT"),
            Opcode::J => todo!("J"),
            Opcode::Rsub => todo!("RSUB"),
            Opcode::Jsub => todo!("JSUB"),

            // ***** immediate addressing possible *****
            // loads
            Opcode::Lda => todo!("LDA"),
            Opcode::Ldx => todo!("LDX"),
            Opcode::Ldl => todo!("LDL"),
            Opcode::Ldch => todo!("LDCH"),
            Opcode::Ldb => todo!("LDB"),
            Opcode::Lds => todo!("LDS"),
            Opcode::Ldf => todo!("LDF"),
            Opcode::Ldt => todo!("LDT"),

            // arithmetic
            Opcode::Add => todo!("ADD"),
            Opcode::Sub => todo!("SUB"),
            Opcode::Mul => todo!("MUL"),
            Opcode::Div => todo!("DIV"),
            Opcode::And => todo!("AND"),
            Opcode::Or => todo!("OR"),
            Opcode::Comp => todo!("COMP"),
            Opcode::Tix => todo!("TIX"),

            // input/output
            Opcode::Rd => todo!("RD"),
            Opcode::Wd => todo!("WD"),
            Opcode::Td => todo!("TD"),

            // floating point arithmetic
            Opcode::Addf => todo!("ADDF"),
            Opcode::Subf => todo!("SUBF"),
            Opcode::Mulf => todo!("MULF"),
            Opcode::Divf => todo!("DIVF"),
            Opcode::Compf => todo!("COMPF"),

            // others
            Opcode::Lps => todo!("LPS"),
            Opcode::Sti => todo!("STI"),
            Opcode::Ssk => todo!("SSK"),
            _ => return false,
        };

        true
    }

    // errors
    // TODO:
    fn not_implemented(mnemonic: String) -> () {}
    fn invalid_opcode(invalid_opcode_byte: u8) -> () {}
    fn invalid_addressing() -> () {}
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

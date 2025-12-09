mod machine;
mod processor;
mod sic_xe;

use std::{
    fs::OpenOptions,
    io::{self, BufRead},
};

use machine::Machine;
use processor::Processor;

use crate::processor::ProcessorExt;

fn main() {
    //test_machine();
    test_processor();
}

fn test_processor() {
    let processor_ptr = Processor::new_handle();

    // setup
    {
        let mut processor = processor_ptr.lock().unwrap();

        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            // .open("./tests/arith.obj")
            .open("./tests/horner.obj")
            .expect("Could not open file");

        // load program at ./prog.obj
        let mut current_load_address: usize;
        let mut execution_address: i32 = 0;
        for line in io::BufReader::new(file).lines() {
            let line = line.unwrap();
            match line.chars().nth(0).unwrap() {
                'T' => {
                    // set load address
                    current_load_address =
                        usize::from_str_radix(line.get(1..7).unwrap(), 16).unwrap();

                    // write bytes into memory
                    let number_of_bytes = i32::from_str_radix(line.get(7..9).unwrap(), 16).unwrap();
                    for i in 0..number_of_bytes {
                        let low_ix: usize = (9 + 2 * i) as usize;
                        let high_ix: usize = (9 + 2 * i + 2) as usize;
                        let val: u8 =
                            u8::from_str_radix(line.get(low_ix..high_ix).unwrap(), 16).unwrap();

                        processor.machine.memory.set_byte(current_load_address, val);
                        current_load_address += 1;
                    }
                }
                'E' => {
                    // set execution address
                    execution_address = i32::from_str_radix(line.get(1..7).unwrap(), 16).unwrap();
                }
                _ => {}
            }
        }

        // execute program
        processor.machine.registers.set_pc(execution_address);
    }

    processor_ptr.start();
    loop {}
}

fn test_machine() {
    // write HELLO: to output
    let mut machine = Machine::new();
    machine.get_device(1).write(0x48);
    machine.get_device(1).write(0x45);
    machine.get_device(1).write(0x4C);
    machine.get_device(1).write(0x4C);
    machine.get_device(1).write(0x4F);
    machine.get_device(1).write(0x3A);
    machine.get_device(1).write(0x0A);

    // get input
    let _ = machine.get_device(0).read();

    // write HI to output
    machine.memory.set_byte(0xabcd, 0x48);
    machine.memory.set_byte(0xabce, 0x49);
    machine.memory.set_byte(0xabcf, 0x0A);
    let word = machine.memory.get_word(0xabcd);
    for byte in word {
        machine.get_device(1).write(byte.clone());
    }

    // change register A values and write to output
    machine.get_device(1).write(0x41);
    machine.get_device(1).write(0x3A);
    machine.get_device(1).write(0x20);
    let val_a = machine.registers.get_a() as u8;
    machine.get_device(1).write(val_a);
    machine.get_device(1).write(0x0A);

    machine.registers.set_a(69);
    machine.get_device(1).write(0x41);
    machine.get_device(1).write(0x3A);
    machine.get_device(1).write(0x20);
    let val_a = machine.registers.get_a() as u8;
    machine.get_device(1).write(val_a);
    machine.get_device(1).write(0x0A);
}

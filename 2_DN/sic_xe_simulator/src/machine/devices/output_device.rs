use crate::machine::devices::device::Device;
use std::io::{self, Write};

pub struct OutputDevice {}

impl Device for OutputDevice {
    fn test(&self) -> bool { true }

    fn read(&mut self) -> i8 { 0 }

    fn write(&mut self, val: i8) -> () {
        let _ = io::stdout().write_all(&[val as u8]).expect("Stdout error");
    }
}

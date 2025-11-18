use crate::machine::devices::device::Device;
use std::io::{self, Read};

pub struct InputDevice {}

impl Device for InputDevice {
    fn test(&self) -> bool { true }

    fn read(&mut self) -> u8 {
        let mut buf = [0];
        io::stdin().read_exact(&mut buf).expect("Stdin error");
        buf[0]
    }

    fn write(&mut self, _val: u8) -> () {}
}

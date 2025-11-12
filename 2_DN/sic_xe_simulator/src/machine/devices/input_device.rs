use crate::machine::devices::device::Device;
use std::io::{self, Read};

pub struct InputDevice {}

impl Device for InputDevice {
    fn test(&self) -> bool { true }

    fn read(&mut self) -> i8 {
        let mut buf = [0];
        io::stdin().read_exact(&mut buf).expect("Stdin error");
        buf[0] as i8
    }

    fn write(&mut self, _val: i8) -> () {}
}

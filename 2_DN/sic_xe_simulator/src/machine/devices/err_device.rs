use crate::machine::devices::device::Device;
use std::{
    any::Any,
    io::{self, Write},
};

pub struct ErrDevice {}

impl Device for ErrDevice {
    fn as_any(&self) -> &dyn Any { self }

    fn test(&self) -> bool { true }

    fn read(&mut self) -> u8 { 0 }

    fn write(&mut self, val: u8) -> () { let _ = io::stderr().write_all(&[val]).unwrap(); }
}

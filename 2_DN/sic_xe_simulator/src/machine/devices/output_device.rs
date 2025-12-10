use crate::machine::devices::device::Device;
use std::{any::Any, io::{self, Write}};

pub struct OutputDevice {
    pub write_buffer: String,
}

impl Device for OutputDevice {
    fn as_any(&self) -> &dyn Any { self }

    fn test(&self) -> bool { true }

    fn read(&mut self) -> u8 { 0 }

    fn write(&mut self, val: u8) -> () {
        // let _ = io::stdout().write_all(&[val]).expect("Stdout error");
        // NOTE: use the write_buffer if using the ratatui ui, if not use the normal printing to stdout
        self.write_buffer.push(val as char);
    }
}

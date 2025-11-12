mod memory;
mod registers;
mod devices;
pub mod opcodes;

use memory::Memory;
use registers::Registers;
use devices::device::Device;
use devices::input_device::InputDevice;
use devices::output_device::OutputDevice;
use devices::err_device::ErrDevice;
use devices::file_device::FileDevice;

const MAX_DEVICES: usize = 256;

pub struct Machine {
    pub registers: Registers,
    pub memory: Memory,
    /// accessable from get_device and set_device
    devices: Vec<Box<dyn Device>>
}

impl Machine {
    #[rustfmt::skip]
    pub fn new() -> Self { 
        Self {
            registers: Registers::new(), 
            memory: Memory::new(),
            devices: Machine::device_init(),
        }
    }

    fn device_init() -> Vec<Box<dyn Device>> {
        let mut vec: Vec<Box<dyn Device>> = Vec::with_capacity(MAX_DEVICES);
        vec.push(Box::new(InputDevice {}));
        vec.push(Box::new(OutputDevice {}));
        vec.push(Box::new(ErrDevice {}));
        for i in 3..MAX_DEVICES {
            let hex_string = format!("{:X}", i);
            vec.push(Box::new(FileDevice::new(hex_string + ".dev")));
        }
        vec
    }
    pub fn get_device(&mut self, index: usize) -> &mut Box<dyn Device> {
        &mut self.devices[index]
    }
    pub fn set_device(&mut self, index: usize, device: Box<dyn Device>) -> () {
        self.devices[index] = device;
    }
}

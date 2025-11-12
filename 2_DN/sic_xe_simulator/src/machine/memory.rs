const MAX_ADDRESS: usize = 1 << 20 - 1;
/// 1MB == 2^20B
const SIZE: usize = MAX_ADDRESS + 1;

/// Size: 1MB == 2^20B
pub struct Memory {
    memory: Vec<i8>,
}

impl Memory {
    pub fn new() -> Self { Self { memory: vec![0; SIZE] } }

    pub fn get_byte(&self, address: usize) -> i8 { self.memory[address] }
    pub fn set_byte(&mut self, address: usize, val: i8) -> () { self.memory[address] = val; }

    pub fn get_word(&self, address: usize) -> [i8; 3] {
        self.memory[address..address + 3].try_into().unwrap()
    }
    pub fn set_word(&mut self, address: usize, val: [i8; 3]) -> () {
        self.memory[address..address + 3].copy_from_slice(&val);
    }

    pub fn get_float(&self, address: usize) -> [i8; 6] {
        self.memory[address..address + 6].try_into().unwrap()
    }
    pub fn set_float(&mut self, address: usize, val: [i8; 6]) -> () {
        self.memory[address..address + 6].copy_from_slice(&val);
    }
}

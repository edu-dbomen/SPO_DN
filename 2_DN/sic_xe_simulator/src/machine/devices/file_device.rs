use crate::machine::devices::device::Device;
use std::{
    any::Any,
    fs::{File, OpenOptions},
    io::{ErrorKind, Read, Write},
};

pub struct FileDevice {
    file_name: String,
    file: Option<File>,
}

impl FileDevice {
    pub fn new(file_name: String) -> Self { Self { file_name, file: None } }
    fn open_file(&mut self) -> &mut File {
        if self.file.is_none() {
            self.file = Some(
                OpenOptions::new()
                    .write(true)
                    .read(true)
                    .create(true)
                    .open(self.file_name.clone())
                    .expect("Could not open file"),
            );
        }

        self.file.as_mut().unwrap()
    }
}

impl Device for FileDevice {
    fn as_any(&self) -> &dyn Any { self }

    fn test(&self) -> bool { true }

    fn read(&mut self) -> u8 {
        let mut buf = [0u8; 1];

        // println!("READING file: {}", self.file_name);
        match self.open_file().read_exact(&mut buf) {
            Ok(()) => buf[0],
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                // EOF: define behavior here
                0
            }
            Err(e) => panic!("File reading error: {e}"),
        }
    }

    fn write(&mut self, val: u8) -> () {
        let _ = self.open_file().write_all(&mut [val]).expect("File writing error");
    }
}

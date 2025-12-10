use std::any::Any;

pub trait Device: Send + Any {
    fn as_any(&self) -> &dyn Any;

    fn test(&self) -> bool;
    fn read(&mut self) -> u8;
    fn write(&mut self, val: u8) -> ();
}

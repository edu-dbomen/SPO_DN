pub trait Device {
    fn test(&self) -> bool;
    fn read(&mut self) -> i8;
    fn write(&mut self, val: i8) -> ();
}

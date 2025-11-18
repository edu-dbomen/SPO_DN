pub trait Device: Send {
    fn test(&self) -> bool;
    fn read(&mut self) -> u8;
    fn write(&mut self, val: u8) -> ();
}

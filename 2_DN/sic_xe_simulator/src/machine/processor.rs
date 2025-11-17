extern crate chrono;
extern crate timer;

const MAX_HZ: i64 = 1_000_000_000;

pub struct Processor {
    /// speed in Hz
    speed: i64,

    timer: timer::Timer,
    guard: Option<timer::Guard>,
}

impl Processor {
    pub fn new() -> Self { Self { speed: 100, timer: timer::Timer::new(), guard: None } }

    pub fn start(&mut self) -> () {
        self.guard = Some(self.timer.schedule_repeating(
            chrono::TimeDelta::nanoseconds(MAX_HZ / self.speed),
            || {
                // TODO:
            },
        ));
    }
    pub fn stop(&mut self) -> () { self.guard = None; }
    pub fn is_running(&self) -> bool { self.guard.is_some() }

    fn fetch() -> i8 { 0 }

    fn execute() -> () {}
    /// opcode: 8b
    fn exec_f1(opcode: i8) -> bool { true }
    /// opcode: 8b
    /// operand: 4b,4b == r1,r2
    fn exec_f2(opcode: i8, operand: i8) -> bool { true }
    /// opcode: 6b
    /// ni: 1b,1b == n,i
    /// \   0, 0 -> SIC
    /// \   else -> F3 or F4
    /// operand: 16b or 24b
    /// \   SIC  -> 1b,15b == x,addr
    /// \   F3   -> 1b,1b,1b,1b,12b == x,b,p,e,offset
    /// \   F4   -> 1b,1b,1b,1b,20b == x,b,p,e,addr
    fn exec_sic_f3_f4(opcode: i8, ni: i8, operand: Vec<i8>) -> bool { true }

    fn get_speed(&self) -> i64 { self.speed }
    fn set_speed(&mut self, hz: i64) -> () { self.speed = hz.max(1).min(MAX_HZ); }

    // errors
    fn not_implemented(mnemonic: String) -> () {}
    fn invalid_opcode(opcode: i8) -> () {}
    fn invalid_addressing() -> () {}
}

use crate::sic_xe::BIT_MASK_DWORD;
use crate::sic_xe::BIT_MASK_WORD;
use crate::sic_xe::BIT_MAX_WORD;

pub struct Registers {
    /// 24b
    a: i32,
    /// 24b
    x: i32,
    /// 24b
    l: i32,
    /// 24b
    b: i32,
    /// 24b
    s: i32,
    /// 24b
    t: i32,
    /// 48b
    f: f64,
    /// 24b
    pc: i32,
    /// 24b
    /// 0x0 .. <
    /// 0x40 .. =
    /// 0x80 .. >
    sw: i32,
}

impl Registers {
    pub fn new() -> Self { Self { a: 0, x: 0, l: 0, b: 0, s: 0, t: 0, f: 0.0, pc: 0, sw: 0 } }

    // Getters and setters
    pub fn get_a(&self) -> i32 { self.a & BIT_MASK_WORD; }
    pub fn get_x(&self) -> i32 { self.x & BIT_MASK_WORD; }
    pub fn get_l(&self) -> i32 { self.l & BIT_MASK_WORD; }
    pub fn get_b(&self) -> i32 { self.b & BIT_MASK_WORD; }
    pub fn get_s(&self) -> i32 { self.s & BIT_MASK_WORD; }
    pub fn get_t(&self) -> i32 { self.t & BIT_MASK_WORD; }
    pub fn get_f(&self) -> f64 { self.f }
    pub fn get_pc(&self) -> i32 { self.pc & BIT_MASK_WORD; }
    pub fn get_sw(&self) -> i32 { self.sw & BIT_MASK_WORD; }

    pub fn set_a(&mut self, val: i32) -> () {
        match val.cmp(BIT_MAX_WORD) {
            std::cmp::Ordering::Greater => -(!val & BIT_MASK_WORD) - 1,
            _ => val,
        }
    }
    pub fn set_x(&mut self, val: i32) -> () {
        match val.cmp(BIT_MAX_WORD) {
            std::cmp::Ordering::Greater => -(!val & BIT_MASK_WORD) - 1,
            _ => val,
        }
    }
    pub fn set_l(&mut self, val: i32) -> () {
        match val.cmp(BIT_MAX_WORD) {
            std::cmp::Ordering::Greater => -(!val & BIT_MASK_WORD) - 1,
            _ => val,
        }
    }
    pub fn set_b(&mut self, val: i32) -> () {
        match val.cmp(BIT_MAX_WORD) {
            std::cmp::Ordering::Greater => -(!val & BIT_MASK_WORD) - 1,
            _ => val,
        }
    }
    pub fn set_s(&mut self, val: i32) -> () {
        match val.cmp(BIT_MAX_WORD) {
            std::cmp::Ordering::Greater => -(!val & BIT_MASK_WORD) - 1,
            _ => val,
        }
    }
    pub fn set_t(&mut self, val: i32) -> () {
        match val.cmp(BIT_MAX_WORD) {
            std::cmp::Ordering::Greater => -(!val & BIT_MASK_WORD) - 1,
            _ => val,
        }
    }
    pub fn set_f(&mut self, val: f64) -> () { self.f = val }
    pub fn set_pc(&mut self, val: i32) -> () {
        match val.cmp(BIT_MAX_WORD) {
            std::cmp::Ordering::Greater => -(!val & BIT_MASK_WORD) - 1,
            _ => val,
        }
    }
    pub fn set_sw(&mut self, val: i32) -> () {
        match val.cmp(BIT_MAX_WORD) {
            std::cmp::Ordering::Greater => -(!val & BIT_MASK_WORD) - 1,
            _ => val,
        }
    }

    /// **UNSTABLE** TODO: Fails on index 6 for now
    /// Get register by index
    /// A .. 0
    /// X .. 1
    /// L .. 2
    /// B .. 3
    /// S .. 4
    /// T .. 5
    /// F .. 6
    /// PC .. 8
    /// SW .. 9
    pub fn get_reg(&self, index: usize) -> i32 {
        match index {
            0 => self.a,
            1 => self.x,
            2 => self.l,
            3 => self.b,
            4 => self.s,
            5 => self.t,
            //6 => self.f,
            8 => self.pc,
            9 => self.sw,
            _ => panic!("INVALID INDEX!"),
        }
    }
    /// **UNSTABLE** TODO: Fails on index 6 for now
    /// Se register by index
    /// A .. 0
    /// X .. 1
    /// L .. 2
    /// B .. 3
    /// S .. 4
    /// T .. 5
    /// F .. 6
    /// PC .. 8
    /// SW .. 9
    pub fn set_reg(&mut self, index: usize, val: i32) -> () {
        match index {
            0 => self.a = val,
            1 => self.x = val,
            2 => self.l = val,
            3 => self.b = val,
            4 => self.s = val,
            5 => self.t = val,
            //6 => self.f = val,
            8 => self.pc = val,
            9 => self.sw = val,
            _ => panic!("INVALID INDEX!"),
        }
    }
}

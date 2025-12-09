use std::fmt;

use crate::machine::Machine;

pub const MASK_WORD: i32 = 0xFFFFFF;
pub const MASK_FIRST_BYTE: i32 = 0xFF0000;
pub const MASK_SECOND_BYTE: i32 = 0x00FF00;
pub const MASK_THIRD_BYTE: i32 = 0x0000FF;
pub const SIGN_BIT: i32 = 0x800000;

// **********************************************
//  CONVERSION helpers
// **********************************************

/// converts i32 to i24 (word)
pub fn i32_to_i24(val: i32) -> i32 {
    let masked = val & MASK_WORD;
    if masked & SIGN_BIT != 0 {
        masked | !MASK_WORD
    } else {
        masked
    }
}

/// converts i24 to [u8;3]
#[rustfmt::skip]
pub fn i24_to_u8arr(val: i32) -> [u8; 3] {
    let v = val & MASK_WORD;
    [ 
        ((v >> 16) & 0xFF) as u8,
        ((v >> 8) & 0xFF) as u8,
        (v & 0xFF) as u8,
    ]
}

/// converts [u8;3] to i24
pub fn u8arr_to_i24(val: [u8; 3]) -> i32 {
    let v: i32 = ((val[0] as i32) << 16) | ((val[1] as i32) << 8) | (val[2] as i32);
    println!("v={:x}", v);

    if v & 0x0080_0000 != 0 {
        v | 0xFF00_0000u32 as i32
    } else {
        v
    }
}

pub fn get_r1_r2(val: &u8) -> (u8, u8) {
    let r1 = (val & 0xF0) >> 4;
    let r2 = val & 0x0F;
    (r1, r2)
}

// **********************************************
//  BITS helpers
// **********************************************

pub struct FormatSicF3F4Bits {
    n: bool,
    i: bool,
    x: bool,
    b: bool,
    p: bool,
    e: bool,
}

impl fmt::Display for FormatSicF3F4Bits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[n={},i={},x={},b={},p={},e={}]", self.n, self.i, self.x, self.b, self.p, self.e)
    }
}

pub fn get_format_sic_f3_f4_bits(opcode: &u8, first_byte: &u8) -> FormatSicF3F4Bits {
    println!("got bytes: {:08b} {:08b}", opcode, first_byte);
    FormatSicF3F4Bits {
        n: (opcode & 0b0000_0010) != 0,
        i: (opcode & 0b0000_0001) != 0,
        x: (first_byte & 0b1000_0000) != 0,
        b: (first_byte & 0b0100_0000) != 0,
        p: (first_byte & 0b0010_0000) != 0,
        e: (first_byte & 0b0001_0000) != 0,
    }
}
pub fn is_format_sic(bits: &FormatSicF3F4Bits) -> bool {
    return bits.n == false && bits.i == false;
}
pub fn is_format_f3(bits: &FormatSicF3F4Bits) -> bool { return bits.e == false }
pub fn is_format_f4(bits: &FormatSicF3F4Bits) -> bool { return bits.e == true }
pub fn is_immediate(bits: &FormatSicF3F4Bits) -> bool { return bits.i && !bits.n }
pub fn is_indirect(bits: &FormatSicF3F4Bits) -> bool { return !bits.i && bits.n }

pub fn is_pc_relative(bits: &FormatSicF3F4Bits) -> bool { return bits.p && !bits.b }
pub fn is_base_relative(bits: &FormatSicF3F4Bits) -> bool { return !bits.p && bits.b }

pub fn is_x(bits: &FormatSicF3F4Bits) -> bool { return bits.x }

pub fn resolve_address(bits: &FormatSicF3F4Bits, mut address: usize, machine: &Machine) -> usize {
    println!("resolving address={}", address);
    if is_pc_relative(bits) {
        // note that address is a signed number for pc relative
        let mut saddress = address as i64;
        if is_format_f3(bits) {
            // we look at bottom 12b
            if address & 0x800 != 0 {
                // make negative number
                saddress = !saddress;
                saddress = saddress & 0x8FF;

                println!("pc relative (negative): addr={}=={:08b}", saddress, saddress);
                saddress = !saddress;
                println!("pc relative (negative): bits inverted={}=={:08b}", saddress, saddress);
                println!(
                    "pc relative (negative): after={} = pc={} - oldaddr={}",
                    (machine.registers.get_pc()) as i64 - saddress,
                    machine.registers.get_pc() as i64,
                    saddress
                );
            }
            saddress = (machine.registers.get_pc()) as i64 + saddress;
        } else if is_format_f4(bits) {
        } else {
            panic!("Wrong format!");
        }

        address = saddress as usize;
    }
    if is_base_relative(bits) {
        address += (machine.registers.get_b()) as usize;
    }

    if is_indirect(bits) {
        address = u8arr_to_i24(machine.memory.get_word(address)) as usize;
    }

    if is_x(bits) {
        address += machine.registers.get_x() as usize;
    }

    println!("resolved address={}", address);
    address
}

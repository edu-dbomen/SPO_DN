pub const MASK_WORD: i32 = 0xFFFFFF;
pub const SIGN_BIT: i32 = 0x800000;

/// converts i32 to i24 (word)
pub fn to_i24(val: i32) -> i32 {
    let masked = val & MASK_WORD;
    if masked & SIGN_BIT != 0 {
        masked | !MASK_WORD
    } else {
        masked
    }
}

pub fn get_r1_r2(val: &u8) -> (u8, u8) {
    let r1 = (val & 0xF0) >> 4;
    let r2 = val & 0x0F;
    (r1, r2)
}

pub struct FormatSicF3F4Bits {
    n: bool,
    i: bool,
    x: bool,
    b: bool,
    p: bool,
    e: bool,
}
pub fn get_FormatSicF3F4Bits(opcode: &u8, first_byte: &u8) -> FormatSicF3F4Bits {
    FormatSicF3F4Bits {
        n: opcode & 0b0000_0010 == 1,
        i: opcode & 0b0000_0001 == 1,
        x: first_byte & 0b1000_0000 == 1,
        b: first_byte & 0b0100_0000 == 1,
        p: first_byte & 0b0010_0000 == 1,
        e: first_byte & 0b0001_0000 == 1,
    }
}
pub fn isFormatSic(bits: &FormatSicF3F4Bits) -> bool { return bits.n == false && bits.i == false; }
pub fn isFormatF3(bits: &FormatSicF3F4Bits) -> bool { return bits.e == false }
pub fn isFormatF4(bits: &FormatSicF3F4Bits) -> bool { return bits.e == true }

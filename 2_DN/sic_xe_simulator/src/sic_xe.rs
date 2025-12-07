pub const MASK_WORD: i32 = 0xFFFFFF;
pub const MASK_FIRST_BYTE: i32 = 0xFF0000;
pub const MASK_SECOND_BYTE: i32 = 0x00FF00;
pub const MASK_THIRD_BYTE: i32 = 0x0000FF;
pub const SIGN_BIT: i32 = 0x800000;

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
    let mut v: i32 = 0;
    v = ((val[0] as i32) << 16) | ((val[1] as i32) << 8) | (val[2] as i32);

    if v & 0x0080_0000 != 0 {
        v | 0xFF00_0000
    } else {
        v
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
pub fn get_format_sic_f3_f4_bits(opcode: &u8, first_byte: &u8) -> FormatSicF3F4Bits {
    FormatSicF3F4Bits {
        n: opcode & 0b0000_0010 == 1,
        i: opcode & 0b0000_0001 == 1,
        x: first_byte & 0b1000_0000 == 1,
        b: first_byte & 0b0100_0000 == 1,
        p: first_byte & 0b0010_0000 == 1,
        e: first_byte & 0b0001_0000 == 1,
    }
}
pub fn is_format_sic(bits: &FormatSicF3F4Bits) -> bool {
    return bits.n == false && bits.i == false;
}
pub fn is_format_f3(bits: &FormatSicF3F4Bits) -> bool { return bits.e == false }
pub fn is_format_f4(bits: &FormatSicF3F4Bits) -> bool { return bits.e == true }
pub fn is_immediate(bits: &FormatSicF3F4Bits) -> bool { return bits.i }
pub fn is_indirect(bits: &FormatSicF3F4Bits) -> bool { return bits.n }

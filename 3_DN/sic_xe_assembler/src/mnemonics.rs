use std::str::FromStr;

pub enum Mnemonic {
    Opcode(Opcode),
    Directive(Directive),
}
impl Mnemonic {
    pub fn parse(s: &str) -> Option<Self> {
        if let Ok(op) = s.parse::<Opcode>() {
            Some(Mnemonic::Opcode(op))
        } else if let Ok(d) = s.parse::<Directive>() {
            Some(Mnemonic::Directive(d))
        } else {
            None
        }
    }
}

// ************************************************************************************************

pub enum Opcode {
    // ***** SIC format, SIC/XE Format 3 and 4

    // load and store
    Lda = 0x00,
    Ldx = 0x04,
    Ldl = 0x08,
    Sta = 0x0C,
    Stx = 0x10,
    Stl = 0x14,

    // fixed point arithmetic
    Add = 0x18,
    Sub = 0x1C,
    Mul = 0x20,
    Div = 0x24,
    Comp = 0x28,
    Tix = 0x2C,

    // jumps
    Jeq = 0x30,
    Jgt = 0x34,
    Jlt = 0x38,
    J = 0x3C,

    // bit manipulation
    And = 0x40,
    Or = 0x44,

    // jump to subroutine
    Jsub = 0x48,
    Rsub = 0x4C,

    // load and store byte/char
    Ldch = 0x50,
    Stch = 0x54,

    // floating point arithmetic
    Addf = 0x58,
    Subf = 0x5C,
    Mulf = 0x60,
    Divf = 0x64,
    Compf = 0x88,

    // load and store (more regs)
    Ldb = 0x68,
    Lds = 0x6C,
    Ldf = 0x70,
    Ldt = 0x74,
    Stb = 0x78,
    Sts = 0x7C,
    Stf = 0x80,
    Stt = 0x84,

    // special load/store
    Lps = 0xD0,
    Sti = 0xD4,
    Stsw = 0xE8,

    // devices
    Rd = 0xD8,
    Wd = 0xDC,
    Td = 0xE0,

    // system
    Ssk = 0xEC,

    // ***** SIC/XE Format 1
    Float = 0xC0,
    Fix = 0xC4,
    Norm = 0xC8,
    Sio = 0xF0,
    Hio = 0xF4,
    Tio = 0xF8,

    // ***** SIC/XE Format 2
    Addr = 0x90,
    Subr = 0x94,
    Mulr = 0x98,
    Divr = 0x9C,
    Compr = 0xA0,
    Shiftl = 0xA4,
    Shiftr = 0xA8,
    Rmo = 0xAC,
    Svc = 0xB0,
    Clear = 0xB4,
    Tixr = 0xB8,
}
impl FromStr for Opcode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Opcode::*;

        match s.to_ascii_uppercase().as_str() {
            "LDA" => Ok(Lda),
            "LDX" => Ok(Ldx),
            "LDL" => Ok(Ldl),
            "STA" => Ok(Sta),
            "STX" => Ok(Stx),
            "STL" => Ok(Stl),

            "ADD" => Ok(Add),
            "SUB" => Ok(Sub),
            "MUL" => Ok(Mul),
            "DIV" => Ok(Div),
            "COMP" => Ok(Comp),
            "TIX" => Ok(Tix),

            "JEQ" => Ok(Jeq),
            "JGT" => Ok(Jgt),
            "JLT" => Ok(Jlt),
            "J" => Ok(J),

            "AND" => Ok(And),
            "OR" => Ok(Or),

            "JSUB" => Ok(Jsub),
            "RSUB" => Ok(Rsub),

            "LDCH" => Ok(Ldch),
            "STCH" => Ok(Stch),

            "ADDF" => Ok(Addf),
            "SUBF" => Ok(Subf),
            "MULF" => Ok(Mulf),
            "DIVF" => Ok(Divf),
            "COMPF" => Ok(Compf),

            "LDB" => Ok(Ldb),
            "LDS" => Ok(Lds),
            "LDF" => Ok(Ldf),
            "LDT" => Ok(Ldt),
            "STB" => Ok(Stb),
            "STS" => Ok(Sts),
            "STF" => Ok(Stf),
            "STT" => Ok(Stt),

            "LPS" => Ok(Lps),
            "STI" => Ok(Sti),
            "STSW" => Ok(Stsw),

            "RD" => Ok(Rd),
            "WD" => Ok(Wd),
            "TD" => Ok(Td),

            "SSK" => Ok(Ssk),

            "FLOAT" => Ok(Float),
            "FIX" => Ok(Fix),
            "NORM" => Ok(Norm),
            "SIO" => Ok(Sio),
            "HIO" => Ok(Hio),
            "TIO" => Ok(Tio),

            "ADDR" => Ok(Addr),
            "SUBR" => Ok(Subr),
            "MULR" => Ok(Mulr),
            "DIVR" => Ok(Divr),
            "COMPR" => Ok(Compr),
            "SHIFTL" => Ok(Shiftl),
            "SHIFTR" => Ok(Shiftr),
            "RMO" => Ok(Rmo),
            "SVC" => Ok(Svc),
            "CLEAR" => Ok(Clear),
            "TIXR" => Ok(Tixr),

            _ => Err(()),
        }
    }
}

pub enum Format {
    F1,
    F2,
    F3_4, // SIC + SIC/XE format 3/4
}
impl Opcode {
    pub fn format(&self) -> Format {
        use Format::*;
        use Opcode::*;

        match self {
            // Format 1
            Float | Fix | Norm | Sio | Hio | Tio => F1,

            // Format 2
            Addr | Subr | Mulr | Divr | Compr | Shiftl | Shiftr | Rmo | Svc | Clear | Tixr => F2,

            // Everything else you listed is SIC / format 3/4
            _ => F3_4,
        }
    }
}

// ************************************************************************************************

#[derive(PartialEq, Eq)]
pub enum Directive {
    Start,
    End,
    Org,
    Equ, // simple version, for constants only
    Base,
    Nobase,
    Resb,
    Resw,
    Byte,
    Word,
    If,
    Mod,
}
impl FromStr for Directive {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Directive::*;

        match s.to_ascii_uppercase().as_str() {
            "START" => Ok(Start),
            "END" => Ok(End),
            "ORG" => Ok(Org),
            "EQU" => Ok(Equ),
            "BASE" => Ok(Base),
            "NOBASE" => Ok(Nobase),
            "RESB" => Ok(Resb),
            "RESW" => Ok(Resw),
            "BYTE" => Ok(Byte),
            "WORD" => Ok(Word),
            "IF" => Ok(If),
            "MOD" => Ok(Mod),

            _ => Err(()),
        }
    }
}

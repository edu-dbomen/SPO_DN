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

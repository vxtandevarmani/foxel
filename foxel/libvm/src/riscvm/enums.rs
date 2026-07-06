
/*  [ Every link I used in order to help me build this ]
 *          + (A tad bit of googling)
 *
 *  https://msyksphinz-self.github.io/riscv-isadoc/html/regs.html
 *  https://msyksphinz-self.github.io/riscv-isadoc/
 *  https://passlab.github.io/CSCE513/notes/lecture04_RISCV_ISA.pdf
 * 
 */


#[derive(Debug)]
pub enum Registers {
    Zero,
    Ra,Sp,Gp,Tp,
    T0,T1,T2,
    S0FP,
    S1,
    A0,A1,A2,A3,A4,A5,A6,A7,
    S2,S3,S4,S5,S6,S7,S8,S9,S10,S11,
    T3,T4,T5,T6
}

#[derive(Debug)]
pub enum Rv64IM {
    Lui     = 0b0110111,
    Auipc   = 0b0010111,
    Jal     = 0b1101111,
    Jalr    = 0b1100111,
    Branch  = 0b1100011,
    Load    = 0b0000011,
    Store   = 0b0100011,
    ImmOp64 = 0b0010011,
    RgOp64  = 0b0110011,
    Fence   = 0b0001111,
    Syscall = 0b1110011,
    ImmOp32 = 0b0011011,
    RegOp32 = 0b0111011,
    Invalid = 0xff,
}

/*
    --> also bottom 2 bits are always 1s so you can
        define a handler table 32 elements long and
        add a (>> 2) in every fetch :p

*/ 

pub enum Branch {
    BEQ     = 0b000,
    BNE     = 0b001,
    BLT     = 0b100,
    BGE     = 0b101,
    BLTU    = 0b110,
    BGEU    = 0b111,
    BINV    = 0xff,
}

pub enum Load {
    LB      = 0b000,
    LH      = 0b001,
    LW      = 0b010,
    LBU     = 0b100,
    LHU     = 0b101,
    LWU     = 0b110,
    LD      = 0b011,
    LINV    = 0xff
}

pub enum Store {
    SB      = 0b000,
    SH      = 0b001,
    SW      = 0b010,
    SD      = 0b011,
    SINV    = 0xff,
}

pub enum ImmOp64 {
    ADDI    = 0b000,
    SLTI    = 0b010,
    SLTIU   = 0b011,
    XORI    = 0b100,
    ORI     = 0b110,
    ANDI    = 0b111,
    SLLI    = 0b001,
    SRXI    = 0b101,
    INVI    = 0xff,
}

pub enum RgOp64 {
    ADDSUB  = 0b000,
    SLL     = 0b001,
    SLT     = 0b010,
    SLTU    = 0b011,
    XOR     = 0b100,
    SRX     = 0b101,
    OR      = 0b110,
    AND     = 0b111,    
    INV     = 0xff,  
}

pub enum ImmOp32 {
    ADDIW   = 0b000,
    SLLIW   = 0b001,
    SRXIW   = 0b101,
    INVIW   = 0xff,
}

pub enum RgOp32 {
    ADDSUBW = 0b000,
    SLLW    = 0b001,
    SRXW    = 0b101,
    INVW    = 0xff,
}

pub enum Syscall {
    Ecall   = 0b000000000000,
    EBreak  = 0b000000000001,
    INVALID = 0xff,
}

pub enum M32 {
    MUL     = 0b000,
    MULH    = 0b001,
    MULHSU  = 0b010,
    MULHU   = 0b011,
    DIV     = 0b100,
    DIVU    = 0b101,
    REM     = 0b110,
    REMU    = 0b111,
    INV     = 0xff,
}

pub enum M64 {
    MULW    = 0b000,
    DIVW    = 0b100,
    DIVUW   = 0b101,
    REMW    = 0b110,
    REMUW   = 0b111,
    INV     = 0xff,
}

pub enum ServiceNumber {
    HOST    = 0xff,
    GPEB    = 0x45,
    INV     = 0x00,
}
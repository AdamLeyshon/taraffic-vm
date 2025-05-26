use crate::tpu::TPU;
use strum_macros::{Display, EnumCount as EnumCountMacro, EnumIter, EnumString, FromRepr};

/// Enum representing the available registers
#[derive(Debug, Clone, Copy, FromRepr, EnumIter, EnumCountMacro, PartialEq, Eq)]
#[repr(u8)]
pub enum Register {
    A = 0,
    X = 1,
    Y = 2,
    R0 = 3,
    R1 = 4,
    R2 = 5,
    R3 = 6,
    R4 = 7,
    R5 = 8,
    R6 = 9,
}

#[derive(Debug, Clone, Copy, FromRepr, EnumIter, EnumCountMacro, PartialEq, Eq)]
#[repr(u16)]
pub enum AnalogPin {
    Analog0 = 0,
    Analog1 = 1,
    Analog2 = 2,
    Analog3 = 3,
}

#[derive(Debug, Clone, Copy, FromRepr, EnumIter, EnumCountMacro, PartialEq, Eq)]
#[repr(u16)]
pub enum DigitalPin {
    Digital0 = 0,
    Digital1 = 1,
    Digital2 = 2,
    Digital3 = 3,
    Digital4 = 4,
    Digital5 = 5,
    Digital6 = 6,
    Digital7 = 7,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetPacket {
    pub sender: u16,
    pub target: u16,
    pub data: u16,
}

#[derive(Debug, Clone, PartialEq, Copy, EnumString, Display)]
pub enum Opcode {
    // Stack operations
    PUSH,
    POP,
    PUSHX,
    POPX,
    PEEK,
    SCR,
    RSP,

    // Network operations
    XMIT,
    RECV,
    TXBS,
    RXBS,

    // Math operators
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    AND,
    OR,
    XOR,
    NOT,
    INCA,
    INCX,
    INCY,
    DECA,
    DECX,
    DECY,

    // Bitshifting operations
    SHLR,
    SHLC,
    SHLA,
    SHRR,
    SHRC,
    SHRA,

    // Rotate operations
    ROL,
    ROR,

    // Memory operations
    RCY,
    RMV,
    STR,
    LDR,
    LDM,
    LDA,
    LDX,
    LDXI,
    STM,
    STA,
    STX,
    STXI,

    // Digital Pin operations
    DPW,
    DPWH,
    DPR,
    DPWW,
    DPRW,

    // Analog Pin operations
    APW,
    APWH,
    APR,

    // Misc operations
    SLP,
    WRX,
    WTX,
    HLT,

    // Branching
    JMP,
    BEZ,
    BNZ,
    BEQ,
    BNE,
    BGE,
    BLE,
    BGT,
    BLT,

    // Relative Branches
    JPR,
    BREZ,
    BRNZ,
    BREQ,
    BRNE,
    BRGE,
    BRLE,
    BRGT,
    BRLT,

    // Subroutines
    GSUB,
    RSUB,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operand {
    Register(Register),
    Constant(u16),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Register(reg) => write!(f, "{}", format!("{:?}", reg)),
            Operand::Constant(val) => write!(f, "{:04X}", val),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.opcode)?;
        if !self.operands.is_empty() {
            write!(f, " ")?;
            for (i, operand) in self.operands.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", operand)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub(crate) struct DecodedOpcode {
    pub(crate) operands: Vec<Operand>,
    pub(crate) function: fn(&mut TPU, operands: &[Operand]) -> bool,
    pub(crate) cycles: u16,
    /// Whether this instruction has modified the program counter
    pub(crate) pc_modified: bool,
}

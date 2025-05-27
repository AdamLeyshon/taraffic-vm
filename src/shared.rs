use strum_macros::{Display, EnumCount as EnumCountMacro, EnumIter, EnumString, FromRepr};

/// Enum representing the available registers
#[derive(Debug, Clone, Copy, FromRepr, EnumIter, EnumString, EnumCountMacro, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperandValueType {
    Immediate(u16),
    Register(Register),
}

/// An instruction, comprising an opcode and operands
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    // Stack operations
    /// Push operand to Stack
    PUSH(OperandValueType),
    /// Pop a value from the stack into Register
    POP(Register),
    /// Copy the value from Stack without removing it into Register
    PEEK(Register, OperandValueType),
    /// Stack Clear
    SCR,
    /// Read Stack Pointer into Register
    RSP(Register),

    // Network operations
    XMIT(Register, OperandValueType),
    RECV,
    TXBS,
    RXBS,

    // Math operators
    ADD(Register, Register),
    SUB(Register, Register),
    MUL(Register, Register),
    DIV(Register, Register),
    MOD(Register, Register),
    AND(Register, Register),
    OR(Register, Register),
    XOR(Register, Register),
    NOT(Register),
    INC(Register),
    DEC(Register),

    // Bitshifting operations
    SLL(Register, Register, OperandValueType),
    SLC(Register, Register, OperandValueType),
    SLR(Register, Register, OperandValueType),
    SRC(Register, Register, OperandValueType),

    // Rotate operations
    ROL(Register, Register, OperandValueType),
    ROR(Register, Register, OperandValueType),

    // Memory operations
    /// Register Copy
    RCY(Register, Register),
    /// Register Move
    RMV(Register, Register),
    /// Load Register
    LDR(Register, OperandValueType),
    /// Load Register w/Offset
    LDO(Register, OperandValueType, Register),
    /// Load Register w/Offset+Inc
    LDOI(Register, OperandValueType, Register),
    /// Store Memory
    STM(OperandValueType, OperandValueType),
    /// Store Memory w/Offset
    STMO(OperandValueType, OperandValueType, Register),
    /// Store Memory w/Offset+Inc
    SMOI(OperandValueType, OperandValueType, Register),

    // Digital Pin operations
    DPW(OperandValueType, OperandValueType),
    //DPWH(OperandValueType),
    DPR(Register, OperandValueType),
    DPWW(OperandValueType),
    DPRW(Register),

    // Analog Pin operations
    APW(OperandValueType, OperandValueType),
    //APWH(OperandValueType, OperandValueType),
    APR(Register, OperandValueType),

    // Misc operations
    NOP,
    SLP(OperandValueType),
    WRX,
    HLT,

    // Branching
    JMP(OperandValueType),
    BEZ(OperandValueType, Register),
    BNZ(OperandValueType, Register),
    BEQ(OperandValueType, Register, OperandValueType),
    BNE(OperandValueType, Register, OperandValueType),
    BGE(OperandValueType, Register, OperandValueType),
    BLE(OperandValueType, Register, OperandValueType),
    BGT(OperandValueType, Register, OperandValueType),
    BLT(OperandValueType, Register, OperandValueType),

    // Relative Branches
    JPR(OperandValueType),
    BREZ(OperandValueType, Register),
    BRNZ(OperandValueType, Register),
    BREQ(OperandValueType, Register, OperandValueType),
    BRNE(OperandValueType, Register, OperandValueType),
    BRGE(OperandValueType, Register, OperandValueType),
    BRLE(OperandValueType, Register, OperandValueType),
    BRGT(OperandValueType, Register, OperandValueType),
    BRLT(OperandValueType, Register, OperandValueType),

    // Subroutines
    JSR(OperandValueType),
    RTS,
}

impl std::fmt::Display for OperandValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperandValueType::Register(reg) => write!(f, "{}", format!("{:?}", reg)),
            OperandValueType::Immediate(val) => write!(f, "{:04X}", val),
        }
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct Instruction {
//     pub opcode: Opcode,
//     pub operands: Vec<Operand>,
// }
//
// impl std::fmt::Display for Instruction {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.opcode)?;
//         if !self.operands.is_empty() {
//             write!(f, " ")?;
//             for (i, operand) in self.operands.iter().enumerate() {
//                 if i > 0 {
//                     write!(f, ", ")?;
//                 }
//                 write!(f, "{}", operand)?;
//             }
//         }
//         Ok(())
//     }
// }

#[derive(Clone)]
pub(crate) struct DecodeResult {
    /// How many cycles to wait before executing
    pub(crate) cycles: u16,
    pub(crate) call_every_cycle: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExecuteResult {
    /// Advance the program counter and decrement the wait cycle counter
    PCAdvance,
    /// Don't advance the program counter, but still decrement the wait cycle counter
    NoPCAdvance,
    /// The instruction was executed, the program counter was modified, fetched the next instruction
    PCModified,
    /// Halt the CPU
    Halt(HaltReason),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HaltReason {
    Div0,
    HLTOpcode,
    InvalidPC,
    InvalidValue,
    StackOverflow,
    IndexOutOfRange,
}

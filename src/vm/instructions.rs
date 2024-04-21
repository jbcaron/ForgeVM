use super::error::{Result, VmError};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction<D, A> {
    /// No operation
    NOP,
    /// Move the value into a register
    MOV { dest: u8, value: D },
    /// Load a value from memory into a register
    LD { dest: u8, address: A },
    /// Store a value from a register into memory
    ST { src: u8, address: A },
    /// Logical AND two registers and store the result in a destination register
    AND { dest: u8, reg1: u8, reg2: u8 },
    /// Logical OR two registers and store the result in a destination register
    OR { dest: u8, reg1: u8, reg2: u8 },
    /// Logical XOR two registers and store the result in a destination register
    XOR { dest: u8, reg1: u8, reg2: u8 },
    /// Logical NOT a register and store the result in a destination register
    NOT { dest: u8, reg: u8 },
    /// Compare two registers and set the zero flag if they are equal
    CMP { reg1: u8, reg2: u8 },
    /// Add two registers and store the result in a destination register
    ADD { dest: u8, reg1: u8, reg2: u8 },
    /// Subtract two registers and store the result in a destination register
    SUB { dest: u8, reg1: u8, reg2: u8 },
    /// Multiply two registers and store the result in a destination register
    MULT { dest: u8, reg1: u8, reg2: u8 },
    /// Divide two registers and store the result in a destination register
    DIV { dest: u8, reg1: u8, reg2: u8 },
    /// Modulo two registers and store the result in a destination register
    MOD { dest: u8, reg1: u8, reg2: u8 },
    /// Increment a register
    INC { reg: u8 },
    /// Decrement a register
    DEC { reg: u8 },
    /// Push a register onto the stack
    PUSHREG { reg: u8 },
    /// Pop a register from the stack
    POPREG { reg: u8 },
    /// Jump to a location in memory
    JMP { address: A },
    /// Jump if negative flag is set
    JMPN { address: A },
    /// Jump if negative flag is not set
    JMPP { address: A },
    /// Jump if zero flag is set
    JMPZ { address: A },
    /// Call a function
    CALL { address: A },
    /// Return from a function
    RET,
    /// Clear the flags
    CLF,
    /// Stop the program
    HLT,
}

impl<D, T> std::fmt::Display for Instruction<D, T>
where
    D: std::fmt::Display,
    T: std::fmt::Display + std::fmt::LowerHex,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Instruction::NOP => write!(f, "NOP"),
            Instruction::MOV { dest, value } => write!(f, "MOV R{} {}", dest, value),
            Instruction::LD { dest, address } => write!(f, "LD R{} 0x{:x}", dest, address),
            Instruction::ST { src, address } => write!(f, "ST R{} 0x{:x}", src, address),
            Instruction::AND { dest, reg1, reg2 } => write!(f, "AND R{} R{} R{}", dest, reg1, reg2),
            Instruction::OR { dest, reg1, reg2 } => write!(f, "OR R{} R{} R{}", dest, reg1, reg2),
            Instruction::XOR { dest, reg1, reg2 } => write!(f, "XOR R{} R{} R{}", dest, reg1, reg2),
            Instruction::NOT { dest, reg } => write!(f, "NOT R{} R{}", dest, reg),
            Instruction::CMP { reg1, reg2 } => write!(f, "CMP R{} R{}", reg1, reg2),
            Instruction::ADD { dest, reg1, reg2 } => write!(f, "ADD R{} R{} R{}", dest, reg1, reg2),
            Instruction::SUB { dest, reg1, reg2 } => write!(f, "SUB R{} R{} R{}", dest, reg1, reg2),
            Instruction::MULT { dest, reg1, reg2 } => {
                write!(f, "MULT R{} R{} R{}", dest, reg1, reg2)
            }
            Instruction::DIV { dest, reg1, reg2 } => write!(f, "DIV R{} R{} R{}", dest, reg1, reg2),
            Instruction::MOD { dest, reg1, reg2 } => write!(f, "MOD R{} R{} R{}", dest, reg1, reg2),
            Instruction::INC { reg } => write!(f, "INC R{}", reg),
            Instruction::DEC { reg } => write!(f, "DEC R{}", reg),
            Instruction::PUSHREG { reg } => write!(f, "PUSHREG R{}", reg),
            Instruction::POPREG { reg } => write!(f, "POPREG R{}", reg),
            Instruction::JMP { address } => write!(f, "JMP 0x{:x}", address),
            Instruction::JMPN { address } => write!(f, "JMPN 0x{:x}", address),
            Instruction::JMPP { address } => write!(f, "JMPP 0x{:x}", address),
            Instruction::JMPZ { address } => write!(f, "JMPZ 0x{:x}", address),
            Instruction::CALL { address } => write!(f, "CALL 0x{:x}", address),
            Instruction::RET => write!(f, "RET"),
            Instruction::CLF => write!(f, "CLF"),
            Instruction::HLT => write!(f, "HLT"),
        }
    }
}

impl<D, A> Instruction<D, A> {
    pub fn size(&self) -> usize {
        match self {
            Instruction::NOP => 1,
            Instruction::MOV { .. } => 2 + std::mem::size_of::<D>(),
            Instruction::LD { .. } => 2 + std::mem::size_of::<A>(),
            Instruction::ST { .. } => 2 + std::mem::size_of::<A>(),
            Instruction::AND { .. } => 4,
            Instruction::OR { .. } => 4,
            Instruction::XOR { .. } => 4,
            Instruction::NOT { .. } => 3,
            Instruction::CMP { .. } => 3,
            Instruction::ADD { .. } => 4,
            Instruction::SUB { .. } => 4,
            Instruction::MULT { .. } => 4,
            Instruction::DIV { .. } => 4,
            Instruction::MOD { .. } => 4,
            Instruction::INC { .. } => 2,
            Instruction::DEC { .. } => 2,
            Instruction::PUSHREG { .. } => 2,
            Instruction::POPREG { .. } => 2,
            Instruction::JMP { .. } => 5,
            Instruction::JMPN { .. } => 5,
            Instruction::JMPP { .. } => 5,
            Instruction::JMPZ { .. } => 5,
            Instruction::CALL { .. } => 5,
            Instruction::RET => 1,
            Instruction::CLF => 1,
            Instruction::HLT => 1,
        }
    }
}

/// Enumeration of all possible opcodes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpCode {
    NOP = 0x0,
    MOV = 0x1,
    LD = 0x2,
    ST = 0x3,
    AND = 0x4,
    OR = 0x5,
    XOR = 0x6,
    NOT = 0x7,
    CMP = 0x8,
    ADD = 0x9,
    SUB = 0xA,
    MULT = 0xB,
    DIV = 0xC,
    MOD = 0xD,
    INC = 0xE,
    DEC = 0xF,
    PUSHREG = 0x10,
    POPREG = 0x11,
    JMP = 0x12,
    JMPN = 0x13,
    JMPP = 0x14,
    JMPZ = 0x15,
    CALL = 0x16,
    RET = 0x17,
    CLF = 0x18,
    HLT = 0xFF,
}

impl TryFrom<u8> for OpCode {
    type Error = VmError;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0x0 => Ok(OpCode::NOP),
            0x1 => Ok(OpCode::MOV),
            0x2 => Ok(OpCode::LD),
            0x3 => Ok(OpCode::ST),
            0x4 => Ok(OpCode::AND),
            0x5 => Ok(OpCode::OR),
            0x6 => Ok(OpCode::XOR),
            0x7 => Ok(OpCode::NOT),
            0x8 => Ok(OpCode::CMP),
            0x9 => Ok(OpCode::ADD),
            0xA => Ok(OpCode::SUB),
            0xB => Ok(OpCode::MULT),
            0xC => Ok(OpCode::DIV),
            0xD => Ok(OpCode::MOD),
            0xE => Ok(OpCode::INC),
            0xF => Ok(OpCode::DEC),
            0x10 => Ok(OpCode::PUSHREG),
            0x11 => Ok(OpCode::POPREG),
            0x12 => Ok(OpCode::JMP),
            0x13 => Ok(OpCode::JMPN),
            0x14 => Ok(OpCode::JMPP),
            0x15 => Ok(OpCode::JMPZ),
            0x16 => Ok(OpCode::CALL),
            0x17 => Ok(OpCode::RET),
            0x18 => Ok(OpCode::CLF),
            0xFF => Ok(OpCode::HLT),
            _ => Err(VmError::InvalidOpcode { opcode: value }),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(opcode: OpCode) -> u8 {
        opcode as u8
    }
}

impl OpCode {
    pub fn size<D, T>(&self) -> usize {
        match self {
            OpCode::NOP => 1,
            OpCode::MOV => 2 + std::mem::size_of::<D>(),
            OpCode::LD => 2 + std::mem::size_of::<T>(),
            OpCode::ST => 2 + std::mem::size_of::<T>(),
            OpCode::AND => 4,
            OpCode::OR => 4,
            OpCode::XOR => 4,
            OpCode::NOT => 3,
            OpCode::CMP => 3,
            OpCode::ADD => 4,
            OpCode::SUB => 4,
            OpCode::MULT => 4,
            OpCode::DIV => 4,
            OpCode::MOD => 4,
            OpCode::INC => 2,
            OpCode::DEC => 2,
            OpCode::PUSHREG => 2,
            OpCode::POPREG => 2,
            OpCode::JMP => 5,
            OpCode::JMPN => 5,
            OpCode::JMPP => 5,
            OpCode::JMPZ => 5,
            OpCode::CALL => 5,
            OpCode::RET => 1,
            OpCode::CLF => 1,
            OpCode::HLT => 1,
        }
    }
}

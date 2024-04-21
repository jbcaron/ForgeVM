use super::error::{Result, VmError};

/// Represents the set of all possible instructions for the `ForgeVM` virtual machine.
/// Each instruction can manipulate registers, perform arithmetic or logical operations,
/// control program flow, or interact with memory.
///
/// This enum is used to decode and execute instructions from the bytecode loaded into the VM.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction<D, A> {
    // ==========================================
    // Control Flow Instructions
    // ==========================================cargo doc --open
    //
    /// No operation.
    /// Performs no action and is typically used to consume a cycle without any effect.
    NOP,
    /// Jump to a specified address in the program
    ///
    /// # Parameters
    /// - `address`: The address to jump to
    JMP { address: A },

    /// Jump if negative flag is set
    ///
    /// # Parameters
    /// - `address`: The address to jump to if the negative flag is set
    JMPN { address: A },

    /// Jump if negative flag is not set
    ///
    /// # Parameters
    /// - `address`: The address to jump to if the negative flag is not set
    JMPP { address: A },

    /// Jump if zero flag is set
    ///
    /// # Parameters
    /// - `address`: The address to jump to if the zero flag is set
    JMPZ { address: A },

    /// Call a function at a specified address in the program
    /// This operation pushes the current program counter onto the stack and jumps to the specified address.
    ///
    /// # Parameters
    /// - `address`: The address of the function to call
    CALL { address: A },

    /// Return from a function call
    /// This operation pops the top of the stack and sets the program counter to the popped value.
    ///
    RET,

    /// Halt the program execution
    /// This operation stops the program execution.
    HLT,

    /// Moves a specified `value` into the designated `dest` register.
    ///
    /// # Parameters
    /// - `dest`: The register number into which the value will be moved.
    /// - `value`: The value to store in the register.
    ///
    MOV { dest: u8, value: D },

    /// Loads a value from the specified `address` in memory into the `dest` register.
    /// This operation reads the memory at the given address and updates the register with the value found.
    ///
    /// # Parameters
    /// - `dest`: The destination register where the memory content will be loaded.
    /// - `address`: Memory address from which data is to be read.
    LD { dest: u8, address: A },

    /// Stores the value from `src` register into the memory at `address`.
    /// This operation writes the value in the specified register to the specified address in memory.
    ///
    /// # Parameters
    /// - `src`: The source register whose content is to be stored.
    /// - `address`: Memory address at which the data is to be stored.
    ST { src: u8, address: A },

    /// Push the value from `reg` register onto the stack.
    /// This operation pushes the value from the specified register onto the stack.
    ///
    /// # Parameters
    /// - `reg`: The register whose value is to be pushed onto the stack.
    PUSHREG { reg: u8 },

    /// Pop the value from the top of the stack into the `reg` register.
    /// This operation pops the value from the top of the stack and stores it in the specified register.
    ///
    /// # Parameters
    /// - `reg`: The register into which the value from the stack is to be popped.
    POPREG { reg: u8 },

    // ==========================================
    // Arithmetic Instructions
    // ==========================================
    //
    /// Add two registers and store the result in a destination register
    /// This operation adds the values in two registers and stores the result in the destination register.
    /// If the result is too large to fit in the register, the overflow flag is set.
    /// If the result is zero, the zero flag is set.
    /// If the result is negative, the negative flag is set.
    ///
    /// # Parameters
    /// - `dest`: The destination register where the result will be stored.
    /// - `reg1`: The first register containing the value to be added.
    /// - `reg2`: The second register containing the value to be added.
    ADD { dest: u8, reg1: u8, reg2: u8 },

    /// Subtract two registers and store the result in a destination register
    /// This operation subtracts the value in the second register from the value in the first register
    /// and stores the result in the destination register.
    /// If the result is too large to fit in the register, the overflow flag is set.
    /// If the result is zero, the zero flag is set.
    /// If the result is negative, the negative flag is set.
    ///
    /// # Parameters
    /// - `dest`: The destination register where the result will be stored.
    /// - `reg1`: The first register containing the value to be subtracted from.
    /// - `reg2`: The second register containing the value to be subtracted.
    SUB { dest: u8, reg1: u8, reg2: u8 },

    /// Multiply two registers and store the result in a destination register
    /// This operation multiplies the values in two registers and stores the result in the destination register.
    /// If the result is too large to fit in the register, the overflow flag is set.
    /// If the result is zero, the zero flag is set.
    /// If the result is negative, the negative flag is set.
    ///
    /// # Parameters
    /// - `dest`: The destination register where the result will be stored.
    /// - `reg1`: The first register containing the value to be multiplied.
    /// - `reg2`: The second register containing the value to be multiplied.
    MULT { dest: u8, reg1: u8, reg2: u8 },

    /// Divide two registers and store the result in a destination register
    /// This operation divides the value in the first register by the value in the second register
    /// and stores the result in the destination register.
    /// If the result is too large to fit in the register, the overflow flag is set.
    /// If the result is zero, the zero flag is set.
    /// If the result is negative, the negative flag is set.
    ///
    /// # Parameters
    /// - `dest`: The destination register where the result will be stored.
    /// - `reg1`: The first register containing the dividend.
    /// - `reg2`: The second register containing the divisor.
    DIV { dest: u8, reg1: u8, reg2: u8 },

    /// Calculate the remainder of dividing two registers and store the result in a destination register
    /// This operation calculates the remainder of dividing the value in the first register by the value in the second register
    /// and stores the result in the destination register.
    /// If the result is zero, the zero flag is set.
    ///
    /// # Parameters
    /// - `dest`: The destination register where the result will be stored.
    /// - `reg1`: The first register containing the dividend.
    /// - `reg2`: The second register containing the divisor.
    MOD { dest: u8, reg1: u8, reg2: u8 },

    /// Increment a register
    /// This operation increments the value in the specified register by one.
    /// If the result is too large to fit in the register, the overflow flag is set.
    /// If the result is zero, the zero flag is set.
    /// If the result is negative, the negative flag is set.
    ///
    /// # Parameters
    /// - `reg`: The register to increment.
    INC { reg: u8 },

    /// Decrement a register
    /// This operation decrements the value in the specified register by one.
    /// If the result is too large to fit in the register, the overflow flag is set.
    /// If the result is zero, the zero flag is set.
    /// If the result is negative, the negative flag is set.
    ///
    /// # Parameters
    /// - `reg`: The register to decrement.
    DEC { reg: u8 },

    // ==========================================
    // Logical Instructions
    // ==========================================
    //
    /// Logical AND two registers and store the result in a destination register
    /// This operation performs a bitwise AND operation on the values in two registers
    /// and stores the result in the destination register.
    /// If the result is zero, the zero flag is set.
    /// If the result is negative, the negative flag is set.
    ///
    /// # Parameters
    /// - `dest`: The destination register where the result will be stored.
    /// - `reg1`: The first register containing the value to be ANDed.
    /// - `reg2`: The second register containing the value to be ANDed.
    AND { dest: u8, reg1: u8, reg2: u8 },

    /// Logical OR two registers and store the result in a destination register
    /// This operation performs a bitwise OR operation on the values in two registers
    /// and stores the result in the destination register.
    /// If the result is zero, the zero flag is set.
    /// If the result is negative, the negative flag is set.
    ///
    /// # Parameters
    /// - `dest`: The destination register where the result will be stored.
    /// - `reg1`: The first register containing the value to be ORed.
    /// - `reg2`: The second register containing the value to be ORed.
    OR { dest: u8, reg1: u8, reg2: u8 },

    /// Logical XOR two registers and store the result in a destination register
    /// This operation performs a bitwise XOR operation on the values in two registers
    /// and stores the result in the destination register.
    /// If the result is zero, the zero flag is set.
    /// If the result is negative, the negative flag is set.
    ///
    /// # Parameters
    /// - `dest`: The destination register where the result will be stored.
    /// - `reg1`: The first register containing the value to be XORed.
    /// - `reg2`: The second register containing the value to be XORed.
    XOR { dest: u8, reg1: u8, reg2: u8 },

    /// Logical NOT a register and store the result in a destination register
    /// This operation performs a bitwise NOT operation on the value in the specified register
    /// and stores the result in the destination register.
    /// If the result is zero, the zero flag is set.
    /// If the result is negative, the negative flag is set.
    ///
    /// # Parameters
    /// - `dest`: The destination register where the result will be stored.
    /// - `reg`: The register containing the value to be NOTed.
    NOT { dest: u8, reg: u8 },

    /// Compare two registers
    /// This operation compares the values in two registers and sets the zero flag if they are equal.
    ///
    /// # Parameters
    /// - `reg1`: The first register to compare.
    /// - `reg2`: The second register to compare.
    CMP { reg1: u8, reg2: u8 },

    // ==========================================
    // Flag Operations
    // ==========================================
    //
    /// Clear the flags
    /// This operation clears all the flags in the status register.
    CLF,
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

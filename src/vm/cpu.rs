use super::error::{Result as VmResult, VmError};
use super::hardware_config::REGISTERS_COUNT;
use super::instructions::Instruction;
use super::memory::Memory;
use super::stack::Stack;

/// The CPU structure used by the VM.
/// The CPU has a fixed number of registers and status flags.
/// The CPU has a program counter (PC) that points to the current instruction.
/// The CPU can execute instructions and interact with memory and the stack.
/// The CPU is generic over the data type used for the registers.
pub struct CPU<T> {
    registers: [T; REGISTERS_COUNT as usize],
    status_flags: StatusFlags,
    pc: usize,
}

/// Implementation of the CPU for the 32-bit architecture
/// The index of the registers was verified in decoder.rs
impl CPU<i32> {
    pub fn new() -> Self {
        Self {
            registers: [0; REGISTERS_COUNT as usize],
            status_flags: StatusFlags::default(),
            pc: 0,
        }
    }

    /// Initialize the CPU by clearing the registers and status flags.
    /// The program counter is set to zero.
    pub fn init(&mut self) {
        self.registers = [0; REGISTERS_COUNT as usize];
        self.status_flags.clear();
        self.pc = 0;
    }

    /// Get the program counter (PC) of the CPU.
    pub fn pc(&self) -> usize {
        self.pc
    }

    /// Get the value of a register by index.
    /// 
    /// # Parameters
    /// - `index`: The index of the register to get.
    /// 
    /// # Returns
    /// The value of the register.
    /// 
    /// # Errors
    /// Returns an error if the register index is out of bounds.
    pub fn get_register(&self, index: u8) -> VmResult<i32> {
        if index as usize >= REGISTERS_COUNT as usize {
            return Err(VmError::InvalidRegister { register: index });
        }
        Ok(self.registers[index as usize])
    }

    /// Execute an instruction on the CPU.
    /// The instruction modifies the registers, status flags, program counter, memory, and stack.
    /// 
    /// # Parameters
    /// - `instruction`: The instruction to execute.
    /// - `memory`: The memory to read from and write to.
    /// - `stack`: The stack to push to and pop from.
    /// 
    /// # Errors
    /// Returns an error if the instruction is invalid or if the HLT instruction is executed.
    /// 
    /// **Note:** Instructions that use registers did already validate by the decoder.
    /// The registers are accessed directly without additional validation.
    pub fn execute_instruction(
        &mut self,
        instruction: Instruction<i32, u32>,
        memory: &mut Memory,
        stack: &mut Stack<i32>,
    ) -> VmResult<()> {
        match instruction {
            Instruction::NOP => {}
            Instruction::MOV { dest, value } => {
                self.registers[dest as usize] = value;
            }
            Instruction::LD { dest, address } => {
                self.registers[dest as usize] = memory.read::<i32>(address as usize)?;
            }
            Instruction::ST { src, address } => {
                memory.write::<i32>(address as usize, self.registers[src as usize])?;
            }
            Instruction::ADD { dest, reg1, reg2 } => {
                let (result, overflow) =
                    self.registers[reg1 as usize].overflowing_add(self.registers[reg2 as usize]);

                self.registers[dest as usize] = result;

                self.status_flags.overflow = overflow;
                self.status_flags.zero = result == 0;
                self.status_flags.negative = result < 0;
            }
            Instruction::SUB { dest, reg1, reg2 } => {
                let (result, overflow) =
                    self.registers[reg1 as usize].overflowing_sub(self.registers[reg2 as usize]);

                self.registers[dest as usize] = result;

                self.status_flags.overflow = overflow;
                self.status_flags.zero = result == 0;
                self.status_flags.negative = result < 0;
            }
            Instruction::MULT { dest, reg1, reg2 } => {
                let (result, overflow) =
                    self.registers[reg1 as usize].overflowing_mul(self.registers[reg2 as usize]);

                self.registers[dest as usize] = result;

                self.status_flags.overflow = overflow;
                self.status_flags.zero = result == 0;
                self.status_flags.negative = result < 0;
            }
            Instruction::DIV { dest, reg1, reg2 } => {
                let (result, overflow) =
                    self.registers[reg1 as usize].overflowing_div(self.registers[reg2 as usize]);

                self.registers[dest as usize] = result;

                self.status_flags.overflow = overflow;
                self.status_flags.zero = result == 0;
                self.status_flags.negative = result < 0;
            }
            Instruction::MOD { dest, reg1, reg2 } => {
                let result = self.registers[reg1 as usize] % self.registers[reg2 as usize];

                self.registers[dest as usize] = result;

                self.status_flags.zero = result == 0;
                self.status_flags.negative = result < 0;
            }
            Instruction::AND { dest, reg1, reg2 } => {
                let result = self.registers[reg1 as usize] & self.registers[reg2 as usize];

                self.registers[dest as usize] = result;

                self.status_flags.zero = result == 0;
                self.status_flags.negative = result < 0;
            }
            Instruction::OR { dest, reg1, reg2 } => {
                let result = self.registers[reg1 as usize] | self.registers[reg2 as usize];

                self.registers[dest as usize] = result;

                self.status_flags.zero = result == 0;
                self.status_flags.negative = result < 0;
            }
            Instruction::XOR { dest, reg1, reg2 } => {
                let result = self.registers[reg1 as usize] ^ self.registers[reg2 as usize];

                self.registers[dest as usize] = result;

                self.status_flags.zero = result == 0;
                self.status_flags.negative = result < 0;
            }
            Instruction::NOT { dest, reg } => {
                let result = !self.registers[reg as usize];

                self.registers[dest as usize] = result;

                self.status_flags.zero = result == 0;
                self.status_flags.negative = result < 0;
            }
            Instruction::CMP { reg1, reg2 } => {
                let result = self.registers[reg1 as usize].cmp(&self.registers[reg2 as usize]);

                self.status_flags.zero = result == std::cmp::Ordering::Equal;
            }
            Instruction::INC { reg } => {
                let (result, overflow) = self.registers[reg as usize].overflowing_add(1);

                self.registers[reg as usize] = result;

                self.status_flags.overflow = overflow;
                self.status_flags.zero = result == 0;
                self.status_flags.negative = result < 0;
            }
            Instruction::DEC { reg } => {
                let (result, overflow) = self.registers[reg as usize].overflowing_sub(1);

                self.registers[reg as usize] = result;

                self.status_flags.overflow = overflow;
                self.status_flags.zero = result == 0;
                self.status_flags.negative = result < 0;
            }
            Instruction::PUSHREG { reg } => {
                stack.push(self.registers[reg as usize])?;
            }
            Instruction::POPREG { reg } => {
                self.registers[reg as usize] = stack.pop()?;
            }
            Instruction::JMP { address } => {
                self.pc = address as usize;
            }
            Instruction::JMPN { address } => {
                if self.status_flags.negative {
                    self.pc = address as usize;
                }
            }
            Instruction::JMPP { address } => {
                if !self.status_flags.negative {
                    self.pc = address as usize;
                }
            }
            Instruction::JMPZ { address } => {
                if self.status_flags.zero {
                    self.pc = address as usize;
                }
            }
            Instruction::CALL { address } => {
                stack.push(self.pc as i32)?;
                self.pc = address as usize;
            }
            Instruction::RET => {
                self.pc = stack.pop()? as usize;
            }
            Instruction::CLF => {
                self.status_flags.clear();
            }
            Instruction::HLT => {
                return Err(VmError::Other("HLT instruction executed".to_string()));
            }
        }
        self.pc += instruction.size();
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StatusFlags {
    pub zero: bool,
    pub carry: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl StatusFlags {
    pub fn clear(&mut self) {
        self.zero = false;
        //self.carry = false;
        self.overflow = false;
        self.negative = false;
    }
}

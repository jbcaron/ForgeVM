use super::error::{Result as VmResult, VmError};
use super::hardware_config::REGISTERS_COUNT;
use super::instructions::Instruction;
use super::memory::Memory;
use super::stack::Stack;

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

    pub fn init(&mut self) {
        self.registers = [0; REGISTERS_COUNT as usize];
        self.status_flags.clear();
        self.pc = 0;
    }

    pub fn pc_mut(&mut self) -> &mut usize {
        &mut self.pc
    }

    pub fn get_register(&self, index: u8) -> VmResult<i32> {
        if index as usize >= REGISTERS_COUNT as usize {
            return Err(VmError::InvalidRegister { register: index });
        }
        Ok(self.registers[index as usize])
    }

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

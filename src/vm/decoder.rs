use super::error::{Result as VmResult, VmError};
use super::hardware_config::REGISTERS_COUNT;
use super::instructions::{Instruction, OpCode};
use super::program::Program;

pub struct Decoder;

/// implementation of the Decoder for the 32-bit architecture
impl Decoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decode_next_instruction(
        &self,
        program: &Program,
        pc: usize,
    ) -> VmResult<Instruction<i32, u32>> {
        let program_slice = program.slice_from(pc);

        // check if the program slice is empty and contains at least the opcode
        if program_slice.is_empty() {
            return Err(VmError::InvalidInstruction);
        }

        // convert the first byte of the program slice to an OpCode
        let opcode: OpCode = program_slice[0].try_into()?;

        let instruction_len = opcode.size::<i32, u32>();

        // check if the program slice is long enough to contain the instruction
        if program_slice.len() < instruction_len {
            return Err(VmError::InvalidInstruction);
        }

        match opcode {
            OpCode::NOP => Ok(Instruction::<i32, u32>::NOP),
            OpCode::MOV => {
                let dest = program_slice[1];
                let value = read_i32(program_slice, 2)?;
                Ok(Instruction::<i32, u32>::MOV { dest, value })
            }
            OpCode::LD => {
                let dest = program_slice[1];
                let address = read_u32(program_slice, 2)?;
                Ok(Instruction::<i32, u32>::LD { dest, address })
            }
            OpCode::ST => {
                if program_slice.len() < 6 {
                    return Err(VmError::InvalidInstruction);
                }
                let src = program_slice[1];
                let address = read_u32(program_slice, 2)?;
                Ok(Instruction::<i32, u32>::ST { src, address })
            }
            OpCode::AND => {
                let dest = program_slice[1];
                let reg1 = register_address(program_slice[2])?;
                let reg2 = register_address(program_slice[3])?;
                Ok(Instruction::<i32, u32>::AND { dest, reg1, reg2 })
            }
            OpCode::OR => {
                let dest = program_slice[1];
                let reg1 = register_address(program_slice[2])?;
                let reg2 = register_address(program_slice[3])?;
                Ok(Instruction::<i32, u32>::OR { dest, reg1, reg2 })
            }
            OpCode::XOR => {
                let dest = register_address(program_slice[1])?;
                let reg1 = register_address(program_slice[2])?;
                let reg2 = register_address(program_slice[3])?;
                Ok(Instruction::<i32, u32>::XOR { dest, reg1, reg2 })
            }
            OpCode::NOT => {
                let dest = register_address(program_slice[1])?;
                let reg = register_address(program_slice[2])?;
                Ok(Instruction::<i32, u32>::NOT { dest, reg })
            }
            OpCode::CMP => {
                let reg1 = register_address(program_slice[1])?;
                let reg2 = register_address(program_slice[2])?;
                Ok(Instruction::<i32, u32>::CMP { reg1, reg2 })
            }
            OpCode::ADD => {
                let dest = register_address(program_slice[1])?;
                let reg1 = register_address(program_slice[2])?;
                let reg2 = register_address(program_slice[3])?;
                Ok(Instruction::<i32, u32>::ADD { dest, reg1, reg2 })
            }
            OpCode::SUB => {
                let dest = register_address(program_slice[1])?;
                let reg1 = register_address(program_slice[2])?;
                let reg2 = register_address(program_slice[3])?;
                Ok(Instruction::<i32, u32>::SUB { dest, reg1, reg2 })
            }
            OpCode::MULT => {
                let dest = register_address(program_slice[1])?;
                let reg1 = register_address(program_slice[2])?;
                let reg2 = register_address(program_slice[3])?;
                Ok(Instruction::<i32, u32>::MULT { dest, reg1, reg2 })
            }
            OpCode::DIV => {
                let dest = register_address(program_slice[1])?;
                let reg1 = register_address(program_slice[2])?;
                let reg2 = register_address(program_slice[3])?;
                Ok(Instruction::<i32, u32>::DIV { dest, reg1, reg2 })
            }
            OpCode::MOD => {
                let dest = register_address(program_slice[1])?;
                let reg1 = register_address(program_slice[2])?;
                let reg2 = register_address(program_slice[3])?;
                Ok(Instruction::<i32, u32>::MOD { dest, reg1, reg2 })
            }
            OpCode::INC => {
                let reg = register_address(program_slice[1])?;
                Ok(Instruction::<i32, u32>::INC { reg })
            }
            OpCode::DEC => {
                let reg = register_address(program_slice[1])?;
                Ok(Instruction::<i32, u32>::DEC { reg })
            }
            OpCode::PUSHREG => {
                let reg = register_address(program_slice[1])?;
                Ok(Instruction::<i32, u32>::PUSHREG { reg })
            }
            OpCode::POPREG => {
                let reg = register_address(program_slice[1])?;
                Ok(Instruction::<i32, u32>::POPREG { reg })
            }
            OpCode::JMP => {
                let address = read_u32(program_slice, 1)?;
                Ok(Instruction::<i32, u32>::JMP { address })
            }
            OpCode::JMPN => {
                let address = read_u32(program_slice, 1)?;
                Ok(Instruction::<i32, u32>::JMPN { address })
            }
            OpCode::JMPP => {
                let address = read_u32(program_slice, 1)?;
                Ok(Instruction::<i32, u32>::JMPP { address })
            }
            OpCode::JMPZ => {
                let address = read_u32(program_slice, 1)?;
                Ok(Instruction::<i32, u32>::JMPZ { address })
            }
            OpCode::CALL => {
                let address = read_u32(program_slice, 1)?;
                Ok(Instruction::<i32, u32>::CALL { address })
            }
            OpCode::RET => Ok(Instruction::<i32, u32>::RET),
            OpCode::CLF => Ok(Instruction::<i32, u32>::CLF),
            OpCode::HLT => Ok(Instruction::<i32, u32>::HLT),
        }
    }
}

fn register_address(register: u8) -> VmResult<u8> {
    if register >= REGISTERS_COUNT {
        return Err(VmError::InvalidRegister { register });
    }
    Ok(register)
}

/// Read a little-endian i32 from a slice of bytes
/// the start parameter is the index of the first byte of the i32
/// the length of the slice must be at least start + 4
fn read_i32(data: &[u8], start: usize) -> VmResult<i32> {
    data[start..start + 4]
        .try_into()
        .map(i32::from_le_bytes)
        .map_err(|_| VmError::InvalidInstruction)
}

/// Read a little-endian u32 from a slice of bytes
/// the start parameter is the index of the first byte of the u32
/// the length of the slice must be at least start + 4
fn read_u32(data: &[u8], start: usize) -> VmResult<u32> {
    data[start..start + 4]
        .try_into()
        .map(u32::from_le_bytes)
        .map_err(|_| VmError::InvalidInstruction)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_i32() {
        let data = [0x78, 0x56, 0x34, 0x12];
        assert_eq!(read_i32(&data, 0).unwrap(), 0x12345678);
    }

    #[test]
    fn test_read_u32() {
        let data = [0x78, 0x56, 0x34, 0x12];
        assert_eq!(read_u32(&data, 0).unwrap(), 0x12345678);
    }
}

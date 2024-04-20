use super::error::{Result, VmError};
use super::instructions::Instruction;
use super::program::Program;

pub struct Decoder;

// pub struct Decoder<T> {
//     opcode_map: HashMap<u8, Box<dyn Fn(&[u8]) -> Result<Instruction<T>>>>,
// }

/// implementation of the Decoder for the 32-bit architecture
impl Decoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decode_next_instruction(
        &self,
        program: &Program,
        pc: &mut usize,
    ) -> Result<Instruction<i32>> {
        let program_slice = program.slice_from(*pc);
        if program_slice.is_empty() {
            return Err(VmError::InvalidInstruction);
        }
        let opcode = program_slice[0];
        match opcode {
            0x0 => {
                *pc += 1;
                Ok(Instruction::<i32>::NOP)
            }
            0x1 => {
                if program_slice.len() < 6 {
                    return Err(VmError::InvalidInstruction);
                }
                let dest = program_slice[1];
                let value = i32::from_le_bytes(
                    program_slice[2..6]
                        .try_into()
                        .map_err(|_| VmError::InvalidInstruction)?,
                );
                *pc += 6;
                Ok(Instruction::<i32>::MOV { dest, value })
            }
            0x2 => {
                if program_slice.len() < 3 {
                    return Err(VmError::InvalidInstruction);
                }
                let dest = program_slice[1];
                let src = program_slice[2];
                *pc += 3;
                Ok(Instruction::<i32>::MOVREG { dest, src })
            }
            _ => Err(VmError::InvalidOpcode { opcode }),
        }
    }
}

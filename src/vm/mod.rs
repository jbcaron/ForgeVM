pub mod cpu;
pub mod decoder;
pub mod error;
pub mod hardware_config;
pub mod instructions;
pub mod memory;
pub mod program;
pub mod stack;

pub struct VM<T> {
    stack: stack::Stack<T>,
    memory: memory::Memory,
    cpu: cpu::CPU<T>,
    steps: u128,
}

/// Implementation of the VM for the 32-bit architecture
impl VM<i32> {
    pub fn new(stack_capacity: usize, memory_size: usize) -> Self {
        Self {
            stack: stack::Stack::<i32>::new(stack_capacity),
            memory: memory::Memory::new(memory_size),
            cpu: cpu::CPU::<i32>::new(),
            steps: 0,
        }
    }

    pub fn run(&mut self, program: &[u8]) -> Result<u128, error::VmError> {
        self.steps = 0;
        self.cpu.init();
        self.memory.clear();
        self.stack.clear();
        let program = program::Program::new(program);
        let decoder = decoder::Decoder::new();

        loop {
            let instructions = decoder.decode_next_instruction(&program, self.cpu.pc_mut())?;
            self.steps += 1;
            if instructions == instructions::Instruction::<i32, u32>::HLT {
                break;
            }
            self.cpu
                .execute_instruction(instructions, &mut self.memory, &mut self.stack)?;
        }
        Ok(self.steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_create() {
        let vm = VM::<i32>::new(1024, 1024);
        assert_eq!(vm.stack.capacity(), 1024);
        assert_eq!(vm.memory.capacity(), 1024);
    }

    #[test]
    fn test_vm_run() {
        let mut vm = VM::<i32>::new(1024, 1024);
        let program = vec![0x00, 0x00, 0xff]; // NOP, NOP, HLT
        assert_eq!(vm.run(&program), Ok(3));
    }

    #[test]
    fn test_vm_run_with_stack_overflow() {
        let mut vm = VM::<i32>::new(1, 1024);
        let program = vec![0x10, 0x00, 0x10, 0x00, 0xff]; // PUSH 0, PUSH 0, HLT
        assert_eq!(vm.run(&program), Err(error::VmError::StackOverflow));
    }

    #[test]
    fn test_vm_run_with_stack_underflow() {
        let mut vm = VM::<i32>::new(1024, 1024);
        let program = vec![0x11, 0x00, 0xff]; // POP 0, HLT
        assert_eq!(vm.run(&program), Err(error::VmError::StackUnderflow));
    }

    #[test]
    fn test_vm_run_with_memory_out_of_bounds() {
        let mut vm = VM::<i32>::new(1024, 1024);
        let program = vec![0x02, 0x01, 0x00, 0x04, 0x00, 0x00, 0xff]; // LD 1 0x00000400, HLT
        assert_eq!(
            vm.run(&program),
            Err(error::VmError::MemoryOutOfBounds {
                address: 1024,
                size: 4
            })
        );
    }

    #[test]
    fn test_vm_run_with_invalid_op_code() {
        let mut vm = VM::<i32>::new(1024, 1024);
        let program = vec![0xf0, 0xff]; // Invalid instruction
        assert_eq!(
            vm.run(&program),
            Err(error::VmError::InvalidOpcode { opcode: 0xf0 })
        );
    }

    #[test]
    fn test_vm_run_add() {
        let mut vm = VM::<i32>::new(1024, 1024);
        let program = vec![
            0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x01, 0x07, 0x00, 0x00, 0x00, 0x09, 0x00,
            0x00, 0x01, 0xff,
        ]; // LD 0 0x02, LD 1 0x07, ADD 0 0 1, HLT
        assert_eq!(vm.run(&program), Ok(4));
        assert_eq!(vm.cpu.get_register(0), Ok(9));
    }
}

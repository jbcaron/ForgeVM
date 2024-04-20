pub mod cpu;
pub mod decoder;
pub mod error;
pub mod instructions;
pub mod memory;
pub mod program;
pub mod stack;

pub struct VM<T> {
    stack: stack::Stack<T>,
    memory: memory::Memory,
    cpu: cpu::CPU<T>,
}

impl<T> VM<T>
where
    T: Default + Copy,
{
    pub fn new(stack_capacity: usize, memory_size: usize) -> VM<T> {
        VM {
            stack: stack::Stack::<T>::new(stack_capacity),
            memory: memory::Memory::new(memory_size),
            cpu: cpu::CPU::<T>::new(),
        }
    }

    pub fn run(&mut self, program: &[u8]) -> Result<(), error::VmError> {
        let decoder = decoder::Decoder::new();
        Ok(())
    }
}

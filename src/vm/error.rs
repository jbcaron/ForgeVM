pub type Result<T> = std::result::Result<T, VmError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmError {
    MemoryOutOfBounds { address: usize, size: usize },
    MemoryNotAligned { address: usize, size: usize },
    InvalidOpcode { opcode: u8 },
    InvalidInstruction,
    RegisterOutOfBounds { register: u8 },
    DivisionByZero,
    StackUnderflow,
    StackOverflow,
    Other(String),
}

impl std::fmt::Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VmError::MemoryOutOfBounds { address, size } => {
                write!(
                    f,
                    "Memory access out of bounds at address: {} with size: {}",
                    address, size
                )
            }
            VmError::MemoryNotAligned { address, size } => {
                write!(
                    f,
                    "Memory access not aligned at address: {} with size: {}",
                    address, size
                )
            }
            VmError::InvalidOpcode { opcode } => {
                write!(f, "Invalid opcode encountered: 0x{:02X}", opcode)
            }
            VmError::InvalidInstruction => {
                write!(f, "Invalid instruction encountered")
            }
            VmError::RegisterOutOfBounds { register } => {
                write!(f, "Register out of bounds: {}", register)
            }
            VmError::DivisionByZero => {
                write!(f, "Attempted to divide by zero")
            }
            VmError::StackUnderflow => {
                write!(f, "Stack underflow error")
            }
            VmError::StackOverflow => {
                write!(f, "Stack overflow error")
            }
            VmError::Other(description) => {
                write!(f, "Error: {}", description)
            }
        }
    }
}

impl std::error::Error for VmError {}

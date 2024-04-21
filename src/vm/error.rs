/// This module contains the error types used by the VM.

/// The `Result` type is a type alias for a `Result` type that uses the `VmError` type as the error variant.
pub type Result<T> = std::result::Result<T, VmError>;

/// Represents the possible errors that can occur during the execution of the VM.
/// The errors are related to memory access, invalid instructions, invalid opcodes,
/// invalid registers, division by zero, stack underflow, stack overflow, and other errors.
/// The `Other` variant is used to represent any other error that does not fit the other categories.
/// The `String` field contains a description of the error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmError {
    // ==========================================
    // Memory errors
    // ==========================================
    //
    /// Memory access out of bounds.
    /// Contains the address and the size of the memory access.
    ///
    /// # Parameters
    /// - `address`: The address of the memory access.
    /// - `size`: The size of the memory access.
    MemoryOutOfBounds {
        address: usize,
        size: usize,
    },

    /// Memory access not aligned.
    /// Contains the address and the size of the memory access.
    ///
    /// # Parameters
    /// - `address`: The address of the memory access.
    /// - `size`: The size of the memory access.
    MemoryNotAligned {
        address: usize,
        size: usize,
    },

    // ==========================================
    // Stack errors
    // ==========================================
    //
    /// Stack underflow error.
    /// This error is used when the stack is empty and an operation that requires a value is attempted.
    StackUnderflow,

    /// Stack overflow error.
    /// This error is used when the stack is full and an operation that requires space is attempted.
    StackOverflow,

    // ==========================================
    // Instruction errors
    // ==========================================
    //
    /// Invalid opcode encountered.
    /// Contains the opcode that caused the error.
    ///
    /// # Parameters
    /// - `opcode`: The opcode that caused the error.
    InvalidOpcode {
        opcode: u8,
    },

    /// Invalid instruction encountered.
    /// This error is used when an instruction is invalid.
    /// For example, when the instruction is not long enough to contain the opcode.
    InvalidInstruction,

    // ==========================================
    // Register errors
    // ==========================================
    //
    /// Invalid register encountered, out of bounds.
    /// Contains the register that caused the error.
    /// 
    /// # Parameters
    /// - `register`: The register that caused the error.
    InvalidRegister {
        register: u8,
    },

    // ==========================================
    // Arithmetic errors
    // ==========================================
    //
    /// Division by zero error.
    DivisionByZero,

    // ==========================================
    // Other errors
    // ==========================================
    //
    /// Other error.
    /// Contains a description of the error.
    /// 
    /// # Parameters
    /// - `description`: A description of the error.
    Other(String),
}

impl std::fmt::Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VmError::MemoryOutOfBounds { address, size } => {
                write!(
                    f,
                    "Memory access out of bounds at address: 0x{:x} with size: {}",
                    address, size
                )
            }
            VmError::MemoryNotAligned { address, size } => {
                write!(
                    f,
                    "Memory access not aligned at address: 0x{:x} with size: {}",
                    address, size
                )
            }
            VmError::InvalidOpcode { opcode } => {
                write!(f, "Invalid opcode encountered: 0x{:02x}", opcode)
            }
            VmError::InvalidInstruction => {
                write!(f, "Invalid instruction encountered")
            }
            VmError::InvalidRegister { register } => {
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

pub enum Instruction<T> {
    /// No operation
    NOP,
    ///  Move a value into a register
    MOV { dest: u8, value: T },
    /// Move the value from one register to another
    MOVREG { dest: u8, src: u8 },
    /// Load a value from memory into a register
    LD { dest: u8, address: T },
    /// Add two registers and store the result in a destination register
    ADD { reg1: u8, reg2: u8, dest: u8 },
    /// Subtract two registers and store the result in a destination register
    SUB { reg1: u8, reg2: u8, dest: u8 },
    /// Multiply two registers and store the result in a destination register
    MULT { reg1: u8, reg2: u8, dest: u8 },
    /// Divide two registers and store the result in a destination register
    DIV { reg1: u8, reg2: u8, dest: u8 },
    /// Modulo two registers and store the result in a destination register
    MOD { reg1: u8, reg2: u8, dest: u8 },
    /// Increment a register
    INC { reg: u8 },
    /// Decrement a register
    DEC { reg: u8 },
    /// Jump to a location in memory
    JMP { address: T },
    /// Jump if negative flag is set
    JMPN,
    /// Jump if negative flag is not set
    JMPP,
    /// Jump if zero flag is set
    JMPZ,
    /// Push a value onto the stack
    PSH { value: i32 },
    /// Pop a value from the stack
    POP,
    /// Call a function
    CLL { address: T },
    /// Return from a function
    RET,
}

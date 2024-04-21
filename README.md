![ForgeVM logo](assets/logo.png)

# ForgeVM: _A virtual machine implemented in Rust_

[![CI](https://github.com/jbcaron/ForgeVM/actions/workflows/rust.yml/badge.svg)](https://github.com/jbcaron/ForgeVM/actions/workflows/rust.yml)
![Github licence](https://img.shields.io/github/license/jbcaron/ForgeVM)
![GitHub tag](https://img.shields.io/github/v/tag/jbcaron/ForgeVm)

Welcome to ForgeVM, a virtual machine (VM) implemented in Rust, designed to be robust, safe, and efficient. ForgeVM is an educational project that provides a platform for understanding the inner workings of a virtual machine, including its instruction set, memory management, and execution flow.
The core components include a dynamically sized stack, a configurable memory module, and a CPU that interprets and executes a set of defined instructions.

## Table of Contents
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Building the Project](#building-the-project)
  - [Usage](#usage)
- [Variable-Length Instruction Set and Decoding Process](#variable-length-instruction-set-and-decoding-process)
  - [Understanding Variable-Length Instructions](#understanding-variable-length-instructions)
  - [Decoding with from_le_bytes](#decoding-with-from_le_bytes)
  - [Decoding Steps](#decoding-steps)
- [Overview of VM Instructions](#overview-of-vm-instructions)
  - [Data Movement Instructions](#data-movement-instructions)
  - [Logical Operations](#logical-operations)
  - [Arithmetic Operations](#arithmetic-operations)
  - [Stack Operations](#stack-operations)
  - [Control Flow](#control-flow)
- [License](#license)



## Features

- **Custom Instruction Set**: ForgeVM uses a simplified custom instruction set designed to demonstrate fundamental concepts of virtual machines.
- **Memory Safety**: Leveraging Rust's ownership and borrowing principles, ForgeVM ensures safe memory access throughout its operation.
- **Efficiency**: Optimized for performance, making use of Rust's compilation guarantees to produce fast and reliable code.
- **Extensible**: Easy to extend with new instructions and features, making it a perfect starting point for deeper exploration into VM architecture.


## Getting Started

### Prerequisites

Before you can run ForgeVM, ensure you have Rust installed on your machine. Rust can be installed using `rustup`, which is available at [https://rustup.rs/](https://rustup.rs/).


### Installation

Clone the repository to your local machine using the following command:

```bash
git clone https://github.com/jbcaron/ForgeVM.git
cd ForgeVM
```

### Building the Project
Compile the project using Cargo, Rust's package manager and build system:

```bash
cargo build --release
```

### Usage

To run a program with the VM, ensure that you have a binary file or a byte array that represents the compiled machine code of your program. Here’s how to initiate the VM and execute a program:

```rust
fn main() {
    let mut vm = VM::<i32>::new(1024, 65536); // stack capacity, memory size
    let program = include_bytes!("path/to/your/program.bin");
    match vm.run(program) {
        Ok(steps) => println!("Program completed in {} steps", steps),
        Err(e) => eprintln!("Error during execution: {:?}", e),
    }
}
```

## Variable-Length Instruction Set and Decoding Process

The virtual machine (VM) supports a range of instructions with variable lengths, which allows for efficient use of memory and dynamic instruction handling based on the operational needs. The instructions may vary in length depending on the type and number of operands they require.

### Understanding Variable-Length Instructions

Variable-length instructions in our VM are designed to accommodate operations that require different amounts of data. For instance, some instructions might only need a register identifier, while others might need full 32-bit addresses or immediate values. Here’s how these are generally structured:

- Opcode (1 byte): Uniquely identifies the instruction to be performed.
- Operands: The number and type of operands can vary. Typical operands include:
  - Register identifiers (1 byte each)
  - Immediate values (size depends on the instruction, commonly 4 bytes for i32)
  - Memory addresses (size can vary, e.g., 2 or 4 bytes depending on the configuration)

### Decoding with from_le_bytes
The from_le_bytes method is crucial for converting sequences of bytes into integer values in little-endian order. This method is particularly important for instructions that involve immediate values or addresses.

### Decoding Steps:

1. Fetch Opcode: Read the first byte to determine the type of instruction.
2. Determine Length: Based on the opcode, determine the total length of the instruction to know how many additional bytes to read.
3. Read Operands: For each operand, extract the appropriate number of bytes from the instruction stream.
4. Convert Bytes: Use `from_le_bytes` for each operand that represents a numerical value (not applicable to register identifiers).

## Overview of VM Instructions

The virtual machine supports a diverse set of operations, ranging from basic data movement to complex logical and arithmetic operations. Below is a description of each instruction, its purpose, and usage:

### Data Movement Instructions
- `NOP`:
  - **Description***: No operation. This instruction does nothing and is used primarily for debugging or as a placeholder.
- `MOV { dest, value }`:
  - **Description**: Moves a value directly into a specified register.
  - **Parameters**:
    - `dest`: The destination register index.
    - `value`: The immediate value or the value from another register to be moved.
- `LD { dest, address }`:
  - **Description**: Loads a value from a specified memory address into a register.
  - Parameters:
    - `dest`: The destination register index.
    - `address`: Memory address from which to load the value.
- `ST { src, address }`:
  - **Description**: Stores the value from a register into a specified memory address.
  - **Parameters**:
    - `src`: Source register containing the value to store.
    - `address`: Memory address where the value will be stored.

### Logical Operations
- `AND { dest, reg1, reg2 }`:
  - **Description**: Performs a logical AND operation between two registers and stores the result in the destination register.
  - **Parameters**:
    - `dest`: Destination register for the result.
    - `reg1`, `reg2`: Source registers for the operands.
- `OR { dest, reg1, reg2 }`:
  - **Description**: Performs a logical OR operation between two registers and stores the result in the destination register.
  - **Parameters**:
    - `dest`: Destination register.
    - `reg1`, `reg2`: Registers containing the operands.
- `XOR { dest, reg1, reg2 }`:
  - **Description**: Executes a logical XOR operation between two registers and saves the outcome in the destination register.
  - **Parameters**:
    - `dest`: Destination register.
    - `reg1`, `reg2`: Operand registers.
- `NOT { dest, reg }`:
  - **Description**: Performs a logical NOT operation on a register and stores the result in the destination register.
  - **Parameters**:
    - `dest`: Destination register.
    - `reg`: Source register to be negated.

### Arithmetic Operations
- `ADD { dest, reg1, reg2 }`:
  - **Description**: Adds values from two registers and stores the result in a destination register.
  - **Parameters**:
    - `dest`: Destination register for the sum.
    - `reg1`, `reg2`: Source registers containing the addends.
- `SUB { dest, reg1, reg2 }`:
  - **Description**: Subtracts the second register from the first and stores the result in the destination register.
  - **Parameters**:
    - `dest`: Destination register for the difference.
    - `reg1`, `reg2`: Source registers, where reg1 is the minuend and reg2 the subtrahend.
- `MULT { dest, reg1, reg2 }`:
  - **Description**: Multiplies values from two registers and stores the result in the destination register.
  - **Parameters**:
    - `dest`: Destination register for the product.
    - `reg1`, `reg2`: Source registers containing the multiplicands.
- `DIV { dest, reg1, reg2 }`:
  - **Description**: Divides the first register by the second and stores the quotient in the destination register.
  - **Parameters**:
    - `dest`: Destination register for the quotient.
    - `reg1`: Dividend register.
    - `reg2`: Divisor register.
- `MOD { dest, reg1, reg2 }`:
  - **Description**: Computes the remainder of division between two registers and stores it in the destination register.
  - **Parameters**:
    - `dest`: Destination register for the modulus.
    - `reg1`, `reg2`: Source registers for the division.

### Stack Operations
- `PUSHREG { reg }`:
  - **Description**: Pushes the value from the specified register onto the stack.
  - **Parameters**:
    - `reg`: Register index whose value is to be pushed.
- `POPREG { reg }`:
  - **Description**: Pops the top value from the stack into the specified register.
  - **Parameters**:
    - `reg`: Register index where the popped value will be stored.

### Control Flow
- `JMP { address }` and related jump instructions (`JMPN`, `JMPP`, `JMPZ`):
  - **Description**: Unconditionally or conditionally jumps to a specified memory address based on flags or conditions.
  - **Parameters**:
    - `address`: Target memory address for the jump.
- `CALL { address }`:
  - **Description**: Calls a subroutine at the specified memory address, typically involving stack operations to save the return address.
  - **Parameters**:
    - `address`: Memory address of the subroutine.
- `RET`:
  - **Description**: Returns from a subroutine, typically involves retrieving the return address from the stack.
- `CLF`:
  - **Description**: Clears the CPU flags, resetting the state for fresh evaluations.
- `HLT`:
  - **Description**: Halts the machine, stopping execution.

## License

This project is licensed under the MIT License - see the 
This project is open-source and is licensed under the [MIT License](LICENSE). You are encouraged to use and modify the code for your own projects, but please provide proper attribution to the original authors.
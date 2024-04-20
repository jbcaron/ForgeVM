![ForgeVM logo](assets/logo.png)

# ForgeVM: _A virtual machine implemented in Rust_

Welcome to ForgeVM, a virtual machine (VM) implemented in Rust, designed to be robust, safe, and efficient. ForgeVM is an educational project that provides a platform for understanding the inner workings of a virtual machine, including its instruction set, memory management, and execution flow.


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


## License

This project is licensed under the MIT License - see the 
This project is open-source and is licensed under the [MIT License](LICENSE). You are encouraged to use and modify the code for your own projects, but please provide proper attribution to the original authors.
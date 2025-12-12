# C-- compiler

This is a work-in-progress project that aims to be implement a significant subset of the C17 programming language. The project delivers a CLI application that allows the user to turn C source code files into binary executables.

## Setup

The project is tested only on MacOS.

### System requirements
The compiler targets the x64 instruction set architecture (also known as x86-64 or amd64). You will need a MacOS system with an x64 processor, or one with Apple Silicon that can emulate x64..

#### Running on Apple Silicon
If your computer has an Apple Silicon processor, you can use Rosetta 2 to run the programs you compile. To open an x64 shell, run: `arch -x86_64 /bin/zsh`.

### Install Rust
The compiler is implemented in Rust. Install Rust using [rustup](https://rustup.rs/). After installation, run `cargo --version` to verify that the installation was successful.

### GCC/Clang
On MacOS, clang is the default C compiler. It is normally installed by default with Xcode. Make sure that clang is installed by running `clang --version`. In case the command was not found, you can install clang by running `xcode-select --install`.

You can also use gcc, but normally it defaults as an alias for clang.

### Compiling
To compile the project, run `cargo build` in the root directory of the project. It will create the compiler driver binary in `./target/debug/cmmc_driver`. More information on how to use the compiled C-- compiler from [TUTORIAL.md](./TUTORIAL.md).

### Testing
The project contains a comprehensive test suite. Execute tests by running `cargo test` in the project root.


## Architecture

This project replaces either the gcc or clang compiler drivers. The compiler driver is responsible for:
1. Preprocessing the C source code
2. Compiling the processed files into assembly code
3. Assembling the assembly code into object files
4. Linking the object files into a final executable

We use gcc's/clang's preprocessor, assembler, and linker, while we replace the compiler with our own custom C-- compiler.

![C-- compiler driver and compiler architecture diagram](./images/C--.png)

The custom C-- compiler turns preprocessed C source files into assembly with a 5-stage process:
1. Lexer: splits the source code into tokens
2. Parser: parses the sequence of tokens into a C-- abstract syntax tree (AST)
3. TACKY IR generation: converts the C-- AST into TACKY IR AST
4. Code generation: converts the TACKY IR AST into assembly AST
5. Code emission: writes the assembly code into a file
 

## Features
C, as a programming language, is known for its simplicity in language constructs and low-level access to physical hardware, allowing peak performance for optimized C programs. 

This compiler is currently in the early stages of development and only implements the most basic features:
- Execution of the main function
- Integer literals
- Unary operations (negation and complement)

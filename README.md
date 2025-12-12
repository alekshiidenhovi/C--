# C-- compiler

This is a work-in-progress project that aims to be implement a significant subset of the C17 programming language.

C, as a programming language, is known for its simplicity in language constructs and low-level access to physical hardware, allowing peak performance for optimized C programs. 

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
3. TACKY IR generation: converts the C-- AST into TACKY IR AST.
4. Code generation: converts the TACKY IR AST into assembly AST.
5. Code emission: writes the assembly code into a file


## Features
This compiler is currently in the early stages of development and only implements the most basic features:
- Execution of the main function
- Integer literals
- Unary operations (negation and complement)

# C-- compiler tutorial

This tutorial will guide you through the process of compiling a C source file into an executable binary file, and executing those binary files. It also showcases the driver and compiler options that can be used to stop the compilation process after a specific stage.

## Tutorial
This tutorial assumes that you have already completed the [Getting Started](./README.md#getting-started) section.

### Simplest example - constant integer
The simplest example consists of returning a constant integer value. After running `cargo build`, run the following command:
```bash
./target/debug/cmmc_driver programs/tutorial/return_2.c
```
The cmmc_driver (C-- Compiler Driver) executes the following steps when no flags are provided:
1. Preprocessing - invokes the clang preprocessor, which is used to expand macros and include files
2. Compilation - executes the custom C-- compiler, which is responsible for compiling the C source code into an executable binary file
3. Assembling - creates object files from the assembly code
4. Linking - links the object files into an executable binary file

The result of the compilation process is an executable binary file named `return_2`, you can find this in the same directory as the `return_2.c` file. Run the binary with `./programs/tutorial/return_2`.

Running the binary does not print anything to the console. But you can run `echo $?` to output the exit code of the program, which should be `2`.

### Compiler stages
The compilation step in the cmmc_driver can be further broken down into the following stages:
1. Lexing - splits the source code into tokens
2. Parsing - converts the tokens into an abstract syntax tree (AST)
3. TACKY IR generation - converts the AST into a TACKY IR
4. Code generation - converts the TACKY IR into x64 assembly
5. Code emission - emits the x64 assembly into an executable binary file

We can stop the compilation process at any stage by providing the corresponding flag to the compiler driver. The compiler stops the execution, and does not output any files as the result. Instead, it prettyprints the result of the compilation stage to the console.

This command outputs a vector of tokens to the console:
```bash
./target/debug/cmmc_driver programs/tutorial/return_2.c --lex
```

This command outputs the AST:
```bash
./target/debug/cmmc_driver programs/tutorial/return_2.c --parse
```

This command outputs the TACKY IR:
```bash
./target/debug/cmmc_driver programs/tutorial/return_2.c --tacky
```

This command outputs the x86-64 assembly:
```bash
./target/debug/cmmc_driver programs/tutorial/return_2.c --codegen
```

As was mentioned earlier, the compiler driver does not output any files. If we want to output the assembly code to a file, and inspect its contents, we can pass the `-S` flag.

```bash
./target/debug/cmmc_driver programs/tutorial/return_2.c -S
```
This command will store the assembly code in the file `return_2.s`. We can inspect the contents of the file by running `cat return_2.s`. The output should look like the following:

```asm
        .globl _main
_main:
        pushq %rbp
        movq %rsp, %rbp
        subq $0, %rsp
        movl $2, %eax
        movq %rbp, %rsp
        popq %rbp
        ret
```

This assembly code is valid x86-64 assembly that you can run natively on Intel machine or emulate on Apple Silicon! Pretty cool, right?

### Unary operations

The compiler supports unary operations for negation and complement.

#### Negation

The file `negate_2.c` simply returns the negative number 2. Compile the program by running the following:
```bash
./target/debug/cmmc_driver programs/tutorial/negate_2.c
```
Then as before, run the binary with `./programs/tutorial/negate_2`, print the exit code with `echo $?`, which should output number 254. This is correct behavior, result of [two's complement representation](https://en.wikipedia.org/wiki/Two%27s_complement) where positive and negative values are expressed with a single representation.

#### Complement
The file `complement_2.c` returns the binary complement of the number 2. Following the same order of execution, the exit code should to be printed should be 253 for this program.

#### Nested unary operations
You can nest unary operations. Compiling and executing the file `nested_unary.c` should output 255 as the exit code.

Note that while you can use multiple complement operators in a row ("~~" is valid), multiple hyphen operators ("--") are tokenized as a unique double hyphen tokens, which will break the tokenization process. If you attempt to compile `nested_negation_broken.c`, this will cause a parser error, caused by an unxpected double hyphen token.
```bash
./target/debug/cmmc_driver ./programs/tutorial/nested_negation_broken.c
Invoking GCC Preprocessor...
Preprocessed file created at: ./programs/tutorial/nested_negation_broken.i
Compiling with a custom C compiler...
Error: Parser error: Unexpected token DoubleHyphen, expected one of [Constant, H
yphen, Tilde, OpenParen]
```

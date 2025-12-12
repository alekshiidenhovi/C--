# C-- compiler tutorial

This tutorial will guide you through the process of compiling a C source file into an executable binary file. It also showcases the driver and compiler options that can be used to stop the compilation process after a specific stage.

## Tutorial
This tutorial assumes that you have already completed the [Getting Started](./README.md#getting-started) section.

### Full compilation
To compile your C source file into an executable binary file, run the following command in the root directory of the project:

```bash
./target/debug/cmmc_driver programs/tutorial/return_2.c
```

This command will compile the `return_2.c` file into an executable binary file named `return_2`. You can run the binary with `./programs/tutorial/return_2`. After the program has been executed, run `echo $?` to output the exit code of the program. If the program executed correctly, number 2 should be printed to the console.

### Early stopping - compiler
The compiler driver allows stopping the compilation process early by providing flags to the compiler.

#### Lexer

```
./target/debug/cmmc_driver programs/tutorial/return_2.c --lex
```

#### Parser
```
./target/debug/cmmc_driver programs/tutorial/return_2.c --parse
```

#### TACKY IR generation
```
./target/debug/cmmc_driver programs/tutorial/return_2.c --tacky
```

#### Code generation
```
./target/debug/cmmc_driver programs/tutorial/return_2.c --codegen
```

### Early stopping - assembly


## Exercises

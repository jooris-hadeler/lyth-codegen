# Lyth Codegen
This is an experimental repository for a code generator for the Lyth programming language.

Instead of dealing with `rust-llvm` bindings I wanted to try to generate machine code myself.

## How to build
```bash
cargo run # compile the program and run it
ndisams -b64 test.o # disassemble the generated code for debugging
```
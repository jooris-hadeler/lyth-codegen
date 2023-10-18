use std::path::Path;
use crate::assembler::{Operand, Registers};

mod assembler;

use crate::assembler::Assembler;

fn main() {
    let mut asm = Assembler::new();

    // asm.enter(0);
    // asm.leave();
    // asm.ret();

    // asm.mov(Registers::Rax, Registers::Rcx);
    // asm.mov(Registers::Rax, Registers::R8);
    // asm.mov(Registers::Rax, 0xbeefu64);
    // asm.mov(Registers::Rax, Operand::memory_and_offset(Registers::Rcx, 0u32));
    // asm.mov(Registers::Rax, Operand::memory_and_offset(Registers::Rcx, 0xbeefu32));
    // asm.mov(Registers::Rax, Operand::memory_and_offset(Registers::R9, 0xbeefu32));
    //
    // asm.mov(Registers::R8, Registers::Rcx);
    // asm.mov(Registers::R8, Registers::R9);
    // asm.mov(Registers::R8, 0xbeefu64);
    // asm.mov(Registers::R8, Operand::memory_and_offset(Registers::Rcx, 0xbeefu32));
    // asm.mov(Registers::R8, Operand::memory_and_offset(Registers::R9, 0xbeefu32));
    //
    // asm.mov(Operand::memory_and_offset(Registers::Rax, 0xbeefu32), Registers::Rcx);
    // asm.mov(Operand::memory_and_offset(Registers::R8, 0xbeefu32), Registers::Rcx);
    // asm.mov(Operand::memory_and_offset(Registers::Rax, 0xbeefu32), Registers::R9);
    // asm.mov(Operand::memory_and_offset(Registers::R8, 0xbeefu32), Registers::R9);

    // asm.push(Registers::R8);
    // asm.push(Registers::Rcx);
    // asm.push(0xbeefu32);
    //
    // asm.pop(Registers::Rax);
    // asm.pop(Registers::R9);

    // asm.add(Registers::Rax, Registers::Rcx);
    // asm.add(Registers::R8, Registers::Rcx);
    //
    // asm.add(Registers::Rax, Registers::R8);
    // asm.add(Registers::R9, Registers::R8);
    //
    // asm.add(Registers::Rax, 0xbeefu32);
    // asm.add(Registers::Rax, Operand::memory_and_offset(Registers::Rax, 0xbeefu32));
    // asm.add(Registers::Rax, Operand::memory_and_offset(Registers::R8, 0xbeefu32));
    //
    // asm.add(Registers::R9, 0xbeefu32);
    // asm.add(Registers::R11, Operand::memory_and_offset(Registers::Rax, 0xbeefu32));
    // asm.add(Registers::R11, Operand::memory_and_offset(Registers::R8, 0xbeefu32));

    asm.xor(Registers::Rax, Registers::Rcx);
    asm.xor(Registers::Rax, Registers::R8);
    asm.xor(Registers::R8, Registers::Rax);
    asm.xor(Registers::R8, Registers::R9);

    asm.xor(Registers::Rax, Operand::memory_and_offset(Registers::Rcx, 0xbeefu32));
    asm.xor(Registers::Rax, Operand::memory_and_offset(Registers::R8, 0xbeefu32));
    asm.xor(Registers::R9, Operand::memory_and_offset(Registers::Rcx, 0xbeefu32));
    asm.xor(Registers::R11, Operand::memory_and_offset(Registers::R8, 0xbeefu32));

    asm.xor(Operand::memory_and_offset(Registers::Rcx, 0xbeefu32), Registers::Rax);
    asm.xor(Operand::memory_and_offset(Registers::R8, 0xbeefu32), Registers::Rax);
    asm.xor(Operand::memory_and_offset(Registers::Rcx, 0xbeefu32), Registers::R9);
    asm.xor(Operand::memory_and_offset(Registers::R8, 0xbeefu32), Registers::R11);

    asm.xor(Registers::Rax, 0xbeefu32);
    asm.xor(Registers::R11, 0xbeefu32);

    let code = asm.finalize();

    std::fs::write(Path::new("test.o"), code).unwrap();
}
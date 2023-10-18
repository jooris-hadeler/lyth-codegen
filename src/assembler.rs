pub type RegisterId = u8;

macro_rules! decide {
    ($cond:expr, $then:expr, $alt:expr) => {
        if $cond {
            $then
        } else {
            $alt
        }
    };
}

/// This enum represents the different registers that can be used in the assembler.
#[repr(u8)]
pub enum Registers {
    Rax,
    Rcx,
    Rdx,
    Rbx,
    Rsp,
    Rbp,
    Rsi,
    Rdi,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

/// This trait is used to convert a `Registers` enum to a `RegisterId`.
impl From<Registers> for RegisterId {
    fn from(reg: Registers) -> Self {
        reg as RegisterId
    }
}

/// This enum represents the different operands that can be used in the assembler.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operand {
    /// A register operand.
    Register(RegisterId),
    /// A memory operand with an offset.
    MemoryAndOffset(RegisterId, u32),
    /// A 64-bit immediate operand.
    Imm64(u64),
    /// A 32-bit immediate operand.
    Imm32(u32),
    /// A 8-bit immediate operand.
    Imm8(u8),
}

impl Operand {
    /// This function creates a new register operand.
    pub fn register<S: Into<RegisterId>>(reg: S) -> Self {
        Operand::Register(reg.into())
    }

    /// This function creates a new memory operand with an offset.
    pub fn memory_and_offset<S: Into<RegisterId>>(reg: S, offset: u32) -> Self {
        Operand::MemoryAndOffset(reg.into(), offset)
    }

    /// This function creates a new 64-bit immediate operand.
    pub fn imm64<S: Into<u64>>(imm64: S) -> Self {
        Operand::Imm64(imm64.into())
    }

    /// This function creates a new 32-bit immediate operand.
    pub fn imm32<S: Into<u32>>(imm32: S) -> Self {
        Operand::Imm32(imm32.into())
    }
}

/// The assembler is a helper class to generate x86_64 machine code.
pub struct Assembler {
    /// The generated code.
    pub code: Vec<u8>,
}

impl Assembler {
    /// This function creates a new assembler.
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    /// This function consumes self and returns the generated code.
    pub fn finalize(self) -> Box<[u8]> {
        self.code.into_boxed_slice()
    }

    /// This function generates a move instruction from the source to the destination.
    ///
    /// Params:
    ///  - `dst`: the destination operand
    ///  - `src`: the source operand
    pub fn mov<A: Into<Operand>, B: Into<Operand>>(&mut self, dst: A, src: B) {
        let dst = dst.into();
        let src = src.into();

        if dst == src {
            return;
        }

        match dst {
            Operand::Register(dst_reg) => match src {
                // mov dst, src
                Operand::Register(src_reg) => {
                    self.emit_rex_prefix(dst_reg, src_reg);

                    self.emit8(0x89);

                    self.emit8(0xC0
                        | ((src_reg & 0x7) << 3)
                        | (dst_reg & 0x7));
                }

                // mov dst, [src+offset]
                Operand::MemoryAndOffset(src_reg, src_offset) => {
                    self.emit_rex_prefix(src_reg, dst_reg);

                    self.emit8(0x8B);

                    self.emit8(0x80
                        | ((dst_reg & 0x7) << 3)
                        | (src_reg & 0x7));

                    self.emit32(src_offset);
                }

                // mov dst, imm64
                Operand::Imm64(imm64) => {
                    self.emit_rex_prefix(dst_reg, 0);

                    self.emit8(0xB8 | ((dst_reg & 0x7) << 3));

                    self.emit64(imm64);
                }

                op => panic!("Invalid source: {:?}", op)
            },
            Operand::MemoryAndOffset(dst_reg, dst_offset) => match src {
                // mov [dst+offset], src
                Operand::Register(src_reg) => {
                    self.emit_rex_prefix(dst_reg, src_reg);

                    self.emit8(0x89);

                    self.emit8(0x80
                        | (src_reg & 7) << 3
                        | (dst_reg & 7));

                    self.emit32(dst_offset);
                }

                Operand::Imm64(..) | Operand::Imm32(..) => panic!("impossible to move an immediate to memory"),
                Operand::MemoryAndOffset(..) => panic!("impossible to move from memory to memory"),

                op => panic!("Invalid source: {:?}", op)
            },

            op => panic!("Invalid destination: {:?}", op)
        }
    }

    /// This function generates an add instruction that adds the source to the destination.
    ///
    /// Params:
    ///  - `dst`: the destination operand
    ///  - `src`: the source operand
    pub fn add<A: Into<Operand>, B: Into<Operand>>(&mut self, dst: A, src: B) {
        let dst = dst.into();
        let src = src.into();

        match dst {
            Operand::Register(dst_reg) => match src {
                // add dst, src
                Operand::Register(src_reg) => {
                    self.emit_rex_prefix(dst_reg, src_reg);

                    self.emit8(0x01);

                    self.emit8(0xC0
                        | ((src_reg & 0x7) << 3)
                        | (dst_reg & 0x7));
                }

                // add dst, [src+offset]
                Operand::MemoryAndOffset(src_reg, src_offset) => {
                    self.emit_rex_prefix(src_reg, dst_reg);

                    self.emit8(0x03);

                    self.emit8(0x80
                        | ((dst_reg & 0x7) << 3)
                        | (src_reg & 0x7));

                    self.emit32(src_offset);
                }

                // add dst, imm32
                Operand::Imm32(imm32) => {
                    self.emit_rex_prefix(dst_reg, 0);

                    self.emit8(0x81);

                    self.emit8(0xC0 | (dst_reg & 0x7));

                    self.emit32(imm32);
                }

                op => panic!("Invalid source: {:?}", op)
            },

            op => panic!("Invalid destination: {:?}", op)
        }
    }

    /// This function generates a sub instruction that subtracts the source from the destination.
    ///
    /// Params:
    ///  - `dst`: the destination operand
    ///  - `src`: the source operand
    pub fn sub<A: Into<Operand>, B: Into<Operand>>(&mut self, dst: A, src: B) {
        let dst = dst.into();
        let src = src.into();

        match dst {
            Operand::Register(dst_reg) => match src {
                // sub dst, src
                Operand::Register(src_reg) => {
                    self.emit_rex_prefix(dst_reg, src_reg);

                    self.emit8(0x29);

                    self.emit8(0xC0
                        | ((src_reg & 0x7) << 3)
                        | (dst_reg & 0x7));
                }

                // sub dst, [src+offset]
                Operand::MemoryAndOffset(src_reg, src_offset) => {
                    self.emit_rex_prefix(src_reg, dst_reg);

                    self.emit8(0x2B);

                    self.emit8(0x80
                        | ((dst_reg & 0x7) << 3)
                        | (src_reg & 0x7));

                    self.emit32(src_offset);
                }

                // sub dst, imm32
                Operand::Imm32(imm32) => {
                    self.emit_rex_prefix(dst_reg, 0);

                    self.emit8(0x81);

                    self.emit8(0xE8 | (dst_reg & 0x7));

                    self.emit32(imm32);
                }

                op => panic!("Invalid source: {:?}", op)
            },

            op => panic!("Invalid destination: {:?}", op)
        }
    }

    /// This function generates a near jump instruction and returns the offset to the jump destination.
    ///
    /// Params:
    ///  - `dst`: the destination operand
    pub fn jump_near<A: Into<Operand>>(&mut self, dst: A) -> usize {
        let dst = dst.into();

        match dst {
            Operand::Imm8(imm8) => {
                self.emit8(0xEB);

                let pos = self.code.len();

                self.emit8(imm8);

                return pos;
            }

            Operand::Imm32(imm32) => {
                self.emit8(0xE9);

                let pos = self.code.len();

                self.emit32(imm32);

                return pos;
            }

            op => panic!("Invalid destination: {:?}", op)
        }
    }

    /// This function is used to emit a enter instruction.
    /// In this case we just do what enter does manually, that way
    /// we have more control over the alignment of the stack.
    ///
    /// Params:
    /// - `stack_size`: the stack size, this will be aligned to 16 bytes
    pub fn enter(&mut self, stack_size: u32) {
        self.push(Registers::Rbp);
        self.mov(Registers::Rbp, Registers::Rsp);

        if stack_size > 0 {
            // align stack_size to 16 bytes
            let stack_size = (stack_size + 15) & !15;

            self.sub(Registers::Rsp, stack_size);
        }
    }

    /// This function is used to emit a leave instruction.
    pub fn leave(&mut self) {
        self.emit8(0xC9);
    }

    /// This function is used to patch a jump instruction.
    ///
    /// Params:
    ///  - `value`: the value to patch
    ///  - `offset`: the offset to patch
    pub fn patch32(&mut self, value: u32, offset: usize) {
        self.code[offset + 0] = ((value >> 0) & 0xFF) as u8;
        self.code[offset + 1] = ((value >> 8) & 0xFF) as u8;
        self.code[offset + 2] = ((value >> 16) & 0xFF) as u8;
        self.code[offset + 3] = ((value >> 24) & 0xFF) as u8;
    }

    /// This function generates a return instruction.
    pub fn ret(&mut self) {
        self.emit8(0xC3);
    }

    /// This function generates a push instruction.
    ///
    /// Currently only supports pushing a register or an immediate.
    ///
    /// Params:
    ///  - `src`: the src operand
    pub fn push<S: Into<Operand>>(&mut self, src: S) {
        match src.into() {
            Operand::Register(reg) => {
                self.emit_rex_prefix(reg, 0);

                self.emit8(0x50 | (reg & 0x7));
            }

            Operand::Imm32(imm32) => {
                // FIXME: this currently produces a push qword imm32 which is not what we want
                self.emit8(0x68);
                self.emit32(imm32);
            }

            op => panic!("impossible tp push {:?}", op),
        }
    }

    /// This function generates a pop instruction.
    ///
    /// Currently only supports popping to a register.
    ///
    /// Params:
    ///  - `dst`: the destination operand
    pub fn pop<S: Into<Operand>>(&mut self, dst: S) {
        let dst = dst.into();

        match dst {
            Operand::Register(reg) => {
                self.emit_rex_prefix(reg, 0);

                self.emit8(0x58 | (reg & 0x7));
            }

            op => panic!("impossible to pop {:?}", op),
        }
    }

    /// This is a helper function that adds a byte to the code.
    /// Params:
    /// - `byte`: the byte to add
    fn emit8(&mut self, byte: u8) {
        self.code.push(byte);
    }

    /// This is a helper function that adds a dword to the code.
    /// Params:
    /// - `dword`: the dword to add
    fn emit32(&mut self, dword: u32) {
        self.code.push(((dword >> 0) & 0xFF) as u8);
        self.code.push(((dword >> 8) & 0xFF) as u8);
        self.code.push(((dword >> 16) & 0xFF) as u8);
        self.code.push(((dword >> 24) & 0xFF) as u8);
    }

    /// This is a helper function that adds a qword to the code.
    /// Params:
    /// - `dword`: the dword to add
    fn emit64(&mut self, dword: u64) {
        self.code.push(((dword >> 0) & 0xFF) as u8);
        self.code.push(((dword >> 8) & 0xFF) as u8);
        self.code.push(((dword >> 16) & 0xFF) as u8);
        self.code.push(((dword >> 24) & 0xFF) as u8);
        self.code.push(((dword >> 32) & 0xFF) as u8);
        self.code.push(((dword >> 40) & 0xFF) as u8);
        self.code.push(((dword >> 48) & 0xFF) as u8);
        self.code.push(((dword >> 56) & 0xFF) as u8);
    }

    /// This is a helper function that emits a REX prefix if necessary.
    ///
    /// The REX prefix is of the form `0b0100WRXB`, where:
    /// - `W` is the 64-bit operand size bit, this is always 1 in our case
    /// - `R` is the extension of the ModR/M `reg` field
    /// - `X` is the extension of the SIB `index` field, this is always 0 in our case
    /// - `B` is the extension of the ModR/M `r/m` field or the SIB `base` field
    ///
    /// Params:
    /// - `src`: the source operand
    /// - `dst`: the destination operand
    fn emit_rex_prefix(&mut self, reg1: RegisterId, reg2: RegisterId) {
        self.emit8(0x48
            | decide!(reg2 >= 8, 1 << 2, 0)
            | decide!(reg1 >= 8, 1 << 0, 0));
    }
}

impl Into<Operand> for Registers {
    fn into(self) -> Operand {
        Operand::Register(self as RegisterId)
    }
}

impl Into<Operand> for u64 {
    fn into(self) -> Operand {
        Operand::Imm64(self)
    }
}

impl Into<Operand> for u32 {
    fn into(self) -> Operand {
        Operand::Imm32(self)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instruction {
    word: u32,
    vram: u32,

    flags: InstructionFlags,
}

impl Instruction {
    #[must_use]
    pub const fn new(
        word: u32,
        vram: u32,
        flags: InstructionFlags,
    ) -> Self {
        Self {
            word,
            vram,
            flags,
        }
    }

    #[must_use]
    pub const fn new_r4000allegrex(word: u32, vram: u32, flags: InstructionFlags) -> Self {
        Self::new(
            word,
            vram,
            flags,
        )
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstructionFlags {
}

impl InstructionFlags {
    pub const fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayFlags {
}

impl DisplayFlags {
    pub const fn default() -> Self {
        Self {}
    }
}

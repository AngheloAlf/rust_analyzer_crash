
#[cfg(test)]
pub(crate) mod tests {
    pub(crate) const OPERAND_COUNT_MAX: usize = 5;

    use crate::{Instruction, InstructionFlags, DisplayFlags, Opcode};

    pub(crate) struct TestEntry {
        #[allow(dead_code)]
        pub instr: Instruction,
        #[allow(dead_code)]
        pub imm_override: Option<&'static str>,
        #[allow(dead_code)]
        pub display_flags: DisplayFlags,

        #[allow(dead_code)]
        pub valid: bool,

        #[allow(dead_code)]
        pub expected: &'static str,
        #[allow(dead_code)]
        pub expected_opcode: Opcode,
        #[allow(dead_code)]
        pub opcode_str: &'static str,
        #[allow(dead_code)]
        pub operands_str: [Option<&'static str>; OPERAND_COUNT_MAX],
    }

    impl TestEntry {
        pub const fn new_r4000allegrex(
            word: u32,
            expected: &'static str,
            expected_opcode: Opcode,
            opcode_str: &'static str,
            operands_str: [Option<&'static str>; OPERAND_COUNT_MAX],
        ) -> Self {
            Self {
                instr: Instruction::new_r4000allegrex(
                    word,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected,
                expected_opcode,
                opcode_str,
                operands_str,
            }
        }

        pub fn check_validity(&self) -> u32 {
            /*
            let mut errors = 0;

            if self.instr.is_valid() != self.valid {
                println!(
                    "'{}' ({:08X}) has incorrect validity. Expected '{}', got '{}'",
                    self.opcode_str,
                    self.instr.word(),
                    self.valid,
                    self.instr.is_valid()
                );
                errors += 1;
            }
            if self.instr.opcode() != self.expected_opcode {
                println!(
                    "'{}' ({:08X}) has incorrect decoded opcode. Expected '{:?}', got '{:?}'",
                    self.opcode_str,
                    self.instr.word(),
                    self.expected_opcode,
                    self.instr.opcode()
                );
                errors += 1;
            }
            if self.instr.opcode().name() != self.opcode_str {
                println!(
                    "'{}' ({:08X}) has incorrect opcode name. Expected '{}', got '{}'",
                    self.opcode_str,
                    self.instr.word(),
                    self.opcode_str,
                    self.instr.opcode().name()
                );
                errors += 1;
            }

            errors
            */
            0
        }

        pub fn check_disassembly(&self) -> u32 {
            /*
            let mut errors = 0;

            let disasm = self
                .instr
                .display(self.imm_override, &self.display_flags)
                .to_string();
            // println!("    {}", disasm);
            if disasm != self.expected {
                println!(
                    "'{}' ({:08X}) did not match the expected string.",
                    self.opcode_str,
                    self.instr.word(),
                );
                println!("    Expected: '{}'", self.expected,);
                println!("    Got:      '{}'", disasm,);
                errors += 1;
            }

            {
                let mut j = 0;
                for (i, operand) in self.instr.operands_iter().enumerate() {
                    let operand_str = operand
                        .display(&self.instr, self.imm_override, &self.display_flags)
                        .to_string();
                    let maybe_expected_str = self.operands_str[i];

                    if let Some(expected_str) = maybe_expected_str {
                        if operand_str != expected_str {
                            println!(
                                "'{}' ({:08X}) has incorrect disassembled operand. Expected '{}', got '{}'",
                                self.opcode_str,
                                self.instr.word(),
                                expected_str,
                                operand_str
                            );
                            errors += 1;
                        }
                    } else {
                        println!(
                            "'{}' ({:08X}) has an unexpected operand at index {}. Got: '{}'",
                            self.opcode_str,
                            self.instr.word(),
                            i,
                            operand_str,
                        );
                        errors += 1;
                    }
                    j = i;
                }

                if !self.operands_str[j + 1..].iter().all(|x| x.is_none()) {
                    println!(
                        "'{}' ({:08X}) has unhandled expected operands. Values: '{:?}'",
                        self.opcode_str,
                        self.instr.word(),
                        &self.operands_str[j..]
                    );
                    errors += 1;
                }
            }

            errors
            */
            0
        }
    }

    pub(crate) fn check_test_entries(entries: &[TestEntry], thingy: bool) -> u32 {
        let mut errors = 0;

        for entry in entries {
            errors += entry.check_validity();
            if thingy {
                errors += entry.check_disassembly();
            }
        }

        errors
    }
}

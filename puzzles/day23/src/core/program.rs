use super::{Instruction, Offset, Registers};

#[derive(Debug, Clone)]
pub struct Program<'a> {
    instructions: &'a [Instruction],
    registers: Registers,
    location: usize,
}

impl<'a> Program<'a> {
    pub fn with_registers(instructions: &'a [Instruction], registers: Registers) -> Self {
        Self {
            instructions,
            registers,
            location: 0,
        }
    }

    fn jump(&mut self, offset: Offset) {
        self.location = self
            .location
            .checked_add_signed(offset)
            .unwrap_or(self.instructions.len());
    }
}

impl Iterator for Program<'_> {
    type Item = Registers;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = self.instructions.get(self.location)?;

        match instruction {
            Instruction::Half(i) => {
                self.registers[*i] /= 2;
                self.location += 1;
            }
            Instruction::Triple(i) => {
                self.registers[*i] *= 3;
                self.location += 1;
            }
            Instruction::Increment(i) => {
                self.registers[*i] += 1;
                self.location += 1;
            }
            Instruction::Jump(offset) => {
                self.jump(*offset);
            }
            Instruction::JumpIfEven(i, offset) => {
                if self.registers[*i] % 2 == 0 {
                    self.jump(*offset);
                } else {
                    self.location += 1;
                }
            }
            Instruction::JumpIfOne(i, offset) => {
                if self.registers[*i] == 1 {
                    self.jump(*offset);
                } else {
                    self.location += 1;
                }
            }
        }

        Some(self.registers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut program = Program::new(&[
            Instruction::Increment(0),
            Instruction::JumpIfOne(0, 2),
            Instruction::Triple(0),
            Instruction::Increment(0),
        ]);

        assert_eq!(program.next(), Some([1, 0]));
        assert_eq!(program.next(), Some([1, 0]));
        assert_eq!(program.next(), Some([2, 0]));
        assert_eq!(program.next(), None);
    }
}

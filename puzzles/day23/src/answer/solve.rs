use anyhow::Context;

use crate::core::{Instruction, Program, RegisterValue, Registers};

use super::{Parsed1, Parsed2};

type Solution = RegisterValue;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn solve(instructions: &[Instruction], registers: Registers) -> Option<RegisterValue> {
    Program::with_registers(instructions, registers)
        .last()
        .map(|[_, b]| b)
}

pub fn solve1(instructions: &Parsed1) -> anyhow::Result<Solution1> {
    solve(instructions, [0, 0]).context("no instructions")
}

pub fn solve2(instructions: &Parsed2) -> anyhow::Result<Solution2> {
    solve(instructions, [1, 0]).context("no instructions")
}

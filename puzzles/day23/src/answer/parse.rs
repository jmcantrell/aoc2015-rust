use anyhow::Context;

use aoc::Input;

use crate::core::Instruction;

type Parsed = Vec<Instruction>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    input
        .lines()
        .enumerate()
        .map(|(i, s)| {
            s.try_into()
                .with_context(|| format!("line number {}", i + 1))
        })
        .collect()
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    parse(input)
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    parse(input)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    const INPUT: Input = include_str!("../../input.txt");

    #[test]
    fn parse() -> anyhow::Result<()> {
        dbg!(super::parse(INPUT)?);
        Ok(())
    }
}

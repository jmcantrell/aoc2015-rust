use anyhow::{ensure, Context};

use aoc::Input;

use crate::core::Weight;

type Parsed = Vec<Weight>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_weight(s: &str) -> anyhow::Result<Weight> {
        let weight = s.parse()?;

        ensure!(weight > 0, "weight must be non-zero: {:?}", weight);

        Ok(weight)
    }

    input
        .lines()
        .enumerate()
        .map(|(i, s)| parse_weight(s).with_context(|| format!("line number {}", i + 1)))
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

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn parse() -> anyhow::Result<()> {
        dbg!(super::parse(INPUT)?);
        Ok(())
    }
}

use aoc::Input;

use anyhow::Context;

use crate::core::{Circuit, Source};

type Parsed = Circuit;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_part(s: &str) -> anyhow::Result<(String, Source)> {
        let (left, right) = s.split_once("->").context("missing connector")?;
        let source = Source::try_from(left.trim())?;
        Ok((right.trim().to_owned(), source))
    }

    input
        .lines()
        .enumerate()
        .map(|(i, s)| parse_part(s).context(format!("part number {}", i + 1)))
        .collect::<Result<Circuit, _>>()
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    parse(input)
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    parse(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() -> anyhow::Result<()> {
        let input = concat!(
            "123 -> x\n",
            "456 -> y\n",
            "x AND y -> d\n",
            "x OR y -> e\n",
            "x LSHIFT 2 -> f\n",
            "y RSHIFT 2 -> g\n",
            "NOT x -> h\n",
            "NOT y -> i\n",
        );

        dbg!(super::parse(input)?);

        Ok(())
    }
}

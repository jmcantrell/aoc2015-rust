use std::collections::HashMap;

use anyhow::Context;

use aoc::Input;

use crate::core::Profile;

type Parsed = HashMap<&'static str, Profile>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_ingredient(s: &str) -> anyhow::Result<(&str, Profile)> {
        let (name, s) = s
            .split_once(':')
            .context("expected the input to start with '<name>:'")?;

        let profile = s.try_into()?;

        Ok((name, profile))
    }

    input
        .lines()
        .enumerate()
        .map(|(i, s)| parse_ingredient(s).with_context(|| format!("line number {}", i + 1)))
        .collect::<Result<HashMap<_, _>, _>>()
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

    use super::*;

    const INPUT: Input = include_str!("../../input-test");

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        dbg!(parse(INPUT)?);
        Ok(())
    }
}

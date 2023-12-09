use anyhow::Context;

use aoc::Input;

use crate::core::{Distance, Route};

type Parsed = Vec<Route>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_route(s: Input) -> anyhow::Result<Route> {
        let (location1, s) = s
            .split_once(" to ")
            .context("expected locations to be delimited by ' to '")?;

        let (location2, s) = s
            .split_once(" = ")
            .context("expected location pair to be delimited from distance by ' = '")?;

        let distance: Distance = s.trim().parse().context("unable to parse distance")?;

        Ok((location1.trim(), location2.trim(), distance))
    }

    input
        .lines()
        .enumerate()
        .map(|(i, s)| parse_route(s).with_context(|| format!("route number {}", i + 1)))
        .collect::<Result<Vec<_>, _>>()
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

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        dbg!(parse(INPUT)?);
        Ok(())
    }
}

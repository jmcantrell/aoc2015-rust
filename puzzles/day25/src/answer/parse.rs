use anyhow::Context;
use lazy_regex::{lazy_regex, Lazy, Regex};

use aoc::Input;

use crate::core::Location;

pub type Parsed = Location;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    static RE: Lazy<Regex> =
        lazy_regex!(r"\bEnter the code at row (?<row>\d+), column (?<column>\d+)\.");

    let captures = RE
        .captures(input)
        .with_context(|| format!("expected input to match: {:?}", RE.as_str()))?;

    let row = captures["row"].parse()?;
    let column = captures["column"].parse()?;

    Ok((row, column))
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use super::*;

    const INPUT: Input = include_str!("../../input");

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        dbg!(parse(INPUT)?);
        Ok(())
    }
}

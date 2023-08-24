use anyhow::{ensure, Context};

use aoc::Input;

use crate::core::Profile;

type Parsed = Vec<Profile>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    static PREFIX_TAG: &str = "Sue";

    fn parse_sue(i: usize, s: &str) -> anyhow::Result<Profile> {
        let (prefix, s) = s.split_once(':').context("missing prefix")?;

        let index = prefix
            .strip_prefix(PREFIX_TAG)
            .context(format!("expected prefix to start with {:?}", PREFIX_TAG))?;

        let index: usize = index.trim().parse()?;

        ensure!(
            index - 1 == i,
            "expected prefix index to be {:?}, but it was {:?}",
            i + 1,
            index
        );

        let sue = s.try_into()?;

        Ok(sue)
    }

    input
        .lines()
        .enumerate()
        .map(|(i, s)| parse_sue(i, s).context(format!("line number {}", i + 1)))
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

    const INPUT: Input = include_str!("../../input.txt");

    #[test]
    fn parse() -> anyhow::Result<()> {
        dbg!(super::parse(INPUT)?);
        Ok(())
    }
}

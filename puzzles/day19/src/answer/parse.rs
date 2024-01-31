use anyhow::Context;

use aoc::Input;

type Parsed = (Vec<(&'static str, &'static str)>, &'static str);
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    let (top, bottom) = input
        .split_once("\n\n")
        .context("expected sections to be separated by an empty line")?;

    fn parse_pair(s: &str) -> anyhow::Result<(&str, &str)> {
        let (left, right) = s
            .split_once("=>")
            .context("expected pair to be separated by a fat arrow")?;

        Ok((left.trim(), right.trim()))
    }

    let pairs = top
        .lines()
        .enumerate()
        .map(|(i, s)| parse_pair(s).with_context(|| format!("pair number {}", i + 1)))
        .collect::<Result<Vec<_>, _>>()?;

    let medicine = bottom.trim();

    Ok((pairs, medicine))
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

    const INPUT1A: Input = include_str!("../../input-test1a");
    const INPUT1B: Input = include_str!("../../input-test1b");

    const INPUT2A: Input = include_str!("../../input-test2a");
    const INPUT2B: Input = include_str!("../../input-test2b");

    #[test]
    fn test_parse1() -> anyhow::Result<()> {
        dbg!(parse1(INPUT1A)?);
        dbg!(parse1(INPUT1B)?);
        Ok(())
    }

    #[test]
    fn test_parse2() -> anyhow::Result<()> {
        dbg!(parse2(INPUT2A)?);
        dbg!(parse2(INPUT2B)?);
        Ok(())
    }
}

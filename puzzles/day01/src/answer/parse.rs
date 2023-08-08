use anyhow::Context;

use aoc::Input;

use crate::core::Direction;

type Parsed = Vec<Direction>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    input
        .chars()
        .enumerate()
        .map(|(i, c)| c.try_into().context(format!("direction number {}", i + 1)))
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
    use super::Direction::*;

    #[test]
    fn parse1() -> anyhow::Result<()> {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(super::parse1($input)?, Vec::from($expected));
            };
        }

        test!("(())", [Up, Up, Down, Down]);
        test!("()()", [Up, Down, Up, Down]);
        test!("(((", [Up, Up, Up]);
        test!("(()(()(", [Up, Up, Down, Up, Up, Down, Up]);
        test!("))(((((", [Down, Down, Up, Up, Up, Up, Up]);
        test!("())", [Up, Down, Down]);
        test!("))(", [Down, Down, Up]);
        test!(")))", [Down, Down, Down]);
        test!(")())())", [Down, Up, Down, Down, Up, Down, Down]);

        Ok(())
    }

    #[test]
    fn parse2() -> anyhow::Result<()> {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(super::parse2($input)?, Vec::from($expected));
            };
        }

        test!(")", [Down]);
        test!("()())", [Up, Down, Up, Down, Down]);

        Ok(())
    }
}

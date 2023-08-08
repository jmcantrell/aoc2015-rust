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
    use crate::core::Direction::*;

    #[test]
    fn parse() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(super::parse($input).unwrap(), Vec::from($expected));
            };
        }

        test!("", []);
        test!(">", [East]);
        test!("^>v<", [North, East, South, West]);

        assert!(super::parse("^>x<v").is_err());
    }
}

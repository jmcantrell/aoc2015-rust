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
        .map(|(i, c)| {
            c.try_into()
                .with_context(|| format!("direction number {}", i + 1))
        })
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
    use crate::core::Direction;

    use super::*;

    use Direction::*;

    #[test]
    fn test_parse() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(parse($input).unwrap(), Vec::from($expected));
            };
        }

        test!(")", [Down]);
        test!("()())", [Up, Down, Up, Down, Down]);
        test!("(())", [Up, Up, Down, Down]);
        test!("()()", [Up, Down, Up, Down]);
        test!("(((", [Up, Up, Up]);
        test!("(()(()(", [Up, Up, Down, Up, Up, Down, Up]);
        test!("))(((((", [Down, Down, Up, Up, Up, Up, Up]);
        test!("())", [Up, Down, Down]);
        test!("))(", [Down, Down, Up]);
        test!(")))", [Down, Down, Down]);
        test!(")())())", [Down, Up, Down, Down, Up, Down, Down]);
    }
}

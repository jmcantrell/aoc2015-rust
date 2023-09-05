use anyhow::Context;

use aoc::Input;

use crate::core::Command;

type Parsed = Vec<Command>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    input
        .lines()
        .enumerate()
        .map(|(i, s)| {
            s.try_into()
                .with_context(|| format!("command number {}", i + 1))
        })
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
    use crate::core::{Action, Command, Rectangle};

    use Action::*;

    #[test]
    fn parse() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    super::parse($input).unwrap(),
                    Vec::from($expected.map(|(action, top_left, bottom_right)| {
                        Command::new(action, Rectangle::new(top_left.into(), bottom_right.into()))
                    }))
                );
            };
        }

        let input = concat!(
            "turn on 0,1 through 2,3\n",
            "turn off 1,2 through 3,4\n",
            "toggle 2,3 through 4,5\n",
        );

        test!(
            input,
            [
                (TurnOn, [1, 0], [3, 2]),
                (TurnOff, [2, 1], [4, 3]),
                (Toggle, [3, 2], [5, 4])
            ]
        );
    }
}

use std::convert::TryFrom;

use anyhow::Context;

use super::Direction;

pub type Floor = isize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instructions(Vec<Direction>);

impl Instructions {
    pub fn iter(&self) -> impl Iterator<Item = Floor> + '_ {
        let mut floor = 0;
        self.0.iter().map(move |direction| {
            floor += match direction {
                Direction::Up => 1,
                Direction::Down => -1,
            };
            floor
        })
    }
}

impl From<Vec<Direction>> for Instructions {
    fn from(directions: Vec<Direction>) -> Self {
        Self(directions)
    }
}

impl TryFrom<&str> for Instructions {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let directions = s
            .chars()
            .enumerate()
            .map(|(i, c)| c.try_into().context(format!("direction number {}", i + 1)))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(directions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Direction::*;

    #[test]
    fn try_from() -> anyhow::Result<()> {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    Instructions::try_from($input)?,
                    Instructions::from(Vec::from($expected))
                );
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

        Ok(())
    }

    #[test]
    fn iter() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    Instructions::try_from($input)
                        .unwrap()
                        .iter()
                        .collect::<Vec<_>>(),
                    Vec::from($expected)
                );
            };
        }

        test!("(())", [1, 2, 1, 0]);
        test!("()()", [1, 0, 1, 0]);
        test!("(((", [1, 2, 3]);
        test!("(()(()(", [1, 2, 1, 2, 3, 2, 3]);
        test!("))(((((", [-1, -2, -1, 0, 1, 2, 3]);
        test!("())", [1, 0, -1]);
        test!("))(", [-1, -2, -1]);
        test!(")))", [-1, -2, -3]);
        test!(")())())", [-1, 0, -1, -2, -1, -2, -3]);
    }
}

use std::convert::TryFrom;

use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(Self::Up),
            ')' => Ok(Self::Down),
            _ => Err(anyhow!("invalid direction: {:?}", c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_char() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(Direction::try_from($input).unwrap(), $expected);
            };
        }

        test!('(', Direction::Up);
        test!(')', Direction::Down);
    }
}

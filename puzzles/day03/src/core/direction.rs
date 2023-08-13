use std::convert::TryFrom;

use anyhow::anyhow;

use super::Offset;

use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn offset(&self) -> Offset {
        match self {
            North => Offset::new(0, 1),
            South => Offset::new(0, -1),
            East => Offset::new(1, 0),
            West => Offset::new(-1, 0),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(North),
            'v' => Ok(South),
            '>' => Ok(East),
            '<' => Ok(West),
            _ => Err(anyhow!("invalid direction: {:?}", c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(Direction::try_from($input).unwrap(), $expected);
            };
        }

        test!('^', North);
        test!('v', South);
        test!('>', East);
        test!('<', West);

        assert!(Direction::try_from('x').is_err());
    }
}

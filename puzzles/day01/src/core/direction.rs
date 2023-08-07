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

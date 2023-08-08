use std::convert::TryFrom;

use anyhow::Context;

use super::{Dimensions2, Dimensions3, Size};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Present(Dimensions3);

impl Present {
    pub fn new(length: Size, width: Size, height: Size) -> Self {
        Dimensions3::new(length, width, height).into()
    }

    fn sides(&self) -> nalgebra::Vector3<Dimensions2> {
        [self.0.xy(), self.0.yz(), self.0.xz()].into()
    }

    fn surface_area(&self) -> Size {
        self.sides()
            .into_iter()
            .map(|&side| 2 * side.product())
            .sum()
    }

    fn volume(&self) -> Size {
        self.0.product()
    }

    fn smallest_side(&self) -> Dimensions2 {
        self.sides()
            .into_iter()
            .min_by(|&&a, &&b| a.product().cmp(&b.product()))
            .cloned()
            .unwrap()
    }

    pub fn wrapping_paper_surface_area(&self) -> Size {
        self.surface_area() + self.smallest_side().product()
    }

    pub fn ribbon_length(&self) -> Size {
        self.volume() + 2 * self.smallest_side().sum()
    }
}

impl From<Dimensions3> for Present {
    fn from(dimensions: Dimensions3) -> Self {
        Self(dimensions)
    }
}

impl From<[Size; 3]> for Present {
    fn from(dimensions: [Size; 3]) -> Self {
        Dimensions3::from(dimensions).into()
    }
}

impl TryFrom<&str> for Present {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut tokens = s.split('x');

        let length = tokens.next().context("missing length")?.parse()?;
        let width = tokens.next().context("missing width")?.parse()?;
        let height = tokens.next().context("missing height")?.parse()?;

        Ok(Self::new(length, width, height))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_str() {
        macro_rules! test {
            ($input:expr, [$length:expr, $width:expr, $height:expr]) => {
                assert_eq!(
                    Present::try_from($input).unwrap(),
                    Present::new($length, $width, $height)
                );
            };
        }

        test!("2x3x4", [2, 3, 4]);
        test!("1x1x10", [1, 1, 10]);
    }

    #[test]
    fn wrapping_paper_surface_area() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    Present::try_from($input)
                        .unwrap()
                        .wrapping_paper_surface_area(),
                    $expected
                );
            };
        }

        test!("2x3x4", 58);
        test!("1x1x10", 43);
    }

    #[test]
    fn ribbon_length() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    Present::try_from($input).unwrap().ribbon_length(),
                    $expected
                );
            };
        }

        test!("2x3x4", 34);
        test!("1x1x10", 14);
    }
}

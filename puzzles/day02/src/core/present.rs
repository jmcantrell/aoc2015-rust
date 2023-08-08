use std::convert::TryFrom;

use anyhow::Context;

type Size = usize;
type Vector2<T> = [T; 2];
type Vector3<T> = [T; 3];
type Dimensions2 = Vector2<Size>;
type Dimensions3 = Vector3<Size>;

fn area(dimensions: &Dimensions2) -> Size {
    dimensions[0] * dimensions[1]
}

fn perimeter(dimensions: &Dimensions2) -> Size {
    2 * (dimensions[0] + dimensions[1])
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Present(Dimensions3);

impl Present {
    pub fn length(&self) -> Size {
        self.0[0]
    }

    pub fn width(&self) -> Size {
        self.0[1]
    }

    pub fn height(&self) -> Size {
        self.0[2]
    }

    pub fn sides(&self) -> Vector3<Dimensions2> {
        [
            [self.length(), self.width()],
            [self.width(), self.height()],
            [self.height(), self.length()],
        ]
    }

    pub fn surface_area(&self) -> Size {
        self.sides()
            .iter()
            .map(|dimensions| 2 * area(dimensions))
            .sum()
    }

    pub fn volume(&self) -> Size {
        self.0.into_iter().product()
    }

    pub fn smallest_side(&self) -> Dimensions2 {
        self.sides()
            .into_iter()
            .min_by(|a, b| area(a).cmp(&area(b)))
            .unwrap()
    }

    pub fn wrapping_paper_surface_area(&self) -> Size {
        self.surface_area() + area(&self.smallest_side())
    }

    pub fn ribbon_length(&self) -> Size {
        self.volume() + perimeter(&self.smallest_side())
    }
}

impl From<Dimensions3> for Present {
    fn from(dimensions: Dimensions3) -> Self {
        Self(dimensions)
    }
}

impl From<Present> for Dimensions3 {
    fn from(present: Present) -> Self {
        present.0
    }
}

impl TryFrom<&str> for Present {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut tokens = s.split('x');

        let length = tokens.next().context("missing length")?.parse()?;
        let width = tokens.next().context("missing width")?.parse()?;
        let height = tokens.next().context("missing height")?.parse()?;

        Ok(Self([length, width, height]))
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
                    Present::from([$length, $width, $height])
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

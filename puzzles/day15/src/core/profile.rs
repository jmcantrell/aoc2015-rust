use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::Sum;
use std::ops::{Add, Mul};

use anyhow::Context;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Profile {
    pub capacity: isize,
    pub durability: isize,
    pub flavor: isize,
    pub texture: isize,
    pub calories: isize,
}

impl Profile {
    pub fn max(self, n: isize) -> Self {
        Self {
            capacity: self.capacity.max(n),
            durability: self.durability.max(n),
            flavor: self.flavor.max(n),
            texture: self.texture.max(n),
            calories: self.calories.max(n),
        }
    }

    pub fn score(&self) -> isize {
        self.capacity * self.durability * self.flavor * self.texture
    }
}

impl Sum<Self> for Profile {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Default::default(), |a, b| a + b)
    }
}

impl Add<Self> for Profile {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            capacity: self.capacity + other.capacity,
            durability: self.durability + other.durability,
            flavor: self.flavor + other.flavor,
            texture: self.texture + other.texture,
            calories: self.calories + other.calories,
        }
    }
}

impl Mul<isize> for Profile {
    type Output = Self;

    fn mul(self, n: isize) -> Self {
        Self {
            capacity: self.capacity * n,
            durability: self.durability * n,
            flavor: self.flavor * n,
            texture: self.texture * n,
            calories: self.calories * n,
        }
    }
}

impl TryFrom<&str> for Profile {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn parse_property(s: &str) -> anyhow::Result<(&str, isize)> {
            let mut tokens = s.split_whitespace();
            let name = tokens.next().context("missing property name")?;
            let value = tokens.next().context("missing property value")?.parse()?;
            Ok((name, value))
        }

        let mut mapping = s
            .split(',')
            .enumerate()
            .map(|(i, s)| parse_property(s).context(format!("property number {}", i + 1)))
            .collect::<Result<HashMap<_, _>, _>>()?;

        let mut take_value = |name: &str| -> anyhow::Result<isize> {
            mapping
                .remove(name)
                .context(format!("missing {:?} property", name))
        };

        let capacity = take_value("capacity")?;
        let durability = take_value("durability")?;
        let flavor = take_value("flavor")?;
        let texture = take_value("texture")?;
        let calories = take_value("calories")?;

        Ok(Self {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_str() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(Profile::try_from($input).unwrap(), $expected);
            };
        }

        test!(
            "capacity -1, durability -2, flavor 6, texture 3, calories 8",
            Profile {
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                calories: 8
            }
        );
    }

    #[test]
    fn add_self() {
        assert_eq!(
            Profile {
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                calories: 8
            } + Profile {
                capacity: 2,
                durability: 3,
                flavor: -2,
                texture: -1,
                calories: 3
            },
            Profile {
                capacity: 1,
                durability: 1,
                flavor: 4,
                texture: 2,
                calories: 11
            }
        );
    }

    #[test]
    fn mul_usize() {
        assert_eq!(
            Profile {
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                calories: 8
            } * 2,
            Profile {
                capacity: -2,
                durability: -4,
                flavor: 12,
                texture: 6,
                calories: 16
            }
        );
    }

    #[test]
    fn score() {
        assert_eq!(
            (Profile {
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                calories: 8,
            } * 44
                + Profile {
                    capacity: 2,
                    durability: 3,
                    flavor: -2,
                    texture: -1,
                    calories: 3,
                } * 56)
                .score(),
            68 * 80 * 152 * 76
        );
    }

    #[test]
    fn sum_self() {
        let a = Profile {
            capacity: -1,
            durability: -2,
            flavor: 6,
            texture: 3,
            calories: 8,
        };

        let b = Profile {
            capacity: 2,
            durability: 3,
            flavor: -2,
            texture: -1,
            calories: 3,
        };

        assert_eq!([a, b].into_iter().sum::<Profile>(), a + b);
    }
}

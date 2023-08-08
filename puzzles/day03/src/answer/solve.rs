use std::collections::HashSet;

use crate::core::Location;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    let mut deliveries = HashSet::new();
    let mut santa = Location::default();

    deliveries.insert(santa);

    for direction in parsed {
        santa += direction.offset();
        deliveries.insert(santa);
    }

    Ok(deliveries.len())
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    let mut deliveries = HashSet::new();
    let mut santa = Location::default();
    let mut robo_santa = Location::default();

    deliveries.insert(santa);
    deliveries.insert(robo_santa);

    for direction in parsed.iter().step_by(2) {
        santa += direction.offset();
        deliveries.insert(santa);
    }

    for direction in parsed.iter().skip(1).step_by(2) {
        robo_santa += direction.offset();
        deliveries.insert(robo_santa);
    }

    Ok(deliveries.len())
}

#[cfg(test)]
pub mod tests {
    use crate::core::Direction::*;

    #[test]
    fn solve1() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(super::solve1(&Vec::from($input)).unwrap(), $expected);
            };
        }

        test!([], 1);
        test!([East], 2);
        test!([North, East, South, West], 4);
        test!(
            [North, South, North, South, North, South, North, South, North, South],
            2
        );
    }

    #[test]
    fn solve2() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(super::solve2(&Vec::from($input)).unwrap(), $expected);
            };
        }

        test!([], 1);
        test!([North, South], 3);
        test!([North, East, South, West], 3);
        test!(
            [North, South, North, South, North, South, North, South, North, South],
            11
        );
    }
}

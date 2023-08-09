use std::collections::HashSet;

use crate::core::iter_path;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(iter_path(parsed).collect::<HashSet<_>>().len())
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    let santa: HashSet<_> = iter_path(parsed.iter().step_by(2)).collect();
    let robo_santa: HashSet<_> = iter_path(parsed.iter().skip(1).step_by(2)).collect();
    Ok(santa.union(&robo_santa).count())
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

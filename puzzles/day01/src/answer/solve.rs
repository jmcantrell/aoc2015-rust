use anyhow::Context;

use crate::core::iter_path;

use super::{Parsed1, Parsed2};

pub type Solution1 = isize;
pub type Solution2 = usize;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    iter_path(parsed)
        .last()
        .context("instructions have no steps")
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    iter_path(parsed)
        .position(|floor| floor < 0)
        .context("santa never enters basement")
        .map(|i| i + 1)
}

#[cfg(test)]
mod tests {
    use crate::core::Direction;

    use super::*;

    use Direction::*;

    #[test]
    fn test_solve1() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(solve1(&Vec::from($input)).unwrap(), $expected);
            };
        }

        test!([Up, Up, Down, Down], 0);
        test!([Up, Down, Up, Down], 0);
        test!([Up, Up, Up], 3);
        test!([Up, Up, Down, Up, Up, Down, Up], 3);
        test!([Down, Down, Up, Up, Up, Up, Up], 3);
        test!([Up, Down, Down], -1);
        test!([Down, Down, Up], -1);
        test!([Down, Down, Down], -3);
        test!([Down, Up, Down, Down, Up, Down, Down], -3);
    }

    #[test]
    fn test_solve2() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(solve2(&Vec::from($input)).unwrap(), $expected);
            };
        }

        test!([Down], 1);
        test!([Up, Down, Up, Down, Down], 5);
    }
}

use anyhow::Context;

use crate::core::iter_floor;

use super::{Parsed1, Parsed2};

pub type Solution1 = isize;
pub type Solution2 = usize;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    iter_floor(parsed)
        .last()
        .context("instructions have no steps")
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    iter_floor(parsed)
        .position(|floor| floor < 0)
        .context("santa never enters basement")
        .map(|i| i + 1)
}

#[cfg(test)]
pub mod tests {
    use crate::answer::{parse1, parse2};

    #[test]
    fn solve1() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(super::solve1(&parse1($input).unwrap()).unwrap(), $expected);
            };
        }

        test!("(())", 0);
        test!("()()", 0);
        test!("(((", 3);
        test!("(()(()(", 3);
        test!("))(((((", 3);
        test!("())", -1);
        test!("))(", -1);
        test!(")))", -3);
        test!(")())())", -3);
    }

    #[test]
    fn solve2() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(super::solve2(&parse2($input).unwrap()).unwrap(), $expected);
            };
        }

        test!(")", 1);
        test!("()())", 5);
    }
}

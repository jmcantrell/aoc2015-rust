use crate::core::{escape, unescape};

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    let mut result = 0;

    for s in parsed {
        result += s.len() - unescape(s)?.len();
    }

    Ok(result)
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    let mut result = 0;

    for s in parsed {
        result += escape(s)?.len() - s.len();
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 12);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 19);
        Ok(())
    }
}

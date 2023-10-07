use anyhow::Context;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn find_first_house_with_at_least(
    target: usize,
    each_elf_carries: usize,
    elf_house_limit: Option<usize>,
) -> Option<usize> {
    let max = target / each_elf_carries;
    let mut memo = vec![0; max];

    for i in 0..max {
        let n = i + 1;
        let presents = each_elf_carries * n;
        for j in (i..max).step_by(n).take(elf_house_limit.unwrap_or(max)) {
            memo[j] += presents;
        }
    }

    memo.iter()
        .position(|&count| count >= target)
        .map(|i| i + 1)
}

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    find_first_house_with_at_least(*parsed, 10, None).context("not found")
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    find_first_house_with_at_least(*parsed, 11, Some(50)).context("not found")
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 48);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 36);
        Ok(())
    }
}

use crate::core::Happiness;

use super::{Parsed1, Parsed2};

type Solution = Happiness;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(parsed.optimal_happiness())
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    let mut potentials = parsed.clone();
    potentials.add_me();
    Ok(potentials.optimal_happiness())
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 330);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 286);
        Ok(())
    }
}

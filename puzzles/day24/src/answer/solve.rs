use anyhow::Context;

use crate::core::{ideal_quantum_entanglement, QuantumEntanglement};

use super::{Parsed1, Parsed2};

type Solution = QuantumEntanglement;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(weights: &Parsed1) -> anyhow::Result<Solution1> {
    ideal_quantum_entanglement(weights, 3).context("no solution")
}

pub fn solve2(weights: &Parsed2) -> anyhow::Result<Solution2> {
    ideal_quantum_entanglement(weights, 4).context("no solution")
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 99);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 44);
        Ok(())
    }
}

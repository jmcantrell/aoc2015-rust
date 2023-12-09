use itertools::Itertools;

use crate::core::ContainerSize;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

#[cfg(test)]
const TARGET: usize = 25;

#[cfg(not(test))]
const TARGET: usize = 150;

fn find_containers(
    containers: &[ContainerSize],
    target: ContainerSize,
) -> impl Iterator<Item = Vec<&ContainerSize>> {
    containers
        .iter()
        .powerset()
        .filter(move |containers| containers.iter().cloned().sum::<ContainerSize>() == target)
}

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(find_containers(parsed, TARGET).count())
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(find_containers(parsed, TARGET)
        .min_set_by_key(|set| set.len())
        .len())
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 4);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 3);
        Ok(())
    }
}

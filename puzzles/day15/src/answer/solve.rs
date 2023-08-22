use std::collections::HashMap;

use anyhow::Context;

use super::{Parsed1, Parsed2};

type Solution = isize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

use crate::core::{iter_partitions, Profile};

fn iter_cookies<'a>(ingredients: &'a HashMap<&str, Profile>) -> impl Iterator<Item = Profile> + 'a {
    iter_partitions(100, ingredients.len()).map(|partitioning| {
        partitioning
            .into_iter()
            .zip(ingredients.values())
            .map(|(partition, &profile)| profile * partition as isize)
            .sum::<Profile>()
            .max(0)
    })
}

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    iter_cookies(parsed)
        .map(|profile| profile.score())
        .max()
        .context("no ingredients")
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    iter_cookies(parsed)
        .filter_map(|profile| (profile.calories == 500).then(|| profile.score()))
        .max()
        .context("no ingredients")
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 62_842_880);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 57_600_000);
        Ok(())
    }
}

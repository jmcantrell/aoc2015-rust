use anyhow::Context;

use crate::core::{Distance, Map};

use super::{Parsed1, Parsed2};

type Solution = Distance;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    let map = Map::from_iter(parsed.clone());
    let (path, distance) = map.shortest_path().context("no paths")?;
    dbg!(path);
    Ok(distance)
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    let map = Map::from_iter(parsed.clone());
    let (path, distance) = map.longest_path().context("no paths")?;
    dbg!(path);
    Ok(distance)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 605);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 982);
        Ok(())
    }
}

use anyhow::{ensure, Context};

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

#[cfg(not(test))]
const TIME_LIMIT: usize = 2503;

#[cfg(test)]
const TIME_LIMIT: usize = 1000;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    parsed
        .iter()
        .map(|reindeer| reindeer.distance_traveled_after(TIME_LIMIT))
        .max()
        .context("no reindeer")
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    ensure!(!parsed.is_empty(), "no reindeer");

    let mut points = vec![0usize; parsed.len()];
    let mut distances = vec![0usize; parsed.len()];

    for duration in 1..=TIME_LIMIT {
        for (i, reindeer) in parsed.iter().enumerate() {
            distances[i] = reindeer.distance_traveled_after(duration);
        }

        let max_distance = distances.iter().max().unwrap();

        for (i, distance) in distances.iter().enumerate() {
            if distance == max_distance {
                points[i] += 1;
            }
        }
    }

    Ok(points.into_iter().max().unwrap())
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 1120);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 689);
        Ok(())
    }
}

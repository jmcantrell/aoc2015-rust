use std::collections::HashSet;

use crate::core::{positions, splice};

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1((pairs, medicine): &Parsed1) -> anyhow::Result<Solution1> {
    let derivatives: HashSet<String> = pairs
        .iter()
        .flat_map(|&(from, to)| {
            positions(medicine, from).map(|i| splice(medicine, i, i + from.len(), to))
        })
        .collect();

    Ok(derivatives.len())
}

pub fn solve2((pairs, medicine): &Parsed2) -> anyhow::Result<Solution2> {
    let mut medicine = medicine.to_string();
    let mut steps = 0;

    while !medicine.chars().all(|c| c == 'e') {
        for &(to, from) in pairs.iter() {
            let maybe_i = positions(&medicine, from).next();

            if let Some(i) = maybe_i {
                medicine = splice(&medicine, i, i + from.len(), to);
                steps += 1;
            }
        }
    }

    Ok(steps)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT1A: Input = include_str!("../../input-test1a.txt");
    const INPUT1B: Input = include_str!("../../input-test1b.txt");

    const INPUT2A: Input = include_str!("../../input-test2a.txt");
    const INPUT2B: Input = include_str!("../../input-test2b.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT1A)?)?, 4);
        assert_eq!(super::solve1(&parse1(INPUT1B)?)?, 7);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT2A)?)?, 3);
        assert_eq!(super::solve2(&parse2(INPUT2B)?)?, 6);
        Ok(())
    }
}

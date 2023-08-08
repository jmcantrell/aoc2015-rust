use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(parsed
        .iter()
        .map(|present| present.wrapping_paper_surface_area())
        .sum())
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(parsed.iter().map(|present| present.ribbon_length()).sum())
}

#[cfg(test)]
pub mod tests {
    use crate::answer::{parse1, parse2};

    #[test]
    fn solve1() -> anyhow::Result<()> {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(super::solve1(&parse1($input)?)?, $expected);
            };
        }

        test!("2x3x4\n1x1x10\n", 58 + 43);

        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(super::solve2(&parse2($input)?)?, $expected);
            };
        }

        test!("2x3x4\n1x1x10\n", 34 + 14);

        Ok(())
    }
}

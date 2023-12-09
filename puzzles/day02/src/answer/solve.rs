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
mod tests {
    use crate::core::Present;

    use super::*;

    #[test]
    fn test_solve1() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    solve1(&$input.into_iter().map(Present::from).collect()).unwrap(),
                    $expected
                );
            };
        }

        test!([[2, 3, 4], [1, 1, 10]], 58 + 43);
    }

    #[test]
    fn test_solve2() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    solve2(&$input.into_iter().map(Present::from).collect()).unwrap(),
                    $expected
                );
            };
        }

        test!([[2, 3, 4], [1, 1, 10]], 34 + 14);
    }
}

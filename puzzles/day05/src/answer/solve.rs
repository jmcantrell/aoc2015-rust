use crate::core::{better, ridiculous};

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(parsed.iter().filter(|&s| ridiculous::is_nice(s)).count())
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(parsed.iter().filter(|&s| better::is_nice(s)).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = vec![
            "ugknbfddgicrmopn",
            "aaa",
            "jchzalrnumimnmhp",
            "haegwjzuvuyypxyu",
            "dvszwmarrgswjxmb",
        ];

        assert_eq!(solve1(&input).unwrap(), 2);
    }

    #[test]
    fn test_solve2() {
        let input = vec![
            "qjhvhtzxzqqjkmpb",
            "xxyxx",
            "uurcxstgmygtbstg",
            "ieodomkazucvgmuy",
        ];

        assert_eq!(solve2(&input).unwrap(), 2);
    }
}

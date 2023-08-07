use crate::core::Visitor;

use super::{Parsed1, Parsed2};

pub type Solution1 = isize;
pub type Solution2 = Option<usize>;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    let mut santa = Visitor::new();
    for dir in parsed {
        santa.step_vertically(dir);
    }
    Ok(santa.current_floor())
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    let mut santa = Visitor::new();
    for (i, dir) in parsed.iter().enumerate() {
        santa.step_vertically(dir);
        if santa.current_floor() < 0 {
            return Ok(Some(i + 1));
        }
    }
    Ok(None)
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
        test!("(())", 0);
        test!("()()", 0);
        test!("(((", 3);
        test!("(()(()(", 3);
        test!("))(((((", 3);
        test!("())", -1);
        test!("))(", -1);
        test!(")))", -3);
        test!(")())())", -3);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(super::solve2(&parse2($input)?)?, $expected);
            };
        }
        test!("(", None);
        test!(")", Some(1));
        test!("()())", Some(5));
        Ok(())
    }
}

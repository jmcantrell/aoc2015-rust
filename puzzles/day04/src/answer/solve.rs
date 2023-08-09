use anyhow::anyhow;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn solve(input: &[u8], prefix_len: usize) -> anyhow::Result<Solution> {
    let prefix: String = "0".repeat(prefix_len);

    for n in 0_usize.. {
        let mut context = md5::Context::new();
        context.consume(input);
        context.consume(n.to_string());
        if format!("{:x}", context.compute()).starts_with(&prefix) {
            return Ok(n);
        }
    }

    Err(anyhow!("no solution"))
}

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    solve(parsed, 5)
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    solve(parsed, 6)
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn solve() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    super::solve(&$input.as_bytes().to_vec(), 5).unwrap(),
                    $expected
                );
            };
        }

        test!("abcdef", 609043);
        test!("pqrstuv", 1048970);
    }
}

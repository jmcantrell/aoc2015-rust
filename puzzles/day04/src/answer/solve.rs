use anyhow::Context;

use crate::core::find_salt;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    find_salt(parsed, 5).context("no solution")
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    find_salt(parsed, 6).context("no solution")
}

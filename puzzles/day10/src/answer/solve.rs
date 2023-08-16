use anyhow::Context;

use crate::core::iter_look_and_say;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    iter_look_and_say(parsed.to_string())
        .nth(39)
        .map(|s| s.len())
        .context("unable to iterate 40 times")
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    iter_look_and_say(parsed.to_string())
        .nth(49)
        .map(|s| s.len())
        .context("unable to iterate 50 times")
}

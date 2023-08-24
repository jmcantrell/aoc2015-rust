use anyhow::Context;
use once_cell::sync::Lazy;

use aoc::Input;

use crate::core::{Compound, Profile};

use super::{Parsed1, Parsed2};

use Compound::*;

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

const PARAMS: Input = include_str!("../../params.txt");
static DETECTION: Lazy<Profile> = Lazy::new(|| PARAMS.try_into().unwrap());

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    parsed
        .iter()
        .position(|sue| sue.matches(&DETECTION))
        .map(|i| i + 1)
        .context("no match")
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    parsed
        .iter()
        .position(|sue| {
            sue.matches_with(&DETECTION, |key, value, other_value| match key {
                Cats | Trees => value > other_value,
                Pomeranians | Goldfish => value < other_value,
                _ => value == other_value,
            })
        })
        .map(|i| i + 1)
        .context("no match")
}

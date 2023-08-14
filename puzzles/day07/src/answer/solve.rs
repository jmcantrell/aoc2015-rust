use anyhow::Context;

use crate::core::{Signal, Simulator};

use super::{Parsed1, Parsed2};

type Solution = Signal;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    Simulator::new(parsed).eval_wire("a").context("no signal")
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    let signal_a = Simulator::new(parsed)
        .eval_wire("a")
        .context("no signal before override")?;

    let mut simulator = Simulator::new(parsed);
    simulator.override_wire("b", signal_a);

    simulator.eval_wire("a").context("no signal after override")
}

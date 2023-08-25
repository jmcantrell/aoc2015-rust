use std::collections::HashSet;

use crate::core::{conway, Light, LightCell, LightGrid, Location};

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

const NUM_STEPS: usize = 100;

fn simulate<F>(grid: &LightGrid, evolve: F) -> LightGrid
where
    F: Fn(LightCell, usize) -> Light,
{
    grid.animate(evolve).nth(NUM_STEPS - 1).unwrap()
}

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(simulate(parsed, |(_, &lit), num_lit_neighbors| {
        conway::evolve(lit, num_lit_neighbors)
    })
    .len_lit())
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    let corners: HashSet<Location> = parsed.corners().into_iter().collect();

    Ok(
        simulate(parsed, move |(location, &lit), num_lit_neighbors| {
            corners.contains(&location) || conway::evolve(lit, num_lit_neighbors)
        })
        .len_lit(),
    )
}

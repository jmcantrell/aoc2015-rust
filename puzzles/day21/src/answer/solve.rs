use anyhow::Context;

use itertools::iproduct;

use crate::core::{battle, Character, Item, Loadout, Value, ARMOR, RINGS, WEAPONS};

use super::{Parsed1, Parsed2};

type Solution = Value;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn iter_loadouts() -> impl Iterator<Item = Loadout> {
    iproduct!(
        0..WEAPONS.len(),
        0..=ARMOR.len(),
        0..=RINGS.len(),
        0..=RINGS.len()
    )
    .filter_map(|(weapon, armor, ring1, ring2)| {
        Loadout::try_from((
            WEAPONS[weapon],
            (armor > 0).then(|| ARMOR[armor - 1]),
            (ring1 > 0).then(|| RINGS[ring1 - 1]),
            (ring2 > 0).then(|| RINGS[ring2 - 1]),
        ))
        .ok()
    })
}

fn iter_players() -> impl Iterator<Item = (Value, Character)> {
    iter_loadouts().map(move |loadout| {
        let hit_points = 100;

        let Item {
            cost,
            damage,
            armor,
        } = loadout.sum();

        let player = (hit_points, damage, armor).into();

        (cost, player)
    })
}

pub fn solve1(boss: &Parsed1) -> anyhow::Result<Solution1> {
    iter_players()
        .filter_map(|(cost, player)| (battle(&player, boss).0 > 0).then_some(cost))
        .min()
        .context("no battles fought")
}

pub fn solve2(boss: &Parsed2) -> anyhow::Result<Solution2> {
    iter_players()
        .filter_map(|(cost, player)| (battle(&player, boss).1 > 0).then_some(cost))
        .max()
        .context("no battles fought")
}

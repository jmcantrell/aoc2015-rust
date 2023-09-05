use std::collections::HashMap;
use std::convert::TryFrom;

use anyhow::Context;

use super::{Character, Damage, HitPoints};

#[derive(Debug, Clone, Copy)]
pub struct Boss {
    pub hit_points: HitPoints,
    pub damage: Damage,
}

impl Character for Boss {
    fn hit_points(&self) -> &HitPoints {
        &self.hit_points
    }

    fn hit_points_mut(&mut self) -> &mut HitPoints {
        &mut self.hit_points
    }
}

impl TryFrom<&str> for Boss {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn parse_mapping(s: &str) -> anyhow::Result<(&str, &str)> {
            let (left, right) = s
                .split_once(':')
                .context("expected mapping to be delimited by a colon")?;

            Ok((left.trim(), right.trim()))
        }

        let mut map: HashMap<_, _> = s
            .lines()
            .enumerate()
            .map(|(i, s)| parse_mapping(s).context(format!("line number {}", i + 1)))
            .collect::<Result<_, _>>()?;

        let hit_points = map
            .remove("Hit Points")
            .context("missing hit points")?
            .parse()?;

        let damage = map.remove("Damage").context("missing damage")?.parse()?;

        Ok(Self { hit_points, damage })
    }
}

use std::collections::HashMap;
use std::convert::TryFrom;

use anyhow::Context;

use super::Value;

type Raw = (Value, Value, Value);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Character {
    pub hit_points: Value,
    pub damage: Value,
    pub armor: Value,
}

impl Character {
    pub fn attack_value_against(&self, other: &Self) -> Value {
        self.damage.saturating_sub(other.armor).max(1)
    }
}

impl From<Raw> for Character {
    fn from((hit_points, damage, armor): Raw) -> Self {
        Self {
            hit_points,
            damage,
            armor,
        }
    }
}

impl TryFrom<&str> for Character {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn parse_mapping(s: &str) -> anyhow::Result<(&str, Value)> {
            let (key, value) = s
                .split_once(':')
                .context("expected key and value to be separated by a colon")?;

            Ok((key.trim(), value.trim().parse()?))
        }

        let mut map: HashMap<&str, Value> = s
            .lines()
            .enumerate()
            .map(|(i, s)| parse_mapping(s).with_context(|| format!("mapping number {}", i + 1)))
            .collect::<Result<_, _>>()?;

        let hit_points: Value = map.remove("Hit Points").context("missing hit points")?;
        let damage: Value = map.remove("Damage").context("missing damage")?;
        let armor: Value = map.remove("Armor").context("missing armor")?;

        Ok(Self {
            hit_points,
            damage,
            armor,
        })
    }
}

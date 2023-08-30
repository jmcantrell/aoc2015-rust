use std::convert::TryFrom;

use anyhow::ensure;

use super::Item;

type Raw = (Item, Option<Item>, Option<Item>, Option<Item>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Loadout {
    pub weapon: Item,
    pub armor: Option<Item>,
    pub ring1: Option<Item>,
    pub ring2: Option<Item>,
}

impl Loadout {
    pub fn sum(&self) -> Item {
        self.weapon
            + self.armor.unwrap_or_default()
            + self.ring1.unwrap_or_default()
            + self.ring2.unwrap_or_default()
    }
}

impl TryFrom<Raw> for Loadout {
    type Error = anyhow::Error;

    fn try_from((weapon, armor, ring1, ring2): Raw) -> Result<Self, Self::Error> {
        ensure!(
            ring1.is_none() || ring1 != ring2,
            "cannot have two of the same ring"
        );

        Ok(Self {
            weapon,
            armor,
            ring1,
            ring2,
        })
    }
}

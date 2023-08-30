use std::ops::Add;

use super::Value;

type Raw = (Value, Value, Value);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Item {
    pub cost: Value,
    pub damage: Value,
    pub armor: Value,
}

impl From<Raw> for Item {
    fn from((cost, damage, armor): Raw) -> Self {
        Self {
            cost,
            damage,
            armor,
        }
    }
}

impl Add<Self> for Item {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            cost: self.cost + other.cost,
            damage: self.damage + other.damage,
            armor: self.armor + other.armor,
        }
    }
}

use super::{Armor, Damage, Mana};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Effect {
    Damage(Damage),
    Armor(Armor),
    Mana(Mana),
}

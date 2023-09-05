use once_cell::sync::Lazy;

use super::{Effect, Mana, Spell};

pub type SpellBook = Vec<(Mana, Spell)>;

pub static SPELLS: Lazy<SpellBook> = Lazy::new(|| {
    vec![
        (53, Spell::Missile(4)),
        (73, Spell::Drain(2)),
        (113, Spell::Shield(6, Effect::Armor(7))),
        (173, Spell::Poison(6, Effect::Damage(3))),
        (229, Spell::Recharge(5, Effect::Mana(101))),
    ]
});

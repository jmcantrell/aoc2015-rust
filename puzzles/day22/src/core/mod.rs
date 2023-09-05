pub type HitPoints = u8;
pub type Damage = u8;
pub type Armor = u8;
pub type Mana = u16;
pub type Timer = u8;

pub mod character;
pub use character::*;

pub mod boss;
pub use boss::*;

pub mod wizard;
pub use wizard::*;

pub mod spell;
pub use spell::*;

pub mod effect;
pub use effect::*;

pub mod magic;
pub use magic::*;

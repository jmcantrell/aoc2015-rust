use super::{Damage, Effect, Timer};

#[derive(Debug, Clone, Copy)]
pub enum Spell {
    Missile(Damage),
    Drain(Damage),
    Shield(Timer, Effect),
    Poison(Timer, Effect),
    Recharge(Timer, Effect),
}

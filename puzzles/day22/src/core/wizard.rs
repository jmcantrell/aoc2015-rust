use super::{Character, HitPoints, Mana};

#[derive(Debug, Clone, Copy)]
pub struct Wizard {
    pub hit_points: HitPoints,
    pub mana: Mana,
}

impl Character for Wizard {
    fn hit_points(&self) -> &HitPoints {
        &self.hit_points
    }

    fn hit_points_mut(&mut self) -> &mut HitPoints {
        &mut self.hit_points
    }
}

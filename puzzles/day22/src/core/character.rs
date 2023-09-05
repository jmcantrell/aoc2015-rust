use super::{Damage, HitPoints};

pub trait Character {
    fn hit_points(&self) -> &HitPoints;

    fn hit_points_mut(&mut self) -> &mut HitPoints;

    fn attack(&mut self, damage: Damage) {
        let hit_points = self.hit_points_mut();
        *hit_points = hit_points.saturating_sub(damage);
    }
}

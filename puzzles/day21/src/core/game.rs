use super::{Character, Value};

pub fn battle(player: &Character, boss: &Character) -> (Value, Value) {
    let mut player_hit_points = player.hit_points;
    let mut boss_hit_points = boss.hit_points;

    let player_attack = player.attack_value_against(boss);
    let boss_attack = boss.attack_value_against(player);

    fn attack(hit_points: &mut Value, attack: &Value) -> bool {
        *hit_points = hit_points.saturating_sub(*attack);
        *hit_points == 0
    }

    loop {
        if attack(&mut boss_hit_points, &player_attack) {
            break;
        }

        if attack(&mut player_hit_points, &boss_attack) {
            break;
        }
    }

    (player_hit_points, boss_hit_points)
}

#[cfg(test)]
mod tests {
    #[test]
    fn battle() {
        assert_eq!(super::battle(&(8, 5, 5).into(), &(12, 7, 2).into()), (2, 0));
    }
}

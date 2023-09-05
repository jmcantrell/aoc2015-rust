use std::collections::HashMap;

use anyhow::Context;

use crate::core::{Armor, Boss, Character, Damage, Effect, Mana, Spell, Timer, Wizard, SPELLS};

use super::{Parsed1, Parsed2};

type Solution = Mana;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

type EffectSet = HashMap<Effect, Timer>;

fn find_cheapest_win<F>(boss: &Boss, map_player_turn: F) -> Option<Mana>
where
    F: Fn(Wizard) -> Wizard,
{
    fn tick_effects(effects: EffectSet) -> (EffectSet, Damage, Armor, Mana) {
        effects.into_iter().fold(
            (HashMap::new(), 0, 0, 0),
            |(mut effects, mut damage, mut armor, mut mana), (effect, mut timer)| {
                match effect {
                    Effect::Damage(value) => {
                        damage += value;
                    }
                    Effect::Armor(value) => {
                        armor += value;
                    }
                    Effect::Mana(value) => {
                        mana += value;
                    }
                }

                timer -= 1;

                if timer > 0 {
                    effects.insert(effect, timer);
                }

                (effects, damage, armor, mana)
            },
        )
    }

    fn check_win(cheapest_win: &mut Option<Mana>, mana_spent: &Mana, boss: &Boss) -> bool {
        if boss.hit_points == 0 {
            if let Some(best) = cheapest_win {
                if mana_spent < best {
                    *best = *mana_spent;
                }
            } else {
                *cheapest_win = Some(*mana_spent);
            }
            true
        } else {
            false
        }
    }

    fn apply_effects(
        effects: EffectSet,
        player: &mut Wizard,
        boss: &mut Boss,
    ) -> (EffectSet, Armor) {
        let (effects, damage, armor, mana) = tick_effects(effects);

        player.mana += mana;
        boss.attack(damage);

        (effects, armor)
    }

    fn recurse<F>(
        cheapest_win: &mut Option<Mana>,
        mana_spent: Mana,
        effects: HashMap<Effect, Timer>,
        mut player: Wizard,
        mut boss: Boss,
        map_player_turn: &F,
    ) where
        F: Fn(Wizard) -> Wizard,
    {
        use Spell::*;

        if let Some(best) = cheapest_win {
            if mana_spent > *best {
                return;
            }
        }

        player = map_player_turn(player);

        if player.hit_points == 0 {
            return;
        }

        // Effects apply at the start of the player's turn.
        let (effects, _) = apply_effects(effects, &mut player, &mut boss);

        if check_win(cheapest_win, &mana_spent, &boss) {
            return;
        }

        // If you cannot afford to cast any spell, you lose (i.e. don't consider that spell).
        // You cannot cast a spell that would start an effect which is already active.
        let spell_choices = SPELLS.iter().filter(|(cost, spell)| {
            cost <= &player.mana
                && match spell {
                    Shield(_, effect) | Poison(_, effect) | Recharge(_, effect) => {
                        !effects.contains_key(effect)
                    }
                    _ => true,
                }
        });

        for &(cost, spell) in spell_choices {
            let mut effects = effects.clone();
            let mut player = player;
            let mut boss = boss;

            // A spell's cost is immediately deducted when you cast it.
            let mana_spent = mana_spent + cost;
            player.mana -= cost;

            match spell {
                Missile(damage) => {
                    boss.attack(damage);
                }
                Drain(damage) => {
                    boss.attack(damage);
                    player.hit_points += damage;
                }
                Shield(timer, effect) | Poison(timer, effect) | Recharge(timer, effect) => {
                    effects.insert(effect, timer);
                }
            }

            if check_win(cheapest_win, &mana_spent, &boss) {
                continue;
            }

            // Effects also apply at the start of the boss' turns.
            let (effects, armor) = apply_effects(effects, &mut player, &mut boss);

            if check_win(cheapest_win, &mana_spent, &boss) {
                continue;
            }

            // The boss' attacks always deal at least 1 damage.
            player.attack(boss.damage.saturating_sub(armor).max(1));

            if player.hit_points == 0 {
                continue;
            }

            recurse(
                cheapest_win,
                mana_spent,
                effects,
                player,
                boss,
                map_player_turn,
            );
        }
    }

    let player = Wizard {
        hit_points: 50,
        mana: 500,
    };

    let mut cheapest_win: Option<Mana> = None;

    recurse(
        &mut cheapest_win,
        0,
        HashMap::new(),
        player,
        *boss,
        &map_player_turn,
    );

    cheapest_win
}

pub fn solve1(boss: &Parsed1) -> anyhow::Result<Solution1> {
    find_cheapest_win(boss, |player| player).context("no wins")
}

pub fn solve2(boss: &Parsed2) -> anyhow::Result<Solution2> {
    find_cheapest_win(boss, |mut player| {
        // At the start of each player turn, you lose 1 hit point.
        player.attack(1);
        player
    })
    .context("no wins")
}

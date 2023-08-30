use once_cell::sync::Lazy;

use super::{Item, Value};

pub static WEAPONS: Lazy<Vec<Item>> = Lazy::new(|| {
    into_items([
        (8, 4, 0),  // Dagger
        (10, 5, 0), // Shortsword
        (25, 6, 0), // Warhammer
        (40, 7, 0), // Longsword
        (74, 8, 0), // Greataxe
    ])
});

pub static ARMOR: Lazy<Vec<Item>> = Lazy::new(|| {
    into_items([
        (13, 0, 1),  // Leather
        (31, 0, 2),  // Chainmail
        (53, 0, 3),  // Splintmail
        (75, 0, 4),  // Bandedmail
        (102, 0, 5), // Platemail
    ])
});

pub static RINGS: Lazy<Vec<Item>> = Lazy::new(|| {
    into_items([
        (25, 1, 0),  // Damage +1
        (50, 2, 0),  // Damage +2
        (100, 3, 0), // Damage +3
        (20, 0, 1),  // Defense +1
        (40, 0, 2),  // Defense +2
        (80, 0, 3),  // Defense +3
    ])
});

fn into_items<I>(iter: I) -> Vec<Item>
where
    I: IntoIterator<Item = (Value, Value, Value)>,
{
    iter.into_iter().map(|tuple| tuple.into()).collect()
}

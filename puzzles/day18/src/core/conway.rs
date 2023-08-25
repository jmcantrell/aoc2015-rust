pub fn evolve(live: bool, num_live_neighbors: usize) -> bool {
    match live {
        // A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
        true => num_live_neighbors == 2 || num_live_neighbors == 3,
        // A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
        false => num_live_neighbors == 3,
    }
}

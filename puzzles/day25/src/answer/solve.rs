use crate::core::{iter_codes, Code};

use super::Parsed;

pub type Solution = Code;

pub fn solve(&(row, column): &Parsed) -> anyhow::Result<Solution> {
    let last_diagonal = row + column - 1; // manhattan distance from origin
    let triangle_size = last_diagonal * (last_diagonal + 1) / 2; // triangular number
    let n = triangle_size - (last_diagonal - column); // accounting for possible partial last diagonal
    Ok(iter_codes(20151125).nth(n - 1).unwrap())
}

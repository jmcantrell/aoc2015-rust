pub type Grid<T> = nalgebra::DMatrix<T>;
pub type Location = (usize, usize);
pub type Direction = (isize, isize);
pub type Cell<'a, T> = (Location, &'a T);

pub mod conway;

pub mod light_grid;
pub use light_grid::*;

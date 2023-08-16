pub type Location = &'static str;
pub type Distance = usize;
pub type Route = (Location, Location, Distance);

pub mod map;
pub use map::*;

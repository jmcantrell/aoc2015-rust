pub type Location = nalgebra::Point2<isize>;
pub type Offset = nalgebra::Vector2<isize>;

pub mod direction;
pub use direction::*;

pub mod path;
pub use path::*;

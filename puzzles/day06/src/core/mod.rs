pub type Location = nalgebra::Point2<usize>;
pub type BoolGrid = nalgebra::DMatrix<bool>;
pub type UintGrid = nalgebra::DMatrix<usize>;

pub mod action;
pub use action::*;

pub mod rectangle;
pub use rectangle::*;

pub mod command;
pub use command::*;

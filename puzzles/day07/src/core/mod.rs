use std::collections::HashMap;

pub type Signal = u16;
pub type Circuit = HashMap<String, Source>;

pub mod input;
pub use input::*;

pub mod source;
pub use source::*;

pub mod simulator;
pub use simulator::*;

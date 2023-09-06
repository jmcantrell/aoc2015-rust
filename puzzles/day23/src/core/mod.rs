pub type RegisterKey = usize;
pub type RegisterValue = usize;

pub const NUM_REGISTERS: usize = 2;
pub type Registers = [RegisterValue; NUM_REGISTERS];

pub type Offset = isize;

pub mod instruction;
pub use instruction::*;

pub mod program;
pub use program::*;

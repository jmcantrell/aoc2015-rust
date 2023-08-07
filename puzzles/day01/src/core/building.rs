use super::Direction;

pub type Floor = isize;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Visitor {
    current_floor: Floor,
}

impl Visitor {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn current_floor(&self) -> Floor {
        self.current_floor
    }

    pub fn step_vertically(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => {
                self.current_floor += 1;
            }
            Direction::Down => {
                self.current_floor -= 1;
            }
        }
    }
}

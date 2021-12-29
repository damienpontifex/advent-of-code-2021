use crate::direction::Direction;

#[derive(Default)]
pub(crate) struct Submarine {
    x: i32,
    y: i32,
}

impl Submarine {
    pub(crate) fn move_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Forward(magnitude) => self.x += magnitude,
            Direction::Up(magnitude) => self.y -= magnitude,
            Direction::Down(magnitude) => self.y += magnitude,
        }
    }

    pub(crate) fn finalize(&self) -> i32 {
        self.x * self.y
    }
}

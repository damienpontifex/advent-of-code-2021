use crate::direction::Direction;

pub(crate) trait Submarine {
    fn move_in_direction(&mut self, direction: Direction);
    fn finalize(&self) -> i32;
}

#[derive(Default)]
pub(crate) struct BasicSubmarine {
    x: i32,
    y: i32,
}

impl Submarine for BasicSubmarine {
    fn move_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Forward(magnitude) => self.x += magnitude,
            Direction::Up(magnitude) => self.y -= magnitude,
            Direction::Down(magnitude) => self.y += magnitude,
        }
    }

    fn finalize(&self) -> i32 {
        self.x * self.y
    }
}

#[derive(Default)]
pub(crate) struct SubmarineWithAim {
    x: i32,
    y: i32,
    aim: i32,
}

impl Submarine for SubmarineWithAim {
    fn move_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Forward(magnitude) => {
                self.x += magnitude;
                self.y += self.aim * magnitude;
            }
            Direction::Up(magnitude) => self.aim -= magnitude,
            Direction::Down(magnitude) => self.aim += magnitude,
        }
    }

    fn finalize(&self) -> i32 {
        self.x * self.y
    }
}

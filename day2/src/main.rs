use nom::Finish;

mod direction;
mod submarine;
use crate::direction::parse_directions;
use crate::submarine::{Submarine, SubmarineWithAim, BasicSubmarine};

fn main() {
    let input = include_str!("input.txt");
    let output = move_submarine::<BasicSubmarine>(input);
    println!("part 1 output {}", output);

    let output = move_submarine::<SubmarineWithAim>(input);
    println!("part 2 output {}", output);

    // Example of parsing the whole file at once
    let (_rem, output) = parse_directions(input).finish().unwrap();
    println!("List {:?}", output);
}

fn move_submarine<T>(input: &str) -> i32 where T: Submarine + Default {
    let submarine = input
        .lines()
        .fold(T::default(), |mut submarine, line| {
            let direction = line.parse().unwrap();
            submarine.move_in_direction(direction);
            submarine
        });
    submarine.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submarine() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(move_submarine(input), 150);
    }

    #[test]
    fn submarine_with_aim() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(move_submarine_with_aim(input), 900);
    }
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, i32, newline},
    error::Error,
    multi::separated_list1,
    Finish, IResult,
};
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let output = move_submarine(input);
    println!("part 1 output {}", output);

    // Example of parsing the whole file at once
    let (_rem, output) = separated_list1(newline, parse_direction)(input)
        .finish()
        .unwrap();
    println!("List {:?}", output);
}

fn move_submarine(input: &str) -> i32 {
    let submarine = input
        .lines()
        .fold(Submarine::default(), |mut submarine, line| {
            let direction = line.parse().unwrap();
            submarine.move_in_direction(direction);
            submarine
        });
    submarine.finalize()
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, direction) = alt((tag("forward"), tag("down"), tag("up")))(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, magnitude) = i32(input)?;
    let result = match direction {
        "forward" => Direction::Forward(magnitude),
        "up" => Direction::Up(magnitude),
        "down" => Direction::Down(magnitude),
        _ => panic!("invalid"),
    };
    Ok((input, result))
}

#[derive(Debug)]
enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Direction {
    type Err = Error<String>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match parse_direction(input).finish() {
            Ok((_remaining, direction)) => Ok(direction),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Default)]
struct Submarine {
    x: i32,
    y: i32,
}

impl Submarine {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(move_submarine(input), 150);
    }
}

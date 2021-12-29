use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, i32},
    error::Error,
    Finish, IResult,
};
#[cfg(feature = "full-direction-parse")]
use nom::{character::complete::newline, multi::separated_list1};

use std::str::FromStr;

pub(crate) fn parse_direction(input: &str) -> IResult<&str, Direction> {
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

#[cfg(feature = "full-direction-parse")]
pub(crate) fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    // Example of parsing the whole file at once
    separated_list1(newline, parse_direction)(input)
}

#[derive(Debug)]
pub(crate) enum Direction {
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

use std::{ops::Div, str::FromStr};

use advent_of_code::advent_stdlib::parse_row_input_as_data_array;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_row_input_as_data_array::<Rotation>(input);
    let mut safe = Safe::new();

    let mut result = 0;
    for rotation in data {
        if safe.rotate_part1(rotation) {
            result += 1
        };
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_row_input_as_data_array::<Rotation>(input);
    let mut safe = Safe::new();

    let mut result: u64 = 0;
    for rotation in data {
        result += safe.rotate_part2(rotation);
    }

    Some(result)
}

struct Safe {
    dials: i32,
}

impl Safe {
    fn new() -> Self {
        Safe { dials: 50 }
    }

    fn rotate_part1(&mut self, to: Rotation) -> bool {
        self.dials += to.value;
        self.dials %= 100;

        self.dials == 0
    }

    fn rotate_part2(&mut self, to: Rotation) -> u64 {
        // calculate distance to zero in rotation direction + cycles count
        // https://www.reddit.com/r/adventofcode/comments/1pb3y8p/comment/nro19r6

        // My initial idea with just div was crap

        let mut distance_to_zero = if to.value < 0 {
            self.dials
        } else {
            (100 - self.dials) % 100
        };

        self.dials += to.value;
        self.dials = self.dials.rem_euclid(100);

        // if now on zero - distance to 0 is 100
        if distance_to_zero == 0 {
            distance_to_zero = 100;
        }

        let rotation_delta = to.value.abs();

        // if Rotation pass 0
        if rotation_delta >= distance_to_zero {
            // pass 0 once + number of cycles
            return (1 + (rotation_delta - distance_to_zero).div(100))
                .try_into()
                .unwrap();
        }

        0
    }
}

#[derive(Debug)]
struct Rotation {
    value: i32,
}

#[derive(Debug)]
pub enum ParseRotationError {
    InvalidFormat,
    InvalidNumber(std::num::ParseIntError),
}

impl FromStr for Rotation {
    type Err = ParseRotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, value_str) = s.split_at(1);

        let value: i32 = value_str
            .parse()
            .map_err(ParseRotationError::InvalidNumber)?;

        match direction {
            "L" => Ok(Rotation { value: -value }),
            "R" => Ok(Rotation { value }),
            _ => Err(ParseRotationError::InvalidFormat),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1.txt",
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two_big_rot() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "test_big_rot.txt",
        ));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two_big_rot2() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "test_big_rot2.txt",
        ));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two_stopping_at_zero() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "stopping_at_zero.txt",
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_stopping_at_zero2() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "stopping_at_zero2.txt",
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_starting_at_zero() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "starting_at_zero.txt",
        ));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1.txt",
        ));
        assert_eq!(result, Some(6));
    }
}

use std::str::FromStr;

use advent_of_code::advent_stdlib::{Matrix, parse_row_input_as_data_array_matrix};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.trim_end().lines();
    let first_args: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let second_args: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let third_args: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let fourth_args: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let ops: Vec<Operation> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut total: u64 = 0;

    for i in 0..ops.len() {
        total += ops[i].act(&[first_args[i], second_args[i], third_args[i], fourth_args[i]]);
    }

    Some(total)
}

pub fn part_one_3_args(input: &str) -> Option<u64> {
    let mut lines = input.trim_end().lines();
    let first_args: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let second_args: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let third_args: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let ops: Vec<Operation> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut total: u64 = 0;

    for i in 0..ops.len() {
        total += ops[i].act(&[first_args[i], second_args[i], third_args[i]]);
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_row_input_as_data_array_matrix::<char>(input);
    let matrix = Matrix {
        data: data.0,
        size: data.1,
    };

    //matrix.print();

    // step 1 - get widths
    let width_list = column_widths(input);

    //dbg!(width_list[0]);
    //dbg!(width_list[1]);
    //dbg!(width_list[2]);

    // step 2 - calc!
    let mut total_result = 0;
    let mut problem_offset = 0;
    for problem_width in width_list {
        let mut operands: Vec<u64> = vec![];
        for x in (problem_offset..problem_offset + problem_width).rev() {
            let mut operand: u64 = 0;
            for y in 0..matrix.size.y - 1 {
                if matrix[y][x] == ' ' {
                    continue;
                }
                operand = operand * 10 + matrix[y][x].to_digit(10).unwrap() as u64
            }
            operands.push(operand);
        }
        //dbg!(matrix.size.y - 1);
        //dbg!(problem_offset);
        let op: Operation = matrix[matrix.size.y - 1][problem_offset]
            .to_string()
            .parse()
            .unwrap();
        total_result += op.act(&operands);
        problem_offset += problem_width + 1;

        //dbg!(&operands);
        //dbg!(&op);
        //dbg!(total_result);
    }

    //dbg!(width_list);

    Some(total_result)
}

fn column_widths(input: &str) -> Vec<usize> {
    let mut widths: Vec<usize> = Vec::new();

    for line in input.lines() {
        let cols: Vec<&str> = line.split_whitespace().collect();

        for (i, col) in cols.iter().enumerate() {
            if widths.len() <= i {
                widths.push(col.len());
            } else {
                widths[i] = widths[i].max(col.len());
            }
        }
    }

    widths
}

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn act(&self, operands: &[u64]) -> u64 {
        match self {
            Operation::Add => operands.iter().sum::<u64>(),
            Operation::Mul => operands.iter().product::<u64>(),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat,
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Mul),
            _ => {
                dbg!(s);
                Err(ParseError::InvalidFormat)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_3_args(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1.txt",
        ));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1.txt",
        ));
        assert_eq!(result, Some(3263827));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_2.txt",
        ));
        assert_eq!(result, Some(9580 + 7238619 + 6325));
    }
}

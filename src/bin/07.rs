use std::fmt::Display;

use advent_of_code::advent_stdlib::Matrix;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    // Just go from top to bottom, simulating beams.

    let mut matrix: Matrix<MapCell> = Matrix::from_char_input(input.trim_ascii_end());

    let mut result = 0;

    for y in 1..matrix.size.y {
        for x in 0..matrix.size.x {
            if matrix[y - 1][x].has_beam || matrix[y - 1][x].source {
                // beam incoming!
                if matrix[y][x].splitter {
                    let mut right_split = false;
                    let mut left_split = false;
                    if x >= 1 {
                        matrix[y][x - 1].has_beam = true;
                        left_split = true;
                    }
                    if x <= matrix.size.x - 2 {
                        matrix[y][x + 1].has_beam = true;
                        right_split = true;
                    }

                    if right_split && left_split {
                        result += 1;
                    }
                } else {
                    matrix[y][x].has_beam = true;
                }
            }
        }
    }

    //matrix.print();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Step 1 - top to bottom, simulation beams.
    // Step 2 - set beam cost at last layers to 1.
    //        - For each splitter s: Cost(s) = Cost(s_1) + Cost(s_2)
    //          where s_1 and s_2 - splitted beams
    //        - For each beam b Cost(b) = Cost(b'), where b' is beam below.

    let mut matrix: Matrix<MapCell> = Matrix::from_char_input(input.trim_ascii_end());

    let mut result = 0;

    for y in 1..matrix.size.y {
        for x in 0..matrix.size.x {
            if matrix[y - 1][x].has_beam || matrix[y - 1][x].source {
                // beam incoming!
                if matrix[y][x].splitter {
                    if x >= 1 {
                        matrix[y][x - 1].has_beam = true;
                    }
                    if x <= matrix.size.x - 2 {
                        matrix[y][x + 1].has_beam = true;
                    }
                } else {
                    matrix[y][x].has_beam = true;
                    if y == matrix.size.y - 1 {
                        // set last layer cost to 1
                        matrix[y][x].cost = 1;
                    }
                }
            }
        }
    }

    for y in (0..matrix.size.y - 1).rev() {
        for x in 0..matrix.size.x {
            if matrix[y][x].has_beam {
                matrix[y][x].cost = matrix[y + 1][x].cost;
            } else if matrix[y][x].splitter {
                let mut splitter_cost = 0;
                if x >= 1 {
                    splitter_cost += matrix[y + 1][x - 1].cost;
                }
                if x <= matrix.size.x - 2 {
                    splitter_cost += matrix[y + 1][x + 1].cost;
                }
                matrix[y][x].cost = splitter_cost;
            } else if matrix[y][x].source {
                result = matrix[y + 1][x].cost;
            }
        }
    }

    //matrix.print();

    Some(result)
}

#[derive(Debug, Clone)]
pub struct MapCell {
    pub source: bool,
    pub has_beam: bool,
    pub splitter: bool,
    pub cost: u64,
}

impl MapCell {
    pub fn new() -> Self {
        MapCell {
            source: false,
            has_beam: false,
            splitter: false,
            cost: 0,
        }
    }

    pub fn new_source() -> Self {
        MapCell {
            source: true,
            has_beam: false,
            splitter: false,
            cost: 0,
        }
    }

    pub fn new_splitter() -> Self {
        MapCell {
            source: false,
            has_beam: false,
            splitter: true,
            cost: 0,
        }
    }
}

impl Default for MapCell {
    fn default() -> Self {
        Self::new()
    }
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            '.' => MapCell::new(),
            '^' => MapCell::new_splitter(),
            'S' => MapCell::new_source(),
            _ => panic!("Unknown char in map data!"),
        }
    }
}

impl Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ch = '.';
        if self.source {
            ch = 'S'
        } else if self.cost != 0 {
            ch = char::from_digit((self.cost % 10) as u32, 10).unwrap()
        } else if self.has_beam {
            ch = '|'
        } else if self.splitter {
            ch = '^'
        }
        write!(f, "{}", ch)
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1.txt",
        ));
        assert_eq!(result, Some(40));
    }
}

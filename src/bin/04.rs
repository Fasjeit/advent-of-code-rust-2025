advent_of_code::solution!(4);

use advent_of_code::advent_stdlib::{Index, Matrix};
use std::{collections::VecDeque, fmt::Display};

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = Matrix::<MapCell>::from_char_input(input.trim_end());

    for position in 0..map.data.len() {
        let index = map.get_index_from_position(position);
        if !map[index].has_roll {
            continue;
        }

        let roll_neighbors = get_neighbors_count(index, &map);

        if roll_neighbors < 4 {
            map[index].pickable = true;
        }
    }

    //map.print();

    let res = map.data.iter().filter(|c| c.pickable).count();
    Some(res as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    part_two_queue(input)
}

fn part_two_queue(input: &str) -> Option<u64> {
    // using queue for adding to_be_removed cells
    // https://www.reddit.com/r/adventofcode/comments/1pdx284/2025_day_4_part_2_python_walkthrough_of_bfs/

    let mut map = Matrix::<MapCell>::from_char_input(input.trim_end());
    let mut queue = VecDeque::new();
    let mut removed = 0;

    // Add nods to exclude to VecDeque
    for position in 0..map.data.len() {
        let index = map.get_index_from_position(position);
        if !map[index].has_roll {
            continue;
        }

        let roll_neighbors = get_neighbors_count(index, &map);

        if roll_neighbors < 4 {
            queue.push_back(index);
        }
    }

    // process VecDeque
    while let Some(index) = queue.pop_front() {
        if !map[index].has_roll {
            continue;
        }
        map[index].has_roll = false;

        removed += 1;

        for to_remove_roll_index in get_neighbouring_rolls(index, &map) {
            queue.push_back(to_remove_roll_index);
        }

        //map.print();
        //println!();
    }

    Some(removed)
}

#[allow(dead_code)]
fn part_two_bruteforce(input: &str) -> Option<u64> {
    let mut map = Matrix::<MapCell>::from_char_input(input.trim_end());

    let mut removed_any = true;
    let mut res = 0;

    while removed_any {
        removed_any = false;
        for position in 0..map.data.len() {
            let index = map.get_index_from_position(position);
            if !map[index].has_roll {
                continue;
            }

            let roll_neighbors = get_neighbors_count(index, &map);

            if roll_neighbors < 4 {
                map[index].has_roll = false;
                res += 1;
                removed_any = true;
            }
        }

        //map.print();
    }
    Some(res as u64)
}

fn get_neighbouring_rolls(index: Index, map: &Matrix<MapCell>) -> Vec<Index> {
    // get neighbors to be excluded in next iteration
    let mut result = vec![];

    if let Some(up_index) = index.up(Some(map)) {
        if map[up_index].has_roll && get_neighbors_count(up_index, map) < 4 {
            result.push(up_index);
        }

        if let Some(urd_index) = up_index.right::<MapCell>(Some(map))
            && map[urd_index].has_roll
            && get_neighbors_count(urd_index, map) < 4
        {
            result.push(urd_index);
        }

        if let Some(uld_index) = up_index.left::<MapCell>(Some(map))
            && map[uld_index].has_roll
            && get_neighbors_count(uld_index, map) < 4
        {
            result.push(uld_index);
        }
    }
    if let Some(down_index) = index.down(Some(map)) {
        if map[down_index].has_roll && get_neighbors_count(down_index, map) < 4 {
            result.push(down_index);
        }

        if let Some(drd_index) = down_index.right::<MapCell>(Some(map))
            && map[drd_index].has_roll
            && get_neighbors_count(drd_index, map) < 4
        {
            result.push(drd_index);
        }
        if let Some(dld_index) = down_index.left::<MapCell>(Some(map))
            && map[dld_index].has_roll
            && get_neighbors_count(dld_index, map) < 4
        {
            result.push(dld_index);
        }
    }
    if let Some(right_index) = index.right(Some(map))
        && map[right_index].has_roll
        && get_neighbors_count(right_index, map) < 4
    {
        result.push(right_index);
    }
    if let Some(left_index) = index.left(Some(map))
        && map[left_index].has_roll
        && get_neighbors_count(left_index, map) < 4
    {
        result.push(left_index);
    }

    result
}

fn get_neighbors_count(index: Index, map: &Matrix<MapCell>) -> u64 {
    let mut roll_neighbors = 0;

    if let Some(up_index) = index.up(Some(map)) {
        if map[up_index].has_roll {
            roll_neighbors += 1;
        }

        if let Some(urd_index) = up_index.right::<MapCell>(Some(map))
            && map[urd_index].has_roll
        {
            roll_neighbors += 1;
        }

        if let Some(uld_index) = up_index.left::<MapCell>(Some(map))
            && map[uld_index].has_roll
        {
            roll_neighbors += 1;
        }
    }
    if let Some(down_index) = index.down(Some(map)) {
        if map[down_index].has_roll {
            roll_neighbors += 1;
        }

        if let Some(drd_index) = down_index.right::<MapCell>(Some(map))
            && map[drd_index].has_roll
        {
            roll_neighbors += 1;
        }
        if let Some(dld_index) = down_index.left::<MapCell>(Some(map))
            && map[dld_index].has_roll
        {
            roll_neighbors += 1;
        }
    }
    if let Some(right_index) = index.right(Some(map))
        && map[right_index].has_roll
    {
        roll_neighbors += 1;
    }
    if let Some(left_index) = index.left(Some(map))
        && map[left_index].has_roll
    {
        roll_neighbors += 1;
    }

    roll_neighbors
}

#[derive(Debug, Clone)]
pub struct MapCell {
    pub has_roll: bool,
    pub pickable: bool,
}

impl MapCell {
    pub fn new(has_roll: bool) -> Self {
        MapCell {
            has_roll,
            pickable: false,
        }
    }
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            '.' => MapCell::new(false),
            '@' => MapCell::new(true),
            _ => panic!("Unknown char in map data!"),
        }
    }
}

impl Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch;
        if self.pickable {
            ch = 'x'
        } else if self.has_roll {
            ch = '@'
        } else {
            ch = '.'
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1.txt",
        ));
        assert_eq!(result, Some(43));
    }

    #[test]
    fn test_part_two_bruteforce() {
        let result = part_two_bruteforce(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1.txt",
        ));
        assert_eq!(result, Some(43));
    }
}

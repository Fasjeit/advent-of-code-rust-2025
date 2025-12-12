use std::vec;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    // Troll task...
    // Checking only shapes area is enough!!!
    // No hard DLX instances with overlapping!
    // see https://www.reddit.com/r/adventofcode/comments/1pkje0o/comment/ntlkg9i
    // and https://www.reddit.com/r/adventofcode/comments/1pkje0o/comment/ntllrox/

    let shape_count = 6;

    let mut parts = input.trim_end().split("\n\n");

    let mut shape_sizes: Vec<u64> = vec![];
    for _ in 0..shape_count {
        let shape = parts.next().unwrap();
        shape_sizes.push(
            shape
                .chars()
                .fold(0, |acc, c| if c == '#' { acc + 1 } else { acc }),
        );
    }

    let mut result = 0;
    let regions = parts.next().unwrap().lines();
    for region in regions {
        let mut region_split = region.split(": ");
        let size_string = region_split.next().unwrap();
        let mut size_split = size_string.split('x');
        //dbg!(&size_string);
        let size_x: u64 = size_split.next().unwrap().parse().unwrap();
        let size_y: u64 = size_split.next().unwrap().parse().unwrap();
        let area = size_x * size_y;

        let requirements = region_split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap());
        let mut required_area = 0;
        for (i, r) in requirements.enumerate() {
            //dbg!(&r);
            //dbg!(&shape_sizes[i]);
            required_area += r * shape_sizes[i];
            //dbg!(&required_area);
        }

        if required_area <= area {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1.txt",
        ));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_2.txt",
        ));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1.txt",
        ));
        assert_eq!(result, None);
    }
}

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<Vec<u32>> = input
        .trim_end()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let res = lines.iter().fold(0, |acc, l| acc + find_max_for_line(l));

    Some(res as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<Vec<u64>> = input
        .trim_end()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect();

    let res = lines
        .iter()
        .fold(0, |acc, l| acc + find_max_for_line_iterative(l, 12));

    Some(res)
}

fn find_max_for_line(line: &[u32]) -> u32 {
    // find biggest digit - and biggest digit after it

    // NB. First digit cannot be the last in array
    let mut first_digit = 0;
    let mut first_index = 0;
    for (index, digit) in line[..line.len() - 1].iter().enumerate() {
        if *digit > first_digit {
            first_digit = *digit;
            first_index = index;
        }
    }

    let mut second_digit = 0;
    for digit in line[first_index + 1..line.len()].iter() {
        if *digit > second_digit {
            second_digit = *digit;
        }
    }

    first_digit * 10 + second_digit
}

#[allow(dead_code)]
fn find_max_for_line_recursive(line: &[u64], start_index: usize, count: usize) -> u64 {
    // find biggest digit - and biggest digit after it

    // NB. First digit cannot be in the last `count` in array
    let mut current_digit = 0;
    let mut current_index = 0;
    for (index, digit) in line[start_index..line.len() - (count - 1)]
        .iter()
        .enumerate()
    {
        if *digit > current_digit {
            current_digit = *digit;
            current_index = index + start_index;
        }
    }

    // recursive call
    if count == 1 {
        current_digit
    } else {
        10_u64.pow(count as u32 - 1) * current_digit
            + find_max_for_line_recursive(line, current_index + 1, count - 1)
    }
}

fn find_max_for_line_iterative(line: &[u64], mut count: usize) -> u64 {
    // find biggest digit - and biggest digit after it

    let mut result = 0;
    let mut slice_index = 0;

    while count > 0 {
        // Current max digit
        let mut current_digit = 0;

        // Current digit index in line
        let mut current_index = 0;

        // NB. First digit cannot be in the last `count` in array
        //
        // At iteration `current_index` we already found max for left part,
        // so we start at `current_index`.
        for (index, digit) in line[slice_index..line.len() - (count - 1)]
            .iter()
            .enumerate()
        {
            if *digit > current_digit {
                current_digit = *digit;
                // Index in line = index in slice + slice_index
                current_index = index + slice_index;
            }
        }

        // starting next iteration at next position
        slice_index = current_index + 1;

        if count == 1 {
            result += current_digit;
        } else {
            result += 10_u64.pow(count as u32 - 1) * current_digit;
        }
        count -= 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_for_line_1() {
        let result = find_max_for_line(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]);
        assert_eq!(result, 92);
    }

    #[test]
    fn test_max_for_line_2() {
        let result = find_max_for_line(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]);
        assert_eq!(result, 89);
    }

    #[test]
    fn test_max_for_line_3() {
        let result = find_max_for_line(&[9, 9, 1, 1, 1, 1, 8, 8]);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_max_for_line_4() {
        let result = find_max_for_line(&[6, 2, 1, 1, 1, 1, 8, 8, 8, 9]);
        assert_eq!(result, 89);
    }

    #[test]
    fn test_max_for_line_rec_1() {
        let result = find_max_for_line_iterative(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 2);
        assert_eq!(result, 92);
    }

    #[test]
    fn test_max_for_line_rec_2() {
        let result = find_max_for_line_iterative(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 2);
        assert_eq!(result, 89);
    }

    #[test]
    fn test_max_for_line_rec_3() {
        let result = find_max_for_line_iterative(&[9, 9, 1, 1, 1, 1, 8, 8], 2);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_max_for_line_rec_4() {
        let result = find_max_for_line_iterative(&[6, 2, 1, 1, 1, 1, 8, 8, 8, 9], 2);
        assert_eq!(result, 89);
    }

    #[test]
    fn test_max_for_line_rec_5() {
        let result =
            find_max_for_line_iterative(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12);
        assert_eq!(result, 811111111119);
    }

    #[test]
    fn test_max_for_line_rec_6() {
        let result =
            find_max_for_line_iterative(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12);
        assert_eq!(result, 987654321111);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1.txt",
        ));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1.txt",
        ));
        assert_eq!(result, Some(3121910778619));
    }
}

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges_str: Vec<&str> = input.trim_end().split(',').collect();
    let ranges = ranges_str.iter().map(|s| {
        let mut split = s.split('-');
        Range {
            start_index: split.next().unwrap().parse().unwrap(),
            end_index: split.next().unwrap().parse().unwrap(),
        }
    });

    Some(ranges.fold(0, |acc, x| acc + count_invalid(x, test_invalid_part1_fast)))
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges_str: Vec<&str> = input.trim_end().split(',').collect();
    let ranges = ranges_str.iter().map(|s| {
        let mut split = s.split('-');
        Range {
            start_index: split.next().unwrap().parse().unwrap(),
            end_index: split.next().unwrap().parse().unwrap(),
        }
    });

    Some(ranges.fold(0, |acc, x| acc + count_invalid(x, test_invalid_part2_fast)))
}

fn count_invalid(range: Range, test_invalid: fn(u64) -> bool) -> u64 {
    let mut res = 0;
    for i in range.start_index..range.end_index + 1 {
        if test_invalid(i) {
            res += i;
        }
    }

    res
}

#[allow(dead_code)]
fn test_invalid_part1(input: u64) -> bool {
    let digits = get_digits(input);
    if !digits.len().is_multiple_of(2) {
        return false;
    }

    let median = digits.len() / 2;
    for i in 0..median {
        if digits[i] != digits[i + median] {
            return false;
        }
    }
    true
}

fn test_invalid_part1_fast(input: u64) -> bool {
    // see test_invalid_part2_fast for details
    // only start-end patterns
    match 1 + input.ilog10() {
        1 => false,
        2 => input.is_multiple_of(11),
        3 => false,
        4 => input.is_multiple_of(101),
        5 => false,
        6 => input.is_multiple_of(1001),
        7 => false,
        8 => input.is_multiple_of(10001),
        9 => false,
        10 => input.is_multiple_of(100001),
        _ => panic!(),
    }
}

#[allow(dead_code)]
fn test_invalid_part2(input: u64) -> bool {
    let digits = get_digits(input);

    // test every `part` digits

    // ex part_len = 2
    // 1 2 3 4 5 6
    // test 1 == 3 == 6 (part index 1, 2)
    // and
    // test 2 == 4 == 6 (part index 1, 2)

    'outer: for pattern_len in 1..digits.len() {
        //dbg!(&pattern_len);
        if !digits.len().is_multiple_of(pattern_len) {
            continue;
        }

        for pattern_index in 0..pattern_len {
            //dbg!(&i);
            // check every `pattern_len` digit
            for part_index in 1..digits.len() / pattern_len {
                if digits[pattern_index] != digits[pattern_index + part_index * pattern_len] {
                    continue 'outer;
                }
            }
        }

        return true;
    }

    false
}

fn test_invalid_part2_fast(input: u64) -> bool {
    // https://www.reddit.com/r/adventofcode/comments/1pc1ms7/comment/nruo03u/
    // Clever idea just to check multiple of some numbers depending on length
    // 11..11 is just the same number
    //
    // len n input gives all divisors of n as possible len patterns.
    // len 1 pattern is also always some len n>1 pattern,
    // so no need to include it explicitly if any other exists
    //
    // ex
    // 12 × (0)10101 = 121212 (len 6)
    // 131 × (0)(0)1001 = 131131 (len 6)
    // 11 × (0)10101 = 111111 (len 6, len 1 pattern included)
    //
    // 10 * 12 × (00)1001 = 120120 (len 6)
    // 9 * 12 × (00)1001 = 108108 (len 6)
    //
    // 4 × 11 = 44 (len 2)

    match 1 + input.ilog10() {
        1 => false,
        2 => input.is_multiple_of(11),
        3 => input.is_multiple_of(111),
        4 => input.is_multiple_of(101), // 0101
        5 => input.is_multiple_of(11111),
        6 => input.is_multiple_of(1001) || input.is_multiple_of(10101), // 001001 or 010101 (3 or 2 len patterns)
        7 => input.is_multiple_of(1111111),
        8 => input.is_multiple_of(1010101) || input.is_multiple_of(10001), // 01010101 or 00010001 (2 or 4 len patterns)
        9 => input.is_multiple_of(1001001), // 001001001 (3 len patterns)
        10 => input.is_multiple_of(101010101) || input.is_multiple_of(100001), // 0101010101 or 0000100001 (2 or 5 len patterns)
        _ => panic!(),
    }
}

fn get_digits(mut n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.into_iter().rev().collect()
}

struct Range {
    start_index: u64,
    end_index: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid1() {
        let result = test_invalid_part1(1212);
        assert!(result);
    }

    #[test]
    fn test_invalid2() {
        let result = test_invalid_part1(12012);
        assert!(!result);
    }

    #[test]
    fn test_invalid3() {
        let result = test_invalid_part1(124124);
        assert!(result);
    }

    #[test]
    fn test_invalid4() {
        let result = test_invalid_part1(1240124);
        assert!(!result);
    }

    #[test]
    fn test_invalid1_fast() {
        let result = test_invalid_part1_fast(1212);
        assert!(result);
    }

    #[test]
    fn test_invalid2_fast() {
        let result = test_invalid_part1_fast(12012);
        assert!(!result);
    }

    #[test]
    fn test_invalid3_fast() {
        let result = test_invalid_part1_fast(124124);
        assert!(result);
    }

    #[test]
    fn test_invalid4_fast() {
        let result = test_invalid_part1_fast(1240124);
        assert!(!result);
    }

    #[test]
    fn test_invalid5_fast() {
        let result = test_invalid_part1_fast(131131);
        assert!(result);
    }

    #[test]
    fn test_invalid_part2_1() {
        let result = test_invalid_part2(1212);
        assert!(result);
    }

    #[test]
    fn test_invalid_part2_2() {
        let result = test_invalid_part2(12012);
        assert!(!result);
    }

    #[test]
    fn test_invalid_part2_3() {
        let result = test_invalid_part2(124124);
        assert!(result);
    }

    #[test]
    fn test_invalid_part2_4() {
        let result = test_invalid_part2(1240124);
        assert!(!result);
    }

    #[test]
    fn test_invalid_part2_5() {
        let result = test_invalid_part2(111);
        assert!(result);
    }

    #[test]
    fn test_invalid_part2_6() {
        let result = test_invalid_part2(123123123);
        assert!(result);
    }

    #[test]
    fn test_invalid_part2_7() {
        let result = test_invalid_part2(1212121212);
        assert!(result);
    }

    #[test]
    fn test_invalid_part2_8() {
        let result = test_invalid_part2(100);
        assert!(!result);
    }

    #[test]
    fn test_invalid_part2_9() {
        let result = test_invalid_part2(1011);
        assert!(!result);
    }

    #[test]
    fn test_invalid_part2_10() {
        let result = test_invalid_part2(9999);
        assert!(result);
    }

    #[test]
    fn test_invalid_part2_11() {
        let result = test_invalid_part2(1188511880);
        assert!(!result);
    }

    #[test]
    fn test_invalid_part2_1_fast() {
        let result = test_invalid_part2_fast(1212);
        assert!(result);
    }

    #[test]
    fn test_invalid_part2_2_fast() {
        let result = test_invalid_part2_fast(12012);
        assert!(!result);
    }

    #[test]
    fn test_invalid_part2_3_fast() {
        let result = test_invalid_part2_fast(124124);
        assert!(result);
    }

    #[test]
    fn test_invalid_part2_4_fast() {
        let result = test_invalid_part2_fast(1240124);
        assert!(!result);
    }

    #[test]
    fn test_invalid_part2_5_fast() {
        let result = test_invalid_part2_fast(111);
        assert!(result);
    }

    #[test]
    fn test_invalid_part2_6_fast() {
        let result = test_invalid_part2_fast(123123123);
        assert!(result);
    }

    #[test]
    fn test_invalid_part2_7_fast() {
        let result = test_invalid_part2_fast(1212121212);
        assert!(result);
    }

    #[test]
    fn test_invalid_part2_8_fast() {
        let result = test_invalid_part2_fast(100);
        assert!(!result);
    }

    #[test]
    fn test_invalid_part2_9_fast() {
        let result = test_invalid_part2_fast(1011);
        assert!(!result);
    }

    #[test]
    fn test_invalid_part2_10_fast() {
        let result = test_invalid_part2_fast(9999);
        assert!(result);
    }

    #[test]
    fn test_invalid_part2_11_fast() {
        let result = test_invalid_part2_fast(1188511880);
        assert!(!result);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1.txt",
        ));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1.txt",
        ));
        assert_eq!(result, Some(4174379265));
    }
}

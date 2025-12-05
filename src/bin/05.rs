use std::cmp::{max, min};
use std::collections::VecDeque;
use std::fmt;
use std::str::FromStr;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut splitted_input = input.trim_end().split("\n\n");
    let ranges_input = splitted_input.next().unwrap().lines();
    let ids_input = splitted_input.next().unwrap().lines();

    let ranges: Vec<Range> = ranges_input.map(|r| Range::from_str(r).unwrap()).collect();
    let numbers: Vec<u64> = ids_input.map(|n| n.parse().unwrap()).collect();

    let mut result = 0;

    'number_loop: for n in &numbers {
        for r in &ranges {
            if r.number_in_range(*n) {
                result += 1;
                continue 'number_loop;
                //dbg!(*n);
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    /*
    Sort the list of intervals in ascending order based on their start values.
    Create an empty list `merged_intervals` to store the resulting non-overlapping intervals.

    Iterate through the sorted intervals.
    For each current interval:
        Empty merged_intervals:
            Append the current interval directly to merged_intervals.
        No Overlap:
            If the current interval's start is greater than the
            of the last interval in merged_intervals, there is no overlap.
            Append the current interval directly to merged_intervals.
        Overlap:
            If the current interval overlaps with the last interval
            in merged_intervals, merge them.
            Add merged interval in merged_intervals
    */

    let mut splitted_input = input.trim_end().split("\n\n");
    let ranges_input = splitted_input.next().unwrap().lines();

    let mut ranges: Vec<Range> = ranges_input.map(|r| Range::from_str(r).unwrap()).collect();

    // sort ranges by start index
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    //dbg!(&ranges);

    let mut merged_intervals = VecDeque::<Range>::new();

    for range in ranges {
        // empty merged_intervals

        match merged_intervals.pop_back() {
            None => {
                merged_intervals.push_back(range);
            }
            Some(last_merged) => {
                // range.start always >= all merged starts, so need to check only ends
                if range.start <= last_merged.end {
                    // intersects!
                    // merge
                    merged_intervals.push_back(range.union(&last_merged));
                } else {
                    // push back last merged and current one
                    merged_intervals.push_back(last_merged);
                    merged_intervals.push_back(range);
                }
            }
        }
    }

    let result = merged_intervals
        .iter()
        .fold(0, |acc, r| acc + r.end - r.start + 1);

    Some(result)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn number_in_range(&self, number: u64) -> bool {
        number >= self.start && number <= self.end
    }

    fn union(&self, other: &Range) -> Range {
        let start = min(self.start, other.start);
        let end = max(self.end, other.end);

        Range { start, end }
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat,
    InvalidChars,
}

impl FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spitted = s.split('-');
        let start = spitted.next().unwrap().parse().unwrap();
        let end = spitted.next().unwrap().parse().unwrap();
        Ok(Range { start, end })
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}-{1}", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_1() {
        let a = Range { start: 2, end: 3 };
        let b = Range { start: 3, end: 4 };

        assert_eq!(a.union(&b), Range { start: 2, end: 4 });
    }

    #[test]
    fn test_union_2() {
        let a = Range { start: 5, end: 7 };
        let b = Range { start: 1, end: 6 };

        assert_eq!(a.union(&b), Range { start: 1, end: 7 });
    }

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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1.txt",
        ));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_2.txt",
        ));
        assert_eq!(result, Some(6));
    }
}

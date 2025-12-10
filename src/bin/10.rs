use std::{
    collections::{HashMap, VecDeque},
    env::current_exe,
    fmt::Display,
    panic::RefUnwindSafe,
};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.trim_end().lines();
    let mut result = 0;
    for (i, l) in lines.enumerate() {
        let mut splitted_1 = l.split("]");
        let state = splitted_1.next().unwrap();
        let mut machine = Machine::from(state);
        // println!("{}", machine);
        // println!("{}", machine.get_score());
        // machine.push_button(&Button {
        //     states_to_change: vec![0, 1, 2],
        // });
        // println!("{}", machine);
        // println!("{}", machine.get_score());

        let mut splitted_2 = splitted_1.next().unwrap().trim_end().split("{");
        let buttons: Vec<Button> = splitted_2
            .next()
            .unwrap()
            .split_whitespace()
            .map(Button::from)
            .collect();

        let mut visited = HashMap::new();
        bfs(
            &machine.current_state,
            &machine.required_state,
            &buttons,
            &mut visited,
        );

        //dbg!(visited.get(&machine.required_state));.
        println!("{} / 193", i + 1);

        result += visited.get(&machine.required_state).unwrap();
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn bfs(
    initial_state: &Vec<bool>,
    target_state: &Vec<bool>,
    buttons: &Vec<Button>,
    visited: &mut HashMap<Vec<bool>, u64>,
) {
    let mut to_visit = VecDeque::new();
    to_visit.push_back((initial_state.to_vec(), 0_u64));

    while let Some((current_state, current_depth)) = to_visit.pop_front() {
        visited.insert(current_state.to_vec(), current_depth);
        let next_depth = current_depth + 1;

        //dbg!(&current_state);
        //dbg!(current_depth);
        //dbg!(&visited);

        for button in buttons {
            //dbg!(&button);

            let mut next_state = current_state.to_vec();

            button
                .states_to_change
                .iter()
                .for_each(|s| next_state[*s] = !current_state[*s]);

            //dbg!(&next_state);

            if let Some(visited_depth) = visited.get(&next_state)
                && *visited_depth <= next_depth
            {
                //dbg!("Already visited STATE with same of better depth");
                continue;
            } else if get_score(&next_state, target_state) != next_state.len() as u64 {
                to_visit.push_back((next_state, next_depth));
            } else {
                // If already reached target state - no need to traverse further, just save
                visited.insert(next_state.to_vec(), next_depth);
            }
        }
    }
}

fn get_score(current_state: &Vec<bool>, target_state: &Vec<bool>) -> u64 {
    target_state.iter().enumerate().fold(
        0,
        |acc, (i, x)| {
            if current_state[i] == *x { acc + 1 } else { acc }
        },
    )
}

#[derive(Debug)]
struct Button {
    states_to_change: Vec<usize>,
}

impl From<&str> for Button {
    fn from(value: &str) -> Self {
        let mut states_to_change: Vec<usize> = vec![];
        for char in value.chars() {
            match char {
                '(' => continue,
                ',' => continue,
                ')' => break,
                c => states_to_change.push(c.to_digit(10).unwrap() as usize),
                _ => panic!(),
            }
        }

        Button { states_to_change }
    }
}

struct Machine {
    required_state: Vec<bool>,
    current_state: Vec<bool>,
}

impl Machine {
    fn get_score(&self) -> u64 {
        self.required_state
            .iter()
            .enumerate()
            .fold(0, |acc, (i, x)| {
                if self.current_state[i] == *x {
                    acc + 1
                } else {
                    acc
                }
            })
    }

    fn push_button(&self, button: &Button) -> Vec<bool> {
        let mut result = vec![];
        button
            .states_to_change
            .iter()
            .for_each(|s| result[*s] = !self.current_state[*s]);
        result
    }
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut required_state: Vec<bool> = vec![];
        for char in value.chars() {
            match char {
                '[' => continue,
                ']' => break,
                '.' => required_state.push(false),
                '#' => required_state.push(true),
                _ => panic!(),
            }
        }

        let state_len = required_state.len();
        Machine {
            required_state,
            current_state: vec![false; state_len],
        }
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current_state_string = "".to_string();
        self.current_state.iter().for_each(|c| match c {
            false => current_state_string += ".",
            true => current_state_string += "#",
        });

        let mut target_state_string = "".to_string();
        self.required_state.iter().for_each(|c| match c {
            false => target_state_string += ".",
            true => target_state_string += "#",
        });
        // state_string = state_string.chars().rev().collect();
        write!(
            f,
            "[{}] - req [{}]",
            current_state_string, target_state_string
        )
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_one_simple1() {
        let result = part_one(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1_simple1.txt",
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_simple3() {
        let result = part_one(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1_simple3.txt",
        ));
        assert_eq!(result, Some(2));
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

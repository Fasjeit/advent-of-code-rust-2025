use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.trim_end().lines();
    let mut result = 0;
    for l in lines {
        let machine = Machine::from(l);

        let mut visited = HashMap::new();
        let iteration_result = bfs(
            &vec![false; machine.required_state.len()],
            &machine.required_state,
            &machine.buttons,
            &mut visited,
        )
        .unwrap();

        //dbg!(visited.get(&machine.required_state));.
        //println!("{} / 193", i + 1);

        result += iteration_result;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    // I've just gave up here...
    // the solution below is from
    // https://www.reddit.com/r/adventofcode/comments/1pity70/comment/ntd2evv/
    // https://gist.github.com/icub3d/16eea2a8b4a94d193a148fef908779a9
    // https://www.youtube.com/watch?v=xibCHVRF6oI

    // Alternative to try later - https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/

    let lines = input.trim_end().lines();
    let mut result = 0;
    for l in lines {
        let machine = Machine::from(l);

        let matrix = Matrix::from_machine(&machine);

        // Now we can DFS over a much smaller solution space.
        let max = machine.joltage.target_state.iter().max().unwrap() + 1;
        let mut min = usize::MAX;
        let mut values = vec![0; matrix.independents.len()];

        dfs(&matrix, 0, &mut values, &mut min, max as usize);
        result += min;

        //println!("{} / 193", i + 1);
    }

    Some(result as u64)
}

fn bfs(
    initial_state: &Vec<bool>,
    target_state: &Vec<bool>,
    buttons: &Vec<Button>,
    visited: &mut HashMap<Vec<bool>, u64>,
) -> Option<u64> {
    // Check inputs
    if initial_state.len() != target_state.len() {
        return None;
    }

    // quick check: already at target
    if initial_state == target_state {
        visited.insert(initial_state.clone(), 0);
        return Some(0);
    }

    let mut to_visit = VecDeque::new();
    to_visit.push_back((initial_state.clone(), 0));

    while let Some((current_state, current_depth)) = to_visit.pop_front() {
        // If we have already seen the state with better or same depth - skip it
        if let Some(&prev_depth) = visited.get(&current_state)
            && prev_depth <= current_depth
        {
            continue;
        }

        // No better sate - save this one
        visited.insert(current_state.clone(), current_depth);
        let next_depth = current_depth + 1;

        //dbg!(&current_state);
        //dbg!(current_depth);
        //dbg!(&visited);

        for button in buttons {
            //dbg!(&button);

            // update the state to get next state
            let mut next_state = current_state.clone();
            for &idx in &button.states_to_change {
                debug_assert!(idx < next_state.len(), "button index out of range");
                next_state[idx] = !next_state[idx];
            }

            //dbg!(&next_state);

            // If we reached the target, return immediately. This is the shortest depth (as we in BFS).
            if next_state == *target_state {
                visited.insert(next_state.clone(), next_depth);
                return Some(next_depth);
            }

            // If we've visited it at an equal or better depth, skip enqueueing next state
            if let Some(&prev_depth) = visited.get(&next_state)
                && prev_depth <= next_depth
            {
                continue;
            }

            to_visit.push_back((next_state, next_depth));
        }
    }

    None
}

// Working for part 2, but too slow :(
#[allow(dead_code)]
fn bfs_2(
    initial_state: &Vec<u64>,
    target_state: &Vec<u64>,
    buttons: &Vec<Button>,
    visited: &mut HashMap<Vec<u64>, u64>,
) -> Option<u64> {
    // Check inputs
    if initial_state.len() != target_state.len() {
        return None;
    }

    // quick check: already at target
    if initial_state == target_state {
        visited.insert(initial_state.clone(), 0);
        return Some(0);
    }

    let mut to_visit = VecDeque::new();
    to_visit.push_back((initial_state.clone(), 0));

    while let Some((current_state, current_depth)) = to_visit.pop_front() {
        // If we have already seen the state with better or same depth - skip it
        if let Some(&prev_depth) = visited.get(&current_state)
            && prev_depth <= current_depth
        {
            continue;
        }

        // No better sate - save this one
        visited.insert(current_state.clone(), current_depth);
        let next_depth = current_depth + 1;

        //dbg!(&current_state);
        //dbg!(current_depth);
        //dbg!(&visited);

        'button_loop: for button in buttons {
            //dbg!(&button);

            // update the state to get next state
            let mut next_state = current_state.clone();
            for &idx in &button.states_to_change {
                debug_assert!(idx < next_state.len(), "button index out of range");
                next_state[idx] += 1;
            }

            //dbg!(&next_state);

            // If we reached the target, return immediately. This is the shortest depth (as we in BFS).
            if next_state == *target_state {
                visited.insert(next_state.clone(), next_depth);
                return Some(next_depth);
            }

            // If we've visited it at an equal or better depth, skip enqueueing next state
            if let Some(&prev_depth) = visited.get(&next_state)
                && prev_depth <= next_depth
            {
                continue;
            }

            //dbg!(target_state);
            //dbg!(&next_state);
            // Or if any index is already bigger - skip
            for i in 0..next_state.len() {
                if next_state[i] > target_state[i] {
                    //dbg!("skip!");
                    continue 'button_loop;
                }
            }

            to_visit.push_back((next_state, next_depth));
        }
    }

    None
}

struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
    dependents: Vec<usize>,
    independents: Vec<usize>,
}

impl Matrix {
    const EPSILON: f64 = 1e-9;

    // Make a matrix, do a Gaussian elimination and setup the fixed and free variables.
    fn from_machine(machine: &Machine) -> Self {
        let rows = machine.joltage.target_state.len();
        let cols = machine.buttons.len();
        let mut data = vec![vec![0.0; cols + 1]; rows];

        // Add all of our buttons.
        for (c, button) in machine.buttons.iter().enumerate() {
            for r in &button.states_to_change {
                if *r < rows {
                    data[*r][c] = 1.0;
                }
            }
        }

        // Add our joltages to the last column
        for (r, &val) in machine.joltage.target_state.iter().enumerate() {
            data[r][cols] = val as f64;
        }

        let mut matrix = Self {
            data,
            rows,
            cols,
            dependents: Vec::new(),
            independents: Vec::new(),
        };

        matrix.gaussian_elimination();
        matrix
    }

    // https://en.wikipedia.org/wiki/Gaussian_elimination
    fn gaussian_elimination(&mut self) {
        let mut pivot = 0;

        let mut col = 0;
        while pivot < self.rows && col < self.cols {
            // Find the best pivot row for this column.
            let (best_row, best_value) = self
                .data
                .iter()
                .enumerate()
                .skip(pivot)
                .map(|(r, row)| (r, row[col].abs()))
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            // If the best value is zero, this is a free variable.
            if best_value < Self::EPSILON {
                self.independents.push(col);
                col += 1;
                continue;
            }

            // Swap rows and mark this column as dependent.
            self.data.swap(pivot, best_row);
            self.dependents.push(col);

            // Normalize pivot row.
            let pivot_value = self.data[pivot][col];
            for val in &mut self.data[pivot][col..=self.cols] {
                *val /= pivot_value;
            }

            // Eliminate this column in all other rows.
            for r in 0..self.rows {
                if r != pivot {
                    let factor = self.data[r][col];
                    if factor.abs() > Self::EPSILON {
                        let pivot_row = self.data[pivot][col..=self.cols].to_vec();
                        self.data[r][col..=self.cols]
                            .iter_mut()
                            .zip(&pivot_row)
                            .for_each(|(val, &pivot_val)| {
                                *val -= factor * pivot_val;
                            });
                    }
                }
            }

            pivot += 1;
            col += 1;
        }

        // Any remaining columns are free variables
        self.independents.extend(col..self.cols);
    }

    // Check if the given values for our independent variables are valid. If so, return the total button presses.
    fn valid(&self, values: &[usize]) -> Option<usize> {
        // We start with how many times we've pressed the free variables.
        let mut total = values.iter().sum::<usize>();

        // Calculate dependent variable values based on independent variables.
        for row in 0..self.dependents.len() {
            // Calculate this dependent by subtracting the sum of the free variable pushes from the solution.
            let val = self
                .independents
                .iter()
                .enumerate()
                .fold(self.data[row][self.cols], |acc, (i, &col)| {
                    acc - self.data[row][col] * (values[i] as f64)
                });

            // We need non-negative, whole numbers for a valid solution.
            if val < -Self::EPSILON {
                return None;
            }
            let rounded = val.round();
            if (val - rounded).abs() > Self::EPSILON {
                return None;
            }

            total += rounded as usize;
        }

        Some(total)
    }
}

fn dfs(matrix: &Matrix, idx: usize, values: &mut [usize], min: &mut usize, max: usize) {
    // When we've assigned all independent variables, check if it's a valid solution.
    if idx == matrix.independents.len() {
        if let Some(total) = matrix.valid(values) {
            *min = (*min).min(total);
        }
        return;
    }

    // Try different values for the current independent variable.
    let total: usize = values[..idx].iter().sum();
    for val in 0..max {
        // Optimization: If we ever go above our min, we can't possibly do better.
        if total + val >= *min {
            break;
        }
        values[idx] = val;
        dfs(matrix, idx + 1, values, min, max);
    }
}

#[derive(Debug)]
struct Button {
    states_to_change: Vec<usize>,
}

#[derive(Debug)]
struct Joltage {
    target_state: Vec<u64>,
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
            }
        }

        Button { states_to_change }
    }
}

impl From<&str> for Joltage {
    fn from(value: &str) -> Self {
        let st: String = value.chars().filter(|c| *c != '{' && *c != '}').collect();
        let target_state: Vec<u64> = st.split(',').map(|s| s.parse().unwrap()).collect();

        Joltage { target_state }
    }
}

struct Machine {
    required_state: Vec<bool>,
    buttons: Vec<Button>,
    joltage: Joltage,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut splitted_1 = value.split("]");
        let state = splitted_1.next().unwrap();
        let mut splitted_2 = splitted_1.next().unwrap().trim_end().split("{");
        let buttons: Vec<Button> = splitted_2
            .next()
            .unwrap()
            .split_whitespace()
            .map(Button::from)
            .collect();
        let joltage = Joltage::from(splitted_2.next().unwrap());

        let mut required_state: Vec<bool> = vec![];
        for char in state.chars() {
            match char {
                '[' => continue,
                ']' => break,
                '.' => required_state.push(false),
                '#' => required_state.push(true),
                _ => panic!(),
            }
        }

        Machine {
            required_state,
            buttons,
            joltage,
        }
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut target_state_string = "".to_string();
        self.required_state.iter().for_each(|c| match c {
            false => target_state_string += ".",
            true => target_state_string += "#",
        });
        write!(f, "[{}]", target_state_string)
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
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part_two_simple1_full() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1_simple1_full.txt",
        ));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two_simple2_full() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1_simple2_full.txt",
        ));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two_simple3_full() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1_simple3_full.txt",
        ));
        assert_eq!(result, Some(11));
    }
}

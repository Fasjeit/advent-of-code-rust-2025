use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, true)
}

fn solve(input: &str, part_2: bool) -> Option<u64> {
    let inputs = input.trim_end().lines();

    let mut index_map: HashMap<usize, &str> = HashMap::new();
    let mut label_map: HashMap<&str, usize> = HashMap::new();
    let mut connection_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for (i, line) in inputs.enumerate() {
        let mut splitted_1 = line.split(": ");
        let label = splitted_1.next().unwrap();
        index_map.insert(i, label);
        label_map.insert(label, i);

        let connections: Vec<&str> = splitted_1.next().unwrap().split_whitespace().collect();

        connection_map.insert(label, connections);
    }

    // Explicitly set index for out
    let out_index = index_map.len();
    index_map.insert(out_index, "out");
    label_map.insert("out", out_index);

    let mut to_visit_set: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
    // do for each out
    to_visit_set.push(Reverse((0_u64, out_index)));

    //let mut cost_map = HashMap::new();

    let start_index = if !part_2 {
        label_map.get("you").unwrap()
    } else {
        label_map.get("svr").unwrap()
    };

    let paths = if !part_2 {
        let mut ways_map: HashMap<usize, u64> = HashMap::new();
        count_paths(
            &index_map,
            &label_map,
            &connection_map,
            &mut ways_map,
            *start_index,
            out_index,
        )
    } else {
        let mut state_cache: HashMap<(usize, bool, bool), u64> = HashMap::new();
        count_paths_part_2(
            &index_map,
            &label_map,
            &connection_map,
            &mut state_cache,
            *start_index,
            out_index,
            false,
            false,
        )
    };

    Some(paths)
}

fn count_paths(
    index_map: &HashMap<usize, &str>,
    label_map: &HashMap<&str, usize>,
    connection_map: &HashMap<&str, Vec<&str>>,
    ways_map: &mut HashMap<usize, u64>,
    current_index: usize,
    ending_position: usize,
) -> u64 {
    // If we reached the ending node
    if current_index == ending_position {
        return 1;
    }

    // Return cached result if exists
    if let Some(&cached) = ways_map.get(&current_index) {
        return cached;
    }

    let mut total = 0;
    let current_label = index_map.get(&current_index).unwrap();
    if let Some(neighbors) = connection_map.get(current_label) {
        for next_label in neighbors {
            let next_index = label_map.get(next_label).unwrap();
            total += count_paths(
                index_map,
                label_map,
                connection_map,
                ways_map,
                *next_index,
                ending_position,
            );
        }
    }

    ways_map.insert(current_index, total);
    total
}

#[allow(clippy::too_many_arguments)]
fn count_paths_part_2(
    index_map: &HashMap<usize, &str>,
    label_map: &HashMap<&str, usize>,
    connection_map: &HashMap<&str, Vec<&str>>,
    state_cache: &mut HashMap<(usize, bool, bool), u64>,
    current_index: usize,
    ending_position: usize,
    visited_dac: bool,
    visited_fft: bool,
) -> u64 {
    // If we reached the ending node
    if current_index == ending_position {
        //dbg!("1");
        //dbg!(visited_dac);
        //dbg!(visited_fft);
        // fft or gac is never last node, no check for current node here
        return if visited_dac && visited_fft { 1 } else { 0 };
    }

    // Return cached result if exists for current index and state
    let current_state = (current_index, visited_dac, visited_fft);
    if let Some(&cached) = state_cache.get(&current_state) {
        //dbg!("CACHE");
        //dbg!(visited_dac);
        //dbg!(visited_fft);
        return cached;
    }

    let mut total = 0;
    let current_label = index_map.get(&current_index).unwrap();

    let visited_dac = visited_dac || *current_label == "dac";
    let visited_fft = visited_fft || *current_label == "fft";

    if let Some(neighbors) = connection_map.get(current_label) {
        for next_label in neighbors {
            let next_index = label_map.get(next_label).unwrap();
            let result = count_paths_part_2(
                index_map,
                label_map,
                connection_map,
                state_cache,
                *next_index,
                ending_position,
                visited_dac,
                visited_fft,
            );
            total += result;
        }
    }
    //dbg!("Return");
    //dbg!(visited_dac);
    //dbg!(visited_fft);
    state_cache.insert(current_state, total);
    total
}

// fn Dijkstra(
//     index_map: HashMap<usize, &str>,
//     label_map: HashMap<&str, usize>,
//     connection_map: &HashMap<&str, Vec<&str>>,
//     cost_map: &mut HashMap<usize, u64>,
//     ways_map: &mut HashMap<usize, u64>,
//     to_visit_set: &mut BinaryHeap<Reverse<(u64, usize)>>,
//     ending_position: Option<&usize>,
// ) -> Option<(u64, u64)> {
//     // returns (cost, number of shortest paths)
//     let mut safe_counter = 100000;
//     while let Some(Reverse((cost, index))) = to_visit_set.pop() {
//         if safe_counter <= 0 {
//             panic!("Safe counter stop.");
//         }
//         safe_counter -= 1;

//         // Update cost for current cell
//         cost_map
//             .entry(index)
//             .and_modify(|e| *e = cost)
//             .or_insert(cost);

//         // Update ways: if no ways yet, set to 1
//         ways_map.entry(index).or_insert(1);

//         if let Some(ending_position) = ending_position
//             && index == *ending_position
//         {
//             return Some((cost, *ways_map.get(&index).unwrap()));
//         }

//         let current_label = index_map.get(&index).unwrap();
//         let next_labels = connection_map.get(current_label).unwrap();

//         for next_label in next_labels {
//             let next_index = label_map.get(next_label).unwrap();
//             let next_cost = cost + 1;

//             match cost_map.get(next_index) {
//                 Some(&existing_cost) if existing_cost < next_cost => {
//                     // already have shorter path, do nothing
//                 }
//                 Some(&existing_cost) if existing_cost == next_cost => {
//                     // found another shortest path → add ways
//                     let ways = ways_map.get(&index).unwrap();
//                     *ways_map.entry(*next_index).or_insert(0) += *ways;
//                 }
//                 _ => {
//                     // first time reaching this node or shorter path found
//                     cost_map.insert(*next_index, next_cost);
//                     let ways = ways_map.get(&index).unwrap();
//                     ways_map.insert(*next_index, *ways);
//                     to_visit_set.push(Reverse((next_cost, *next_index)));
//                 }
//             }
//         }
//     }

//     None
// }

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
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_2.txt",
        ));
        assert_eq!(result, Some(2));
    }
}

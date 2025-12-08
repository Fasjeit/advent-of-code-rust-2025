use std::{collections::HashMap, fmt::Display};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    part_one_for_iteration(input, 1000)
}

fn part_one_for_iteration(input: &str, iteration_count: usize) -> Option<u64> {
    let boxes: Vec<JunctionBox> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| JunctionBox::from(line, i))
        .collect();

    // Just compute all distances
    // The size of input is 1000 and we need 1000 connections, so
    // kd-tree finding k=100 closest neighbors is useless here.

    let mut edges: Vec<(usize, usize, f32)> = vec![];
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            edges.push((i, j, boxes[i].distance_to(boxes[j])));
        }
    }

    // Sort by distance ascending
    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    // for e in &edges {
    //     println!("[{}:{}] - [{}:{}]", e.0, boxes[e.0], e.1, boxes[e.1]);
    // }

    let mut dsu = UnionFind::new(boxes.len());
    let mut chosen_edges = Vec::new();

    for (iterations, &(a, b, dist)) in edges.iter().enumerate() {
        //println!("===> [{} - {}]", a, b);
        if iterations == iteration_count {
            break;
        }
        if dsu.find(a) != dsu.find(b) {
            dsu.union(a, b);
            chosen_edges.push((a, b, dist));
        }
    }

    // Count circuit sizes
    let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();
    for i in 0..boxes.len() {
        let r = dsu.find(i);
        *circuit_sizes.entry(r).or_insert(0) += 1;
    }

    //dbg!(&chosen_edges);
    //dbg!(&circuit_sizes);

    let result = three_largest_values(&circuit_sizes).unwrap();

    Some((result[0] * result[1] * result[2]) as u64)
}

fn part_two(input: &str) -> Option<u64> {
    // same as part one, but waiting until elements-1 merges in Dsu

    let lines: Vec<&str> = input.trim_end().lines().collect();
    let boxes: Vec<JunctionBox> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| JunctionBox::from(line, i))
        .collect();

    // Number of merges = number of elements - 1
    let expected_merges_count = lines.len() - 1;

    let mut edges: Vec<(usize, usize, f32)> = vec![];
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            edges.push((i, j, boxes[i].distance_to(boxes[j])));
        }
    }

    // Sort by distance ascending
    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    // for e in &edges {
    //     println!("[{}:{}] - [{}:{}]", e.0, boxes[e.0], e.1, boxes[e.1]);
    // }

    let mut dsu = UnionFind::new(boxes.len());

    let mut iterations = 0;
    let mut chosen_edges = Vec::new();

    for &(a, b, dist) in &edges {
        //println!("===> [{} - {}]", a, b);
        if dsu.find(a) != dsu.find(b) {
            dsu.union(a, b);
            chosen_edges.push((a, b, dist));
            iterations += 1;
            if iterations == expected_merges_count {
                return Some(boxes[a].x as u64 * boxes[b].x as u64);
            }
        }
    }

    // Count circuit sizes - just for debug this time
    let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();
    for i in 0..boxes.len() {
        let r = dsu.find(i);
        *circuit_sizes.entry(r).or_insert(0) += 1;
    }

    dbg!(iterations);
    dbg!(circuit_sizes);
    panic!("Cannot merge all boxes into circuit!")
}

fn three_largest_values(map: &HashMap<usize, usize>) -> Option<[usize; 3]> {
    if map.len() < 3 {
        return None;
    }

    let (first, second, third) = map.values().fold((0, 0, 0), |(f, s, t), &v| {
        if v > f {
            (v, f, s)
        } else if v > s {
            (f, v, s)
        } else if v > t {
            (f, s, v)
        } else {
            (f, s, t)
        }
    });

    Some([first, second, third])
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return false;
        }
        if self.rank[x] < self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent[y] = x;
        if self.rank[x] == self.rank[y] {
            self.rank[x] += 1;
        }
        true
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Hash, Eq, PartialOrd)]
struct JunctionBox {
    x: u32,
    y: u32,
    z: u32,
}

impl JunctionBox {
    fn distance_to(&self, other: JunctionBox) -> f32 {
        ((self.z as i64 - other.z as i64).pow(2)
            + (self.y as i64 - other.y as i64).pow(2)
            + (self.x as i64 - other.x as i64).pow(2)) as f32
    }
}

impl JunctionBox {
    fn from(value: &str, _: usize) -> Self {
        let mut parsed = value.split(',');
        JunctionBox {
            x: parsed.next().unwrap().parse().unwrap(),
            y: parsed.next().unwrap().parse().unwrap(),
            z: parsed.next().unwrap().parse().unwrap(),
        }
    }
}

impl Display for JunctionBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_for_iteration(
            &advent_of_code::template::read_file_input("examples", DAY, "example_1.txt"),
            10,
        );
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1.txt",
        ));
        assert_eq!(result, Some(25272));
    }
}

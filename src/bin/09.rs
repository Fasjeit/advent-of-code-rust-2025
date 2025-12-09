advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.trim_end().lines();
    let points: Vec<(u64, u64)> = lines
        .map(|l| {
            let mut splitted = l.split(',');
            (
                splitted.next().unwrap().parse().unwrap(),
                splitted.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    //dbg!(points.len());

    let mut max_value = 0;
    for i in 0..points.len() {
        for j in 0..points.len() {
            if i == j {
                continue;
            }
            //dbg!(points[i]);
            //dbg!(points[j]);
            let size = ((points[i].0 as i64 - points[j].0 as i64 + 1)
                * (points[i].1 as i64 - points[j].1 as i64 + 1))
                .unsigned_abs();
            //dbg!(size);
            if size > max_value {
                max_value = size
            }
        }
    }

    Some(max_value)
}

pub fn part_two(input: &str) -> Option<u64> {
    // thanks to
    // https://www.reddit.com/r/adventofcode/comments/1phywvn/comment/nt2g1g7/
    // and
    // (not working, but still stole some ideas) https://www.reddit.com/r/adventofcode/comments/1phywvn/comment/nt2kaiu/

    let lines = input.trim_end().lines();
    let points: Vec<(u64, u64)> = lines
        .map(|l| {
            let mut splitted = l.split(',');
            (
                splitted.next().unwrap().parse().unwrap(),
                splitted.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    Some(find_largest_valid_rectangle(&points))
}

fn find_largest_valid_rectangle(points: &[(u64, u64)]) -> u64 {
    let mut max_area = 0;

    for i in 0..points.len() {
        let (x1, y1) = points[i];
        for j in (i + 1)..points.len() {
            let (x2, y2) = points[j];

            if x1 != x2 && y1 != y2 {
                let width = (x2 as i64 - x1 as i64).unsigned_abs() + 1;
                let height = (y2 as i64 - y1 as i64).unsigned_abs() + 1;
                let area = width * height;

                if area > max_area && is_rectangle_inside_polygon(x1, y1, x2, y2, points) {
                    max_area = area;
                }
            }
        }
    }

    max_area
}

fn is_rectangle_inside_polygon(
    x1: u64,
    y1: u64,
    x2: u64,
    y2: u64,
    polygon_points: &[(u64, u64)],
) -> bool {
    let (min_x, max_x) = (x1.min(x2), x1.max(x2));
    let (min_y, max_y) = (y1.min(y2), y1.max(y2));

    // Check if any polygon edge cuts through the rectangle
    for i in 0..polygon_points.len() {
        let (sx1, sy1) = polygon_points[i];
        let (sx2, sy2) = polygon_points[(i + 1) % polygon_points.len()];
        if sx1 == sx2 {
            // Vertical edge
            if sx1 > min_x && sx1 < max_x {
                let seg_y_min = sy1.min(sy2);
                let seg_y_max = sy1.max(sy2);
                if min_y.max(seg_y_min) < max_y.min(seg_y_max) {
                    return false;
                }
            }
        } else {
            // Horizontal edge
            if sy1 > min_y && sy1 < max_y {
                let seg_x_min = sx1.min(sx2);
                let seg_x_max = sx1.max(sx2);
                if min_x.max(seg_x_min) < max_x.min(seg_x_max) {
                    return false;
                }
            }
        }
    }

    // Use ray casting to check if rectangle center is inside polygon
    let center_x = (min_x + max_x) as f64 / 2.0;
    let center_y = (min_y + max_y) as f64 / 2.0;

    let mut intersections = 0;
    for i in 0..polygon_points.len() {
        let (sx1, sy1) = polygon_points[i];
        let (sx2, sy2) = polygon_points[(i + 1) % polygon_points.len()];
        if sx1 == sx2 {
            let edge_x = sx1 as f64;
            if edge_x > center_x {
                let edge_y_min = sy1.min(sy2) as f64;
                let edge_y_max = sy1.max(sy2) as f64;
                if center_y > edge_y_min && center_y < edge_y_max {
                    intersections += 1;
                }
            }
        }
    }

    intersections % 2 == 1
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_input(
            "examples",
            DAY,
            "example_1.txt",
        ));
        assert_eq!(result, Some(24));
    }
}

use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::*;
use itertools::Itertools;


fn part1(contents: String) -> i64 {
    let corners: Vec<(i64, i64)> = Vec::from_iter(
        contents
            .lines()
            .map(|line| {
                let mut parts = line.split(",");
                let x = parts.next().unwrap().parse::<i64>().unwrap();
                let y = parts.next().unwrap().parse::<i64>().unwrap();
                (x, y)
            })
    );

    return corners.iter().cartesian_product(corners.iter())
        .map(|((x1, y1), (x2, y2))| ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1))
        .max()
        .unwrap();
}

fn part2(contents: String) -> i64 {
    let mut x_heap: BTreeSet<i64> = BTreeSet::new();
    let mut y_heap: BTreeSet<i64> = BTreeSet::new();

    let corners: Vec<(i64, i64)> = Vec::from_iter(
        contents
            .lines()
            .map(|line| {
                let mut parts = line.split(",");
                let x = parts.next().unwrap().parse::<i64>().unwrap();
                let y = parts.next().unwrap().parse::<i64>().unwrap();
                x_heap.insert(x);
                y_heap.insert(y);
                (x, y)
            })
    );

    let x_sorted: Vec<i64> = x_heap.into_iter().collect();
    let y_sorted: Vec<i64> = y_heap.into_iter().collect();

    let reduced_corners: Vec<(i64, i64)> = corners.iter()
        .map(|(x, y)| {
            let x_index = x_sorted.iter().position(|v| v == x).unwrap();
            let y_index = y_sorted.iter().position(|v| v == y).unwrap();

            (x_index as i64, y_index as i64)
        }).collect();

    let mut shape: HashSet<(i64, i64)> = HashSet::from_iter(
        Itertools::zip_eq(reduced_corners.iter(), reduced_corners.iter().skip(1).chain(std::iter::once(&reduced_corners[0])))
            .flat_map(|((x1, y1), (x2, y2))| {
                let mut points: Vec<(i64, i64)> = Vec::new();

                if x1 == x2 {
                    let (start, end) = if y1 < y2 { (*y1, *y2) } else { (*y2, *y1) };
                    for y in start..=end {
                        points.push((*x1, y));
                    }
                } else if y1 == y2 {
                    let (start, end) = if x1 < x2 { (*x1, *x2) } else { (*x2, *x1) };
                    for x in start..=end {
                        points.push((x, *y1));
                    }
                }

                points.into_iter()
            })
    );

    // Find the top-most edge, run flood-fill from the first inside point found
    let mut to_visit: VecDeque<(i64, i64)> = VecDeque::from({
        let mut top_left_vec = reduced_corners.clone();
        top_left_vec.sort_by_key(|(x, y)| (*y, *x));
        top_left_vec.iter()
            .map(|(x, y)| (*x + 1, *y + 1))
            .filter(|(x, y)| !shape.contains(&(*x, *y)))
            .take(1)
            .collect::<Vec<(i64, i64)>>()
    });

    while let Some((x, y)) = to_visit.pop_front() {
        if shape.contains(&(x, y)) {
            continue;
        }

        shape.insert((x, y));

        let neighbors = vec![
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
        ];

        for neighbor in neighbors {
            if !shape.contains(&neighbor) {
                to_visit.push_back(neighbor);
            }
        }
    }

    return corners.iter().cartesian_product(corners.iter())
        .filter(|((x1, y1), (x2, y2))| {
            if (x1, y1) == (x2, y2) {
                return false;
            }
            let rx1 = x_sorted.iter().position(|v| v == x1).unwrap() as i64;
            let ry1 = y_sorted.iter().position(|v| v == y1).unwrap() as i64;
            let rx2 = x_sorted.iter().position(|v| v == x2).unwrap() as i64;
            let ry2 = y_sorted.iter().position(|v| v == y2).unwrap() as i64;
            let corners = vec![(rx1, ry1), (rx1, ry2), (rx2, ry2), (rx2, ry1)];
            // Check if all boundary points are in the shape
            let mut all_in_shape = true;
            for (c1, c2) in Itertools::zip_eq(corners.iter(), corners.iter().skip(1).chain(std::iter::once(&corners[0]))) {
                if c1.0 == c2.0 {
                    let (start, end) = if c1.1 < c2.1 { (c1.1, c2.1) } else { (c2.1, c1.1) };
                    for y in start..=end {
                        if !shape.contains(&(c1.0, y)) {
                            all_in_shape = false;
                            break;
                        }
                    }
                } else if c1.1 == c2.1 {
                    let (start, end) = if c1.0 < c2.0 { (c1.0, c2.0) } else { (c2.0, c1.0) };
                    for x in start..=end {
                        if !shape.contains(&(x, c1.1)) {
                            all_in_shape = false;
                            break;
                        }
                    }
                }
                if !all_in_shape {
                    break;
                }
            }
            all_in_shape
        })
        .map(|((x1, y1), (x2, y2))| ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1))
        .max()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 50);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 24);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2025".to_string();
    let day = "9".to_string();

    let root = env::current_dir().unwrap();
    let path_str = if args.len() > 1 {
        args[1].clone()
    } else if root.ends_with(format!("{}", day)) {
        format!("../../../Inputs/{}_{}.txt", year, day)
    } else {
        format!("/Inputs/{}_{}.txt", year, day)
    };

    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nLargest rectangle: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLargest fully contained rectangle: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}
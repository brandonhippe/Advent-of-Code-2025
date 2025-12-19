use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::*;

fn part1(contents: String) -> i64 {
    let mut splitters: HashSet<(usize, usize)> = HashSet::new();
    let mut max_y = 0;
    let mut start: Option<(usize, usize)> = None;
    for (y, line) in contents.lines().step_by(2).enumerate() {
        max_y = y;
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = Some((x, y));
                },
                '^' => {
                    splitters.insert((x, y));
                },
                '.' => {},
                _ => panic!("Unexpected character: {}", c),
            }
        }
    }

    let start = start.expect("No start position found");
    let mut open_list: VecDeque<(usize, usize)> = VecDeque::from([start]);
    let mut closed_list: HashSet<(usize, usize)> = HashSet::new();
    
    let mut split_count = 0;
    while let Some(current) = open_list.pop_front() {
        if closed_list.contains(&current) || current.1 > max_y {
            continue;
        }
        closed_list.insert(current);

        let new_pos = (current.0, current.1 + 1);
        if splitters.contains(&new_pos) {
            split_count += (!closed_list.contains(&new_pos)) as i64;
            open_list.push_back((new_pos.0 - 1, new_pos.1));
            open_list.push_back((new_pos.0 + 1, new_pos.1));
        } else {
            open_list.push_back(new_pos);
        }
    }

    return split_count;
}

fn part2(contents: String) -> i64 {
    let mut splitters: HashSet<(usize, usize)> = HashSet::new();
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut start: Option<(usize, usize)> = None;
    for (y, line) in contents.lines().step_by(2).enumerate() {
        max_y = y;
        for (x, c) in line.chars().enumerate() {
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }

            match c {
                'S' => {
                    start = Some((x, y));
                },
                '^' => {
                    splitters.insert((x, y));
                },
                '.' => {},
                _ => panic!("Unexpected character: {}", c),
            }
        }
    }

    let start = start.expect("No start position found");

    let mut path_counts: Vec<i64> = vec![1; max_x - min_x + 1];
    
    for y in (0..=max_y).rev() {
        let p_path_counts = path_counts.clone();
        for splitter in splitters.iter().filter(|(_, sy)| *sy == y) {
            let (sx, _) = *splitter;
            let lix = sx - min_x - 1;
            let rix = sx - min_x + 1;

            path_counts[sx - min_x] = p_path_counts[lix] + p_path_counts[rix];
        }
    }

    return path_counts[start.0 - min_x];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 21);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 40);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2025".to_string();
    let day = "7".to_string();

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
        "\nPart 1:\nNumber of times beam is split: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nNumber of timelines: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}
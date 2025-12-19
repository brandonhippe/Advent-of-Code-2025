use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let (ranges_str, ids_str) = contents.split_once("\n\n").unwrap();
    
    let mut ranges: Vec<(i64, i64)> = ranges_str
        .lines()
        .map(|line| {
            let (start_str, end_str) = line.split_once('-').unwrap();
            (
                start_str.parse::<i64>().unwrap(),
                end_str.parse::<i64>().unwrap(),
            )
        })
        .collect();

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged_ranges: Vec<(i64, i64)> = Vec::new();
    for range in ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if range.0 <= last.1 + 1 {
                last.1 = last.1.max(range.1);
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }

    return ids_str.lines()
        .map(|id_str| id_str.parse::<i64>().unwrap())
        .filter(|id| {
            for range in &merged_ranges {
                if *id >= range.0 && *id <= range.1 {
                    return true;
                }
            }
            return false;
        })
        .count() as i64;
}

fn part2(contents: String) -> i64 {
    let (ranges_str, _) = contents.split_once("\n\n").unwrap();
    
    let mut ranges: Vec<(i64, i64)> = ranges_str
        .lines()
        .map(|line| {
            let (start_str, end_str) = line.split_once('-').unwrap();
            (
                start_str.parse::<i64>().unwrap(),
                end_str.parse::<i64>().unwrap(),
            )
        })
        .collect();

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged_ranges: Vec<(i64, i64)> = Vec::new();
    for range in ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if range.0 <= last.1 + 1 {
                last.1 = last.1.max(range.1);
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }

    return merged_ranges.iter()
        .map(|(start, end)| end - start + 1)
        .sum::<i64>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 3);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 14);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2025".to_string();
    let day = "5".to_string();

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
        "\nPart 1:\nNumber of available fresh ingredients: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nNumber of possible fresh ingredients: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}
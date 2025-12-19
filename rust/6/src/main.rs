use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashSet;

fn part1(contents: String) -> i64 {
    let mut columns: HashSet<usize> = HashSet::from_iter(0..=contents.lines().next().unwrap().len());
    for line in contents.lines() {
        for (i, c) in line.chars().enumerate() {
            if c != ' ' {
                columns.remove(&i);
            }
        }
    }
    let mut sorted_columns: Vec<&usize> = Vec::from_iter(columns.iter());
    sorted_columns.sort();

    let mut result_total: i64 = 0;
    let mut start_col: Option<usize> = None;
    for end_col in sorted_columns {
        let nums: Vec<i64> = contents
            .lines()
            .rev()
            .skip(1)
            .map(|line| line[start_col.unwrap_or(0)..*end_col].trim().parse::<i64>().unwrap())
            .collect();

        match contents.lines().last().unwrap()[start_col.unwrap_or(0)..*end_col].trim() {
            "+" => {
                result_total += nums.iter().sum::<i64>();
            }
            "*" => {
                result_total += nums.iter().product::<i64>();
            }
            _ => panic!("Unknown operation"),
        }
        start_col = Some(end_col + 1);
    }

    return result_total;
}

fn part2(contents: String) -> i64 {
    let mut columns: HashSet<usize> = HashSet::from_iter(0..=contents.lines().next().unwrap().len());
    for line in contents.lines() {
        for (i, c) in line.chars().enumerate() {
            if c != ' ' {
                columns.remove(&i);
            }
        }
    }
    let mut sorted_columns: Vec<&usize> = Vec::from_iter(columns.iter());
    sorted_columns.sort();

    let mut result_total: i64 = 0;
    let mut start_col: Option<usize> = None;
    for end_col in sorted_columns {
        let nums: Vec<i64> = (start_col.unwrap_or(0)..*end_col)
            .map(|i| 
                contents
                    .lines()
                    .rev()
                    .skip(1)
                    .map(|line| line.chars().nth(i).unwrap())
                    .filter(|&c| c != ' ')
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap()
            )
            .collect();

        match contents.lines().last().unwrap()[start_col.unwrap_or(0)..*end_col].trim() {
            "+" => {
                result_total += nums.iter().sum::<i64>();
            }
            "*" => {
                result_total += nums.iter().product::<i64>();
            }
            _ => panic!("Unknown operation"),
        }
        start_col = Some(end_col + 1);
    }

    return result_total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 4277556);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 3263827);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2025".to_string();
    let day = "6".to_string();

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
        "\nPart 1:\nSum of problem answers: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSum of problem answers: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}
use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

use std::collections::HashSet;


fn invalid_ids(min_id: i64, max_id: i64, rep_count: Option<u32>) -> HashSet<i64> {
    let min_digs = min_id.ilog10() + 1;
    let max_digs = max_id.ilog10() + 1;

    if min_digs != max_digs {
        return invalid_ids(min_id, 10i64.pow(min_digs as u32) - 1, rep_count).union(&invalid_ids(10i64.pow(min_digs as u32), max_id, rep_count)).cloned().collect();
    }

    if min_digs % rep_count.unwrap_or(2) != 0 {
        return HashSet::new();
    }

    let rep_digs = min_digs / rep_count.unwrap_or(2);
    let divisor = (0..min_digs).step_by(rep_digs as usize).fold(0, |acc, exp| acc + 10i64.pow(exp as u32));

    return HashSet::from_iter((((min_id - 1) / divisor + 1)..=(max_id / divisor))
        .map(|quot| quot * divisor));
}

fn part1(contents: String) -> i64 {
    return contents.lines().next().unwrap().split(',')
        .map(|range| {
            let mut bounds = range.split('-').map(|x| x.parse::<i64>().unwrap());
            let min_id = bounds.next().unwrap();
            let max_id = bounds.next().unwrap();

            invalid_ids(min_id, max_id, None).into_iter().sum::<i64>()
        })
        .sum();
}

fn part2(contents: String) -> i64 {
    return contents.lines().next().unwrap().split(',')
        .map(|range| {
            let mut bounds = range.split('-').map(|x| x.parse::<i64>().unwrap());
            let min_id = bounds.next().unwrap();
            let max_id = bounds.next().unwrap();

            (2..=(max_id.ilog10() + 1) as u32)
                .map(|rep_count| invalid_ids(min_id, max_id, Some(rep_count)))
                .fold(HashSet::new(), |acc, set| acc.union(&set).cloned().collect())
                .into_iter().sum::<i64>()
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 1227775554);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 4174379265);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2025".to_string();
    let day = "2".to_string();

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
        "\nPart 1:\nSum of invalid IDs: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSum of invalid IDs: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}
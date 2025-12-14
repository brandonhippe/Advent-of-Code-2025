use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut dial = 50;
    let mut count = 0;
    for line in contents.lines() {
        let d = match line.chars().next().unwrap() {
            'R' => 1,
            'L' => -1,
            _ => 0,
        };
        let val = line[1..].parse::<i64>().unwrap();
        dial += d * val;
        if dial >= 100 || dial <= 0 {
            dial = ((dial % 100) + 100) % 100;
            count += (dial == 0) as i64;
        }
    }
    return count;
}

fn part2(contents: String) -> i64 {
    let mut dial = 50;
    let mut count = 0;
    for line in contents.lines() {
        let d = match line.chars().next().unwrap() {
            'R' => 1,
            'L' => -1,
            _ => 0,
        };
        let mut val = line[1..].parse::<i64>().unwrap();
        for _ in 0..val {
            dial += d;
            if dial >= 100 {
                dial = 0;
            } else if dial < 0 {
                dial = 99;
            }
            if dial == 0 {
                count += 1;
            }
        }
    }
    return count;
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

        assert_eq!(part2(contents), 6);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2025".to_string();
    let day = "1".to_string();

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
        "\nPart 1:\nPassword: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nPassword: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}
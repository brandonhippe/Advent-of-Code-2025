use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;


fn max_val(line: &str, digs: usize) -> i64 {
    let mut max_val = 0;
    let mut ix = 0;
    for dig in 1..=digs {
        let (new_ix, d) = line[ix..(line.len() - (digs - dig))]
            .chars()
            .enumerate()
            .max_by_key(|&(i, c)| (c, -(i as isize)))
            .map(|(i, c)| (i + ix, c.to_digit(10).unwrap() as i64))
            .unwrap();
        max_val = max_val * 10 + d;
        ix = new_ix + 1;
    }
    max_val
}

fn part1(contents: String) -> i64 {
    return contents.lines().map(|line| max_val(line, 2)).sum();
}

fn part2(contents: String) -> i64 {
    return contents.lines().map(|line| max_val(line, 12)).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 357);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 3121910778619);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2025".to_string();
    let day = "3".to_string();

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
        "\nPart 1:\nTotal Joltage: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nTotal Joltage: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}
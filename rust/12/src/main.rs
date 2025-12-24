use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::*;

fn part1(contents: String, test: bool) -> i64 {
    let shapes: Vec<Vec<Vec<bool>>> = Vec::from_iter(contents.split("\n\n")
        .filter_map(|shape_str| {
            let mut shape_lines = shape_str.lines();
            if !shape_lines.next().unwrap().ends_with(':') {
                return None;
            }
            Some(shape_lines.map(|line| {
                line.chars().map(|c| c == '#').collect::<Vec<bool>>()
            }).collect::<Vec<Vec<bool>>>())
        })
    );

    return contents.split("\n\n").last().unwrap().lines()
        .filter(|line| {
            let shape: Vec<i64> = Vec::from_iter(line.split(':').next().unwrap().split('x').map(|n| n.parse::<i64>().unwrap()));
            let counts: Vec<i64> = Vec::from_iter(line.split(':').last().unwrap().trim().split(' ').map(|n| n.parse::<i64>().unwrap()));

            if counts.iter().zip(shapes.iter()).map(|(count, shape)| {
                count * shape.iter().map(|row| row.iter().filter(|&&c| c).count() as i64).sum::<i64>()
            }).sum::<i64>() > shape.iter().product::<i64>() {
                return false;
            }

            let shape_dims: HashSet<(i64, i64)> = HashSet::from_iter(shapes.iter().map(|shape| {
                (shape.len() as i64, shape[0].len() as i64)
            }));

            assert_eq!(shape_dims.len(), 1, "Multiple shape dimensions found!");
            let (shape_h, shape_w) = *shape_dims.iter().next().unwrap();

            if counts.into_iter().sum::<i64>() <= (shape[0] / shape_h) * (shape[1] / shape_w) {
                return true;
            }

            assert_eq!(test, true, "Too complex to determine fit!");
            true
        }).count() as i64;
}

fn part2(contents: String) -> String {
    return "Christmas has been saved!".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, true), 2);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2025".to_string();
    let day = "12".to_string();

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
        "\nPart 1:\nRegions that can fit the listed presents: {}\nRan in {:.5?}",
        part1(contents.clone(), false),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\n {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}
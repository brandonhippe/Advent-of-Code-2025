use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use cached::proc_macro::cached;


#[cached]
fn min_presses(buttons: Vec<i64>, goal: i64, remaining_counts: Option<Vec<i64>>) -> i64 {
    if goal == 0 && remaining_counts.as_ref().unwrap().iter().all(|&count| count == 0) {
        return 0;
    }
    
    let mut minimum = i64::MAX;
    for choices in (0..2i64.pow(buttons.len() as u32)).map(|n| {
        (0..buttons.len()).filter(|&i| (n & (1 << i)) != 0).collect::<Vec<usize>>()
    }) {
        let state = choices.iter().fold(0, |acc, &i| acc ^ buttons[i]);
        if state != goal {
            continue;
        }
        
        let mut new_remaining: Vec<i64>;
        if let Some(ref counts) = remaining_counts {
            new_remaining = counts.to_vec();
        } else {
            minimum =  minimum.min(choices.len() as i64);
            continue;
        }

        for b in choices.iter() {
            let mut v = buttons[*b];
            let mut ix = 0;
            while v != 0 {
                new_remaining[ix] -= v & 1;
                v >>= 1;
                ix += 1;
            }
        }

        if new_remaining.iter().any(|&count| count < 0) {
            continue;
        }

        let mut new_goal = 0;
        for ix in (0..new_remaining.len()).rev() {
            new_remaining[ix] >>= 1;
            new_goal = (new_goal << 1) | (new_remaining[ix] % 2);
        }

        let reduced_min = min_presses(buttons.clone(), new_goal, Some(new_remaining));
        if reduced_min != i64::MAX {
            minimum = minimum.min(2 * reduced_min + choices.len() as i64);
        }
    }

    return minimum;
}


fn part1(contents: String) -> i64 {
    return contents.lines()
        .map(|line| {
            let mut space_iter = line.split(' ');
            let goal_str = space_iter.next().unwrap().chars().rev().collect::<String>();
            let goal = goal_str.chars().skip(1).take(goal_str.len() - 2).fold(0, |acc, c| acc << 1 | (c == '#') as i64);
            let buttons: Vec<i64> = Vec::from_iter(
                space_iter.filter(|s| s.chars().next().unwrap_or(' ') == '(')
                    .map(|s| s.chars().skip(1).take(s.len() - 2).collect::<String>().split(',')
                        .map(|num_str| num_str.parse::<i64>().unwrap())
                        .fold(0, |acc, n| acc | (1 << n))
            ));
            min_presses(buttons.clone(), goal, None)
        })
        .sum();
}

fn part2(contents: String) -> i64 {
    return contents.lines()
        .map(|line| {
            let space_iter = line.split(' ');
            let activate_counts: Vec<i64> = Vec::from_iter(space_iter.clone().last().unwrap().chars().skip(1).take(space_iter.clone().last().unwrap().len() - 2).collect::<String>().split(',')
                .map(|num_str| num_str.parse::<i64>().unwrap())
            );

            let buttons: Vec<i64> = Vec::from_iter(
                space_iter.filter(|s| s.chars().next().unwrap_or(' ') == '(')
                    .map(|s| s.chars().skip(1).take(s.len() - 2).collect::<String>().split(',')
                        .map(|num_str| num_str.parse::<i64>().unwrap())
                        .fold(0, |acc, n| acc | (1 << n))
            ));

            let p = min_presses(buttons, activate_counts.clone().into_iter().rev().fold(0, |acc, x| acc << 1 | (x % 2)), Some(activate_counts));
            if p == i64::MAX {
                panic!("No solution found");
            }
            p
        }).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 7);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 33);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2025".to_string();
    let day = "10".to_string();

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
        "\nPart 1:\nFewest # of presses: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nFewest # of presses: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}
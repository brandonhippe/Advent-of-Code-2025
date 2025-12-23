use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::*;
use cached::proc_macro::cached;


#[cached]
fn num_paths(keys: Vec<String>, connections: Vec<Vec<String>>, start: String, end: String, visited: Vec<bool>) -> i64 {
    if start == end {
        return 1;
    }

    let mut total_paths = 0;
    let start_index = keys.iter().position(|k| *k == start).unwrap();
    for neighbor in &connections[start_index] {
        let neighbor_index = keys.iter().position(|k| *k == *neighbor).unwrap();
        if !visited[neighbor_index] {
            total_paths += num_paths(keys.clone(), connections.clone(), neighbor.to_string(), end.clone(), (0..visited.len()).map(|ix| if ix == neighbor_index { true } else { visited[ix] }).collect::<Vec<bool>>());
        }
    }

    return total_paths;
}


fn part1(contents: String) -> i64 {
    let connections: HashMap<String, Vec<String>> = HashMap::from_iter(
        contents.lines().map(|line| {
            let mut space_iter = line.split(" ");
            let mut from_key: String = space_iter.next().unwrap().to_string();
            from_key = from_key.trim_end_matches(':').to_string();

            (from_key.to_string(), space_iter.map(|s| s.to_string()).collect::<Vec<String>>())
        })
    );

    let mut sorted_keys: Vec<String> = HashSet::<String>::from_iter(connections.keys()
        .flat_map(|k| {
            vec![k.to_string()].into_iter().chain(connections.get(k).unwrap().to_vec().into_iter())
        })).into_iter().collect();

    sorted_keys.sort();

    return num_paths(
        sorted_keys.clone(),
        sorted_keys.iter().map(|k| connections.get(k).unwrap_or(&vec![]).to_vec()).collect(),
        "you".to_string(),
        "out".to_string(),
        vec![false; sorted_keys.len()]
    );
}

fn part2(contents: String) -> i64 {
    let mut in_connections: HashMap<String, i64> = HashMap::new();
    let connections: HashMap<String, Vec<String>> = HashMap::from_iter(
        contents.lines().map(|line| {
            let mut space_iter = line.split(" ");
            let mut from_key: String = space_iter.next().unwrap().to_string();
            from_key = from_key.trim_end_matches(':').to_string();
            let to_keys: Vec<String> = space_iter.map(|s| s.to_string()).collect::<Vec<String>>();
            for to_key in &to_keys {
                *in_connections.entry(to_key.to_string()).or_insert(0) += 1;
            }

            (from_key.to_string(), to_keys)
        })
    );

    let mut sorted_keys: Vec<String> = HashSet::<String>::from_iter(connections.keys()
    .flat_map(|k| {
        vec![k.to_string()].into_iter().chain(connections.get(k).unwrap().to_vec().into_iter())
    })).into_iter().collect();
    
    sorted_keys.sort();
    let connection_vecs: Vec<Vec<String>> = sorted_keys.iter().map(|k| connections.get(k).unwrap_or(&vec![]).to_vec()).collect();

    let mut most_connections: Vec<String> = sorted_keys.clone();
    most_connections.sort_by_key(|k| -in_connections.get(k).unwrap_or(&0));
    most_connections = most_connections.clone().into_iter().filter(|k| *in_connections.get(k).unwrap_or(&0) >= (in_connections.get(most_connections.clone().iter().next().unwrap()).unwrap_or(&0)) / 2).collect::<Vec<String>>();
    for k in ["dac", "fft", "out"] {
        if !most_connections.contains(&k.to_string()) {
            most_connections.push(k.to_string());
        }
    }
    
    let mut poi_paths: HashMap<String, HashSet<String>> = HashMap::new();
    let mut positions: VecDeque<(String, String)> = VecDeque::from(connections.get("svr").unwrap_or(&vec![]).iter().map(|k| (k.to_string(), "svr".to_string())).collect::<Vec<(String, String)>>());
    let mut visited: HashSet<(String, String)> = HashSet::new();

    while let Some((current, from)) = positions.pop_front() {
        let new_from =if most_connections.contains(&current) {
            poi_paths.entry(from.clone()).or_insert(HashSet::new()).insert(current.clone());
            current.clone()
        } else {
            from.clone()
        };

        for neighbor in connections.get(&current).unwrap_or(&vec![]) {
            if visited.contains(&(neighbor.to_string(), new_from.clone())) {
                continue;
            }
            visited.insert((neighbor.to_string(), new_from.clone()));
            positions.push_back((neighbor.to_string(), new_from.clone()));
        }
    }

    for k in most_connections.clone() {
        let required_intersection: HashSet<String> = poi_paths.get(&k).unwrap_or(&HashSet::new()).intersection(&HashSet::from(["fft".to_string(), "dac".to_string()])).cloned().collect();
        if required_intersection.len() > 0 {
            poi_paths.insert(k.clone(), required_intersection);
        }
    }

    let mut main_paths: HashSet<Vec<String>> = HashSet::new();
    let mut path_positions: VecDeque<Vec<String>> = VecDeque::from([vec!["svr".to_string()]]);
    
    while let Some(current_path) = path_positions.pop_front() {
        let last_node = current_path.last().unwrap().to_string();
        if last_node == "out" {
            if vec!["fft", "dac"].iter().all(|k| current_path.contains(&k.to_string())) {
                main_paths.insert(current_path.clone());
            }
            continue;
        }

        for neighbor in poi_paths.get(&last_node).unwrap_or(&HashSet::new()) {
            if current_path.contains(neighbor) {
                continue;
            }
            let mut new_path = current_path.clone();
            new_path.push(neighbor.to_string());
            path_positions.push_back(new_path);
        }
    }

    return main_paths.iter().map(|p| {
        p.windows(2).map(|w| {
            let start = w[0].to_string();
            let end = w[1].to_string();
            num_paths(
                sorted_keys.clone(),
                connection_vecs.clone(),
                start,
                end.clone(),
                sorted_keys.iter().map(|sk| *sk != end && most_connections.contains(sk)).collect::<Vec<bool>>()
            )
        }).product::<i64>()
    }).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example1.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 5);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 2);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2025".to_string();
    let day = "11".to_string();

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
        "\nPart 1:\n# of paths from 'you' to 'out': {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\n# of paths from 'svr' to 'out' through 'fft' and 'dac': {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}
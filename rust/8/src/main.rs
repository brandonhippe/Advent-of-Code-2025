use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::*;
use std::cmp::{Ordering, Reverse};
use std::hash::{Hash, Hasher};


#[derive(Debug, Clone)]
struct Pair {
    a: (i64, i64, i64),
    b: (i64, i64, i64),
    dist: f64,
}

impl Pair {
    fn new(a: (i64, i64, i64), b: (i64, i64, i64)) -> Self {
        Pair { 
            a, 
            b, 
            dist: f64::sqrt(((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)) as f64)
        }
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.partial_cmp(&other.dist).unwrap()
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}

impl Eq for Pair {}

impl Hash for Pair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.b.hash(state);
    }
}


#[derive(Debug, Clone)]
struct Circuit {
    points: HashSet<(i64, i64, i64)>,
}

impl Circuit {
    fn new() -> Self {
        Circuit {
            points: HashSet::new(),
        }
    }

    fn from(point: (i64, i64, i64)) -> Self {
        let mut circuit = Circuit::new();
        circuit.add_point(point);
        circuit
    }

    fn add_point(&mut self, point: (i64, i64, i64)) {
        self.points.insert(point);
    }

    fn union(&self, other: &Circuit) -> Circuit {
        let mut new_circuit = self.clone();
        new_circuit.points = new_circuit.points.union(&other.points).cloned().collect();
        new_circuit
    }

    fn contains(&self, point: &(i64, i64, i64)) -> bool {
        self.points.contains(point)
    }
}

impl Ord for Circuit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.points.len().cmp(&other.points.len())
    }
}

impl PartialOrd for Circuit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Circuit {
    fn eq(&self, other: &Self) -> bool {
        self.points.clone().symmetric_difference(&other.points.clone()).count() == 0
    }
}

impl Eq for Circuit {}

impl Hash for Circuit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let cloned_points: HashSet<(i64, i64, i64)> = self.points.clone();
        let mut points_vec: Vec<(i64, i64, i64)> = cloned_points.into_iter().collect();
        points_vec.sort();
        for point in points_vec {
            point.hash(state);
        }
    }
}


fn part1(contents: String) -> i64 {
    let num_pairs = if contents.lines().count() == 20 { 10 } else { 1000 };
    let lights: HashSet<(i64, i64, i64)> = HashSet::from_iter(
        contents
            .lines()
            .map(|line| {
                let coords: Vec<i64> = line
                    .split(',')
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect();
                (coords[0], coords[1], coords[2])
            })
    );

    let mut sorted_pairs: BinaryHeap<Reverse<Pair>> = BinaryHeap::from_iter(
        HashSet::from(
            lights
                .iter()
                .enumerate()
                .flat_map(|(i, a)| lights.iter().skip(i + 1).map(move |b| (a, b)))
                .map(|(a, b)| Pair::new(*a, *b))
                .collect::<HashSet<Pair>>()
        ).into_iter().map(|pair| Reverse(pair))
    );

    let mut point_ixs: HashMap<(i64, i64, i64), usize> = HashMap::new();
    let mut circuits: Vec<Circuit> = Vec::from_iter(
        lights.iter().enumerate().map(|(ix, point)| {
            point_ixs.insert(*point, ix);
            Circuit::from(*point)
        })
    );
    
    for it in 1..=num_pairs {
        let Reverse(pair) = sorted_pairs.pop().unwrap();
        if point_ixs[&pair.a] == point_ixs[&pair.b] {
            continue;
        }

        let new_circuit: Circuit = {
            let mut points = vec![pair.a, pair.b];
            points.sort_by(|a, b| point_ixs[b].cmp(&point_ixs[a]));
            points.iter().fold(Circuit::new(), |acc, point| acc.union(&circuits.remove(point_ixs[point])))
        };

        point_ixs = HashMap::from_iter(
            point_ixs.iter().map(|(point, ix)| {
                (*point,
                    if new_circuit.contains(point) {
                        circuits.len()
                    } else {
                        *ix - (*ix > point_ixs[&pair.a]) as usize - (*ix > point_ixs[&pair.b]) as usize
                    }
                )
            })
        );

        circuits.push(new_circuit);
    }

    circuits.sort_by(|a, b| b.cmp(a));
    return circuits.into_iter().take(3).fold(1, |acc, c| acc * (c.points.len() as i64));
}

fn part2(contents: String) -> i64 {
    let lights: HashSet<(i64, i64, i64)> = HashSet::from_iter(
        contents
            .lines()
            .map(|line| {
                let coords: Vec<i64> = line
                    .split(',')
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect();
                (coords[0], coords[1], coords[2])
            })
    );

    let mut sorted_pairs: BinaryHeap<Reverse<Pair>> = BinaryHeap::from_iter(
        HashSet::from(
            lights
                .iter()
                .enumerate()
                .flat_map(|(i, a)| lights.iter().skip(i + 1).map(move |b| (a, b)))
                .map(|(a, b)| Pair::new(*a, *b))
                .collect::<HashSet<Pair>>()
        ).into_iter().map(|pair| Reverse(pair))
    );

    let mut point_ixs: HashMap<(i64, i64, i64), usize> = HashMap::new();
    let mut circuits: Vec<Circuit> = Vec::from_iter(
        lights.iter().enumerate().map(|(ix, point)| {
            point_ixs.insert(*point, ix);
            Circuit::from(*point)
        })
    );
    
    loop {
        let Reverse(pair) = sorted_pairs.pop().unwrap();
        if point_ixs[&pair.a] == point_ixs[&pair.b] {
            continue;
        }

        if circuits.len() == 2 {
            // This connection will fully connect, no need to actually do it
            return pair.a.0 * pair.b.0;
        }

        let new_circuit: Circuit = {
            let mut points = vec![pair.a, pair.b];
            points.sort_by(|a, b| point_ixs[b].cmp(&point_ixs[a]));
            points.iter().fold(Circuit::new(), |acc, point| acc.union(&circuits.remove(point_ixs[point])))
        };

        point_ixs = HashMap::from_iter(
            point_ixs.iter().map(|(point, ix)| {
                (*point,
                    if new_circuit.contains(point) {
                        circuits.len()
                    } else {
                        *ix - (*ix > point_ixs[&pair.a]) as usize - (*ix > point_ixs[&pair.b]) as usize
                    }
                )
            })
        );

        circuits.push(new_circuit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 40);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 25272);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2025".to_string();
    let day = "8".to_string();

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
        "\nPart 1:\nProduct of 3 largest circuits: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nProducts of x-coordinates of final connections: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}
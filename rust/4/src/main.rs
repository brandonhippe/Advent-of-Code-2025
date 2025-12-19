use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use convolutions_rs::convolutions::*;
use ndarray::*;
use convolutions_rs::Padding;

fn part1(contents: String) -> i64 {
    let mut input_arr: Array3<f32> = Array::from_shape_vec(
        (1, contents.lines().count(), contents.lines().next().unwrap().len()),
        contents
            .lines()
            .flat_map(|line| line.chars().map(|b| ((b == '@') as u8) as f32).collect::<Vec<f32>>())
            .collect(),
    ).unwrap();

    let kernel: Array4<f32> = Array::from_shape_vec(
        (1, 1, 3, 3),
        vec![
            1., 1., 1.,
            1., 0., 1.,
            1., 1., 1.,
        ],
    ).unwrap();

    let conv_layer = ConvolutionLayer::new(kernel.clone(), None, 1, Padding::Same);
    let output = conv_layer.convolve(&input_arr);
    input_arr = (input_arr - 1.) * (2. * kernel.sum());

    return (output + input_arr).into_iter().filter(|&x| x >= 0. && x < 4.).count() as i64;
}

fn part2(contents: String) -> i64 {
    let mut input_arr: Array3<f32> = Array::from_shape_vec(
        (1, contents.lines().count(), contents.lines().next().unwrap().len()),
        contents
            .lines()
            .flat_map(|line| line.chars().map(|b| ((b == '@') as u8) as f32).collect::<Vec<f32>>())
            .collect(),
    ).unwrap();

    let kernel: Array4<f32> = Array::from_shape_vec(
        (1, 1, 3, 3),
        vec![
            1., 1., 1.,
            1., 0., 1.,
            1., 1., 1.,
        ],
    ).unwrap();

    let conv_layer = ConvolutionLayer::new(kernel.clone(), None, 1, Padding::Same);

    let mut count: i64 = 0;
    loop {
        let mut output = conv_layer.convolve(&input_arr);
        input_arr = (input_arr - 1.) * (2. * kernel.sum());
        output += &input_arr;

        let modify_arr: Array3<f32> = output.mapv(|x| if x >= 0. && x < 4. {0.} else {1.});
        let modify_count = modify_arr.iter().filter(|&&x| x == 0.).count() as i64;

        if modify_count == 0 {
            break;
        }
        
        count += modify_count;
        input_arr = input_arr / (2. * kernel.sum()) + modify_arr;
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

        assert_eq!(part1(contents), 13);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 43);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2025".to_string();
    let day = "4".to_string();

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
        "\nPart 1:\nNumber of reachable rolls: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nNumber of reachable rolls: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}
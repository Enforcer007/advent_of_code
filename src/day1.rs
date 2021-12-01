// Link to the Challenge https://adventofcode.com/2021/day/1

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let firststar_ans = first_star();
    let secondstar_ans = second_star();

    println!("firstStar --> {}, secondStar --> {}", firststar_ans,secondstar_ans);
}

fn read_numbers_from_file() -> Vec<i32> {
    let file = File::open("data/day1input.txt").expect("File doesn't exist!");
    let reader = BufReader::new(file);
    let levels: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
    return levels;
}

fn get_incs(levels:Vec<i32>)->i32{
    let inc_depths: i32 = levels[1..]
        .iter()
        .zip(levels[..levels.len() - 1].iter())
        .map(|(x, y)| if x - y > 0 { 1 } else { 0 })
        .sum();
    return inc_depths;
}

fn first_star() -> i32 {
    let levels = read_numbers_from_file();
    let inc_depths = get_incs(levels);
    return inc_depths;
}

fn second_star() -> i32 {
    let levels = read_numbers_from_file();
    let composite_levels:Vec<_> = levels[..levels.len() - 2]
        .iter()
        .zip(levels[1..levels.len() - 1].iter())
        .zip(levels[2..].iter())
        .map(|((x,y),z)| x+y+z)
        .collect();
    let inc_depths = get_incs(composite_levels);
    return inc_depths;
}

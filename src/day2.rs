use super::lib_comp;

pub fn run() {
    let firststar_ans = first_star();
    let secondstar_ans = second_star();

    println!(
        "firstStar --> {}, secondStar --> {}",
        firststar_ans, secondstar_ans
    );
}

fn get_directions_levels() -> Vec<Vec<String>>{
    let lines = lib_comp::read_lines_from_file("data/day2input.txt");
    let directions = lines
        .iter()
        .map(|x| x.split(" ").map(|x| x.to_string()).collect::<Vec<String>>())
        .collect::<Vec<_>>();
    return directions
}

fn first_star() -> i32{
    let mut vertical = 0;
    let mut horizontal = 0;
    let directions = get_directions_levels();
    for x in directions {
        if x[0] == "forward" {
            horizontal += x[1].parse::<i32>().unwrap()
        } else if x[0] == "up" {
            vertical -= x[1].parse::<i32>().unwrap()
        } else {
            vertical += x[1].parse::<i32>().unwrap()
        }
    }
    return horizontal*vertical
}

fn second_star() -> i32{
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;
    let directions = get_directions_levels();
    for x in directions {
        let level = x[1].parse::<i32>().unwrap();
        if x[0] == "forward" {
            horizontal += level;
            depth+=aim*level;
        } else if x[0] == "up" {
            aim -= level;
        } else {
            aim += level;
        }
    }
    return horizontal*depth
}
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_lines_from_file(x: &str) -> Vec<String> {
    let file = File::open(x).expect("File doesn't exist!");
    let reader = BufReader::new(file);
    let levels: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    return levels;
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let mut depth: u32 = 0;
    let mut pos: u32 = 0;

    if let Ok(lines) = read_lines("data/day02_input.txt") {
        for line in lines {
            if let Ok(val) = line {
                let split = val.split_once(" ");
                let (cmd, amt_str) = split.unwrap();
                let amt: u32 = amt_str.parse::<u32>().unwrap();
                match cmd {
                    "down" => depth = depth + amt,
                    "up" => depth = depth - amt,
                    _ => pos = pos + amt,
                }
            }
        }
    }
    println!("day02 part a: horizontal pos {}", pos);
    println!("day02 part a: depth {}", depth);
    println!("day02 part a: product {}", pos * depth);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

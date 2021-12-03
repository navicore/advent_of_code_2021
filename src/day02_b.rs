use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    // product should be 1857958050
    let mut depth: u32 = 0;
    let mut pos: u32 = 0;
    let mut aim: u32 = 0;

    let mut update = |val: String| {
        let split = val.split_once(" ");
        let (cmd, amt_str) = split.unwrap();
        let amt: u32 = amt_str.parse::<u32>().unwrap();
        match cmd {
            "down" => aim = aim + amt,
            "up" => aim = aim - amt,
            _ => {
                pos = pos + amt;
                depth = depth + aim * amt;
            }
        }
    };

    if let Ok(lines) = read_lines("data/day02_input.txt") {
        for line in lines {
            if let Ok(val) = line {
                update(val);
            }
        }
    }

    println!("day02 part b: horizontal pos {}", pos);
    println!("day02 part b: depth {}", depth);
    println!("day02 part b: product {}", pos * depth);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

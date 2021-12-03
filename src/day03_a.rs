use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let mut count = 0;

    let mut update = |val: String| {
        count = count + 1;
    };

    if let Ok(lines) = read_lines("data/day02_input.txt") {
        for line in lines {
            if let Ok(val) = line {
                update(val);
            }
        }
    }

    println!("day03 part a: rec count {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

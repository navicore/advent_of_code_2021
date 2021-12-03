use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let mut prev: u32 = 0;
    let mut count = 0;
    let mut total_recs = 0;

    if let Ok(lines) = read_lines("data/day01_input.txt") {
        for line in lines {
            if let Ok(val) = line {
                let nval: u32 = val.parse::<u32>().unwrap();
                if prev > 0 && nval > prev {
                    count = count + 1;
                }
                total_recs = total_recs + 1;
                prev = nval;
            }
        }
    }
    println!("day01 part a: total measurments {}", total_recs);
    println!("day01 part a: increasing measurement count {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

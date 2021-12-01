use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let mut prev = 0;
    let mut count = 0;
    let mut total_recs = 0;

    if let Ok(lines) = read_lines("data/day1_input.txt") {
        for line in lines {
            if let Ok(val) = line {
                let nval: u32 = val.parse::<u32>().unwrap();
                if nval > prev {
                    count = count + 1;
                }
                total_recs = total_recs + 1;
                prev = nval;
            }
        }
    }
    println!("total recs {}", total_recs);
    println!("count {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

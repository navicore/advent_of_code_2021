use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let mut prev1: u32 = 0;
    let mut prev2: u32 = 0;
    let mut prev_window: u32 = 0;
    let mut total_windows: u32 = 0;
    let mut window_incr_count: u32 = 0;

    if let Ok(lines) = read_lines("data/day01_input.txt") {
        for line in lines {
            if let Ok(val) = line {
                let nval: u32 = val.parse::<u32>().unwrap();
                if prev1 > 0 && prev2 > 0 {
                    total_windows = total_windows + 1;
                    let curr_window = prev1 + prev2 + nval;
                    if prev_window > 0 && curr_window > prev_window {
                        window_incr_count = window_incr_count + 1;
                    }
                    prev_window = curr_window;
                }
                total_windows = total_windows + 1;
                prev1 = prev2;
                prev2 = nval;
            }
        }
    }

    println!("day01 part b: total windows {}", total_windows);
    println!(
        "day01 part b: increasing window count {}",
        window_incr_count
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

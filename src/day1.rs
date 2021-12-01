use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part_a() {
    let mut prev: u32 = 0;
    let mut count = 0;
    let mut total_recs = 0;

    if let Ok(lines) = read_lines("data/day1_input.txt") {
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
    println!("day1 part a: total measurments {}", total_recs);
    println!("day1 part a: increasing measurement count {}", count);
}

pub fn part_b() {
    let mut prev1: u32 = 0;
    let mut prev2: u32 = 0;
    let mut prev_window: u32 = 0;
    let mut total_windows: u32 = 0;
    let mut window_incr_count: u32 = 0;

    if let Ok(lines) = read_lines("data/day1_input.txt") {
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

    println!("day1 part b: total windows {}", total_windows);
    println!("day1 part b: increasing window count {}", window_incr_count);
}

pub fn run() {
    println!("------------");
    part_a();
    println!("............");
    part_b();
    println!("------------");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let mut counts: [u32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let rec_len = 12;
    let mut gamma = vec![false; rec_len];
    let mut epsilon = vec![false; rec_len];
    let mut rec_count: u32 = 0;

    let mut update = |val: String| {
        let my_counts = &mut counts;
        let chars: Vec<char> = val.chars().collect();
        let mut idx = 0;
        for c in chars {
            if c == '1' {
                my_counts[idx] = my_counts[idx] + 1;
            }
            idx = idx + 1;
        }
        rec_count = rec_count + 1;
    };

    let mut calc = |my_rec_count: u32, my_counts: &[u32; 12]| {
        let mut idx = 0;
        let half_way = my_rec_count / 2;
        for _ in *my_counts {
            if my_counts[idx] > half_way {
                gamma[idx] = true;
            } else {
                epsilon[idx] = true;
            }
            idx = idx + 1;
        }
    };

    fn value_from_binstr(bin_idx: String) -> isize {
        isize::from_str_radix(bin_idx.as_str(), 2).unwrap()
    }

    if let Ok(lines) = read_lines("data/day03_input.txt") {
        for line in lines {
            if let Ok(val) = line {
                update(val);
            }
        }
        calc(rec_count, &counts);
    }

    println!("day03 part a: recs {}", rec_count);
    println!("day03 part a: measures {:?}", counts);
    println!("day03 part a: gamma {:?}", gamma);
    println!("day03 part a: epsilon {:?}", epsilon);

    let mut gamma_str = String::new();
    for x in &gamma {
        if *x == true {
            gamma_str.push('1');
        } else {
            gamma_str.push('0');
        }
    }
    let gamma_val = value_from_binstr(gamma_str);
    println!("day03 part a: gamma val {}", gamma_val);

    let mut epsilon_str = String::new();
    for x in &epsilon {
        if *x == true {
            epsilon_str.push('1');
        } else {
            epsilon_str.push('0');
        }
    }
    let epsilon_val = value_from_binstr(epsilon_str);
    println!("day03 part a: epsilon val {}", epsilon_val);

    println!("day03 part a: answer {}", gamma_val * epsilon_val);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

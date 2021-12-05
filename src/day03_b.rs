use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub fn run() {
    let lines: Vec<String> = lines_from_file("data/day03_input.txt");

    let oxygen_gen_rating = filter(&lines, 'g', 0).unwrap();
    let oxygen_gen_rating_value = value_from_binstr(&oxygen_gen_rating);
    println!(
        "day03 part b: oxygen gen {:?} {:?}",
        &oxygen_gen_rating, &oxygen_gen_rating_value
    );
    let co2_scrubber_rating = filter(&lines, 'e', 0).unwrap();
    let co2_scrubber_rating_value = value_from_binstr(&co2_scrubber_rating);
    println!(
        "day03 part b: co2 scrubber {:?} {:?}",
        &co2_scrubber_rating, &co2_scrubber_rating_value
    );

    let answer = oxygen_gen_rating_value * co2_scrubber_rating_value;

    println!("answer: {}", &answer);
}

fn value_from_binstr(bin_idx: &String) -> isize {
    isize::from_str_radix(bin_idx.as_str(), 2).unwrap()
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

//let mut calc_gamma_and_epsilon =
fn calc_gamma_and_epsilon(my_rec_count: u32, my_counts: &[u32; 12]) -> (Vec<bool>, Vec<bool>) {
    let rec_len = 12;
    let mut idx = 0;
    let half_way = my_rec_count / 2;
    let mut gamma = vec![false; rec_len];
    let mut epsilon = vec![false; rec_len];
    for _ in *my_counts {
        if my_counts[idx] == half_way {
            gamma[idx] = true;
            epsilon[idx] = true;
        } else if my_counts[idx] > half_way {
            gamma[idx] = true;
            epsilon[idx] = false;
        } else {
            gamma[idx] = false;
            epsilon[idx] = true;
        }
        idx = idx + 1;
    }
    (gamma, epsilon)
}

fn filter(lines: &Vec<String>, selector: char, idx: usize) -> Option<String> {
    let mut counts: [u32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut rec_count: u32 = 0;
    let mut count_ones = |val: &String| {
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
    for line in lines {
        count_ones(line);
    }
    let mut new_lines = lines.to_vec();

    let (gamma, epsilon) = calc_gamma_and_epsilon(rec_count, &counts);
    let pattern = match selector {
        'g' => gamma,
        _ => epsilon,
    };
    new_lines.retain(|l| {
        let chars: Vec<char> = l.chars().collect();
        match (pattern[idx], chars[idx]) {
            (true, '1') => true,
            (false, '0') => true,
            _ => false,
        }
    });
    if new_lines.len() == 1 {
        let l = new_lines.get(0).cloned();
        match l {
            Some(lo) => Some(String::from(lo)),
            _ => None, // GETTING HERE THIS IS BAD
        }
    } else if idx < 11 {
        filter(&new_lines, selector, idx + 1)
    } else {
        None
    }
}

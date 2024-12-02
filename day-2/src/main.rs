use std::{
    fs::File,
    io::{self, BufRead},
};

const FILENAME: &str = "assets/input";

fn main() {
    let lines = load_from_file().expect("Could not load from file");

    // Part 1
    let mut total_safe_lines = 0;

    for line in &lines {
        let is_safe = is_line_safe(line.clone());

        if is_safe {
            total_safe_lines += 1;
        }
    }

    println!("Total safe lines: {}", total_safe_lines);

    // Part 2
    total_safe_lines = 0;

    for line in &lines {
        let is_safe = is_line_safe(line.clone());

        if is_safe {
            total_safe_lines += 1;
            continue;
        }

        for i in 0..line.len() {
            let mut slice = line.clone();
            slice.remove(i);

            let is_safe = is_line_safe(slice);

            if is_safe {
                total_safe_lines += 1;
                break;
            }
        }
    }

    println!("Total safe lines with tolerance: {}", total_safe_lines);
}

enum Order {
    Increasing,
    Descreasing,
}

fn is_line_safe(line: Vec<usize>) -> bool {
    if line.len() == 1 {
        return true;
    }

    // Equal does not affect, since it is checked in the distance
    let order = if line[0] < line[1] {
        Order::Increasing
    } else {
        Order::Descreasing
    };

    for pair in line.windows(2) {
        let distance = pair[0].abs_diff(pair[1]);

        if distance < 1 || distance > 3 {
            return false;
        }

        match order {
            Order::Increasing => {
                if pair[0] > pair[1] {
                    return false;
                }
            }
            Order::Descreasing => {
                if pair[0] < pair[1] {
                    return false;
                }
            }
        }
    }

    true
}

fn load_from_file() -> Result<Vec<Vec<usize>>, String> {
    let file: File = File::open(FILENAME).expect("no file found");
    let reader = io::BufReader::new(file);

    let mut lines_levels: Vec<Vec<usize>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        let numbers: Vec<usize> = parts.iter().filter_map(|part| part.parse().ok()).collect();

        if !numbers.is_empty() {
            lines_levels.push(numbers);
        }
    }

    Ok(lines_levels)
}

use std::{fs::File, io::{self, BufRead}};

const FILENAME: &str = "assets/input";

fn main() {
    let ( mut left_numbers, mut right_numbers) = load_from_file().expect("Could not load from file");

    // Part 1
    left_numbers.sort();
    right_numbers.sort();

    let mut total_distance = 0;
    
    let numbers_iter = left_numbers.iter().zip(right_numbers.iter());
    for (left, right) in numbers_iter {
        let distance = left.abs_diff(*right);
        total_distance += distance;
    }

    println!("Total distance: {:?}", total_distance);

    // Part 2
    let mut total_similary_score = 0;

    for left in left_numbers.iter() {
        let count = right_numbers.iter().filter(|&right| left == right).count();

        total_similary_score += count * left;
    }

    println!("Total similarity score: {:?}", total_similary_score);

}


pub fn load_from_file() -> Result<(Vec<usize>, Vec<usize>), String> {
    let file: File = File::open(FILENAME).expect("no file found");
    let reader = io::BufReader::new(file);

    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() != 2 {
            return Err("Line does not have two parts".into());
        }

        let left_number = parts[0]
            .parse::<usize>()
            .map_err(|_| "Failed to parse left number".to_string())?;

        let right_number = parts[1]
            .parse::<usize>()
            .map_err(|_| "Failed to parse right number".to_string())?;

        left_numbers.push(left_number);
        right_numbers.push(right_number);
    }

    Ok((left_numbers, right_numbers))
}

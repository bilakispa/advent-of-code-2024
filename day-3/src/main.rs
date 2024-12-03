use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
};

const FILENAME: &str = "assets/input";

fn main() {
    let lines = load_from_file().expect("Could not load from file");
    let joined_lines = lines.join("");

    // Part 1
    let valid_instructions = get_valid_instructions(&joined_lines);

    let total_multiplications = get_total_multiplications(valid_instructions);

    println!("Sum of valid multiplications: {}", total_multiplications);

    // Part 2
    let joined_lines_without_disabled = remove_disabled_instructions(&joined_lines);
    let valid_instructions = get_valid_instructions(&joined_lines_without_disabled);

    let total_multiplications = get_total_multiplications(valid_instructions);

    println!(
        "Sum of valid multiplications (without disabled): {}",
        total_multiplications
    );
}

fn get_valid_instructions(line: &str) -> Vec<(usize, usize)> {
    let mut valid_instructions = Vec::new();

    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for captures in regex.captures_iter(line) {
        let left_number = captures[1].parse::<usize>().unwrap();
        let right_number = captures[2].parse::<usize>().unwrap();

        valid_instructions.push((left_number, right_number));
    }

    valid_instructions
}

fn remove_disabled_instructions(line: &str) -> String {
    let regex = Regex::new(r"don't\(\)(.*?)(do\(\)|$)").unwrap();

    regex.replace_all(line, "").to_string()
}

fn get_total_multiplications(instructions: Vec<(usize, usize)>) -> usize {
    instructions.iter().map(|(left, right)| left * right).sum()
}

fn load_from_file() -> io::Result<Vec<String>> {
    let file: File = File::open(FILENAME).expect("no file found");
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

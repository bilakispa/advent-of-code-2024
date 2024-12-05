use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
};

const FILENAME: &str = "assets/input";
const PADDING_SIZE: usize = 3;

fn main() {
    let lines = load_from_file().expect("Could not load from file");
    let padded_lines = add_padding_to_lines(&lines);

    // Part 1
    let x_pairs = find_index_pairs_of_character(&lines, 'X');
    let s_pairs = find_index_pairs_of_character(&lines, 'S');

    let mut xmas_words = 0;
    xmas_words += word_search_full(&padded_lines, x_pairs, &['M', 'A', 'S']);
    xmas_words += word_search_full(&padded_lines, s_pairs, &['A', 'M', 'X']);

    println!("\"XMAS\" words found: {}", xmas_words);

    // Part 2
    let a_pairs = find_index_pairs_of_character(&lines, 'A');

    let cross_mas_words = word_search_cross(&padded_lines, a_pairs);

    println!("Cross \"MAS\" words found: {}", cross_mas_words);
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

fn add_padding_to_lines(lines: &[String]) -> Vec<String> {
    let mut padded_lines = Vec::new();

    let row_length = lines[0].len();
    let padded_row_length = row_length + 2 * PADDING_SIZE;
    let padding_row: String = ".".repeat(padded_row_length);

    // Top padding
    for _ in 0..PADDING_SIZE {
        padded_lines.push(padding_row.clone());
    }

    for line in lines {
        let padding_columns = ".".repeat(PADDING_SIZE);
        let padded_line = format!("{}{}{}", padding_columns, line, padding_columns);

        padded_lines.push(padded_line);
    }

    // Bottom padding
    for _ in 0..PADDING_SIZE {
        padded_lines.push(padding_row.clone());
    }

    padded_lines
}

fn find_index_pairs_of_character(lines: &[String], character: char) -> Vec<(usize, usize)> {
    let pattern = regex::escape(&character.to_string());
    let regex = Regex::new(&pattern).unwrap();
    let mut pairs = Vec::new();

    for (x, line) in lines.iter().enumerate() {
        let found_pair: Vec<(usize, usize)> =
            regex.find_iter(line).map(|mat| (x, mat.start())).collect();

        pairs.extend(found_pair);
    }

    pairs
}

fn word_search_full(
    padded_lines: &[String],
    first_character_pairs: Vec<(usize, usize)>,
    remaining_characters: &[char],
) -> usize {
    let mut words_found = 0;

    for (x, y) in first_character_pairs {
        let row = x + PADDING_SIZE;
        let column = y + PADDING_SIZE;

        let mut lines_slice = Vec::new();
        for padded_line in padded_lines.iter().take(row + PADDING_SIZE + 1) {
            lines_slice.push(padded_line.chars().collect::<Vec<char>>());
        }

        if check_horizontal(&lines_slice, row, column, remaining_characters) {
            words_found += 1;
        }

        if check_vertical(&lines_slice, row, column, remaining_characters) {
            words_found += 1;
        }

        if check_diagonal_bottom_right(&lines_slice, row, column, remaining_characters) {
            words_found += 1;
        }

        if check_diagonal_bottom_left(&lines_slice, row, column, remaining_characters) {
            words_found += 1;
        }
    }

    words_found
}

fn check_horizontal(lines: &[Vec<char>], row: usize, column: usize, characters: &[char]) -> bool {
    for (i, character) in characters.iter().enumerate() {
        if lines[row].get(column + (i + 1)).unwrap() != character {
            return false;
        }
    }

    true
}

fn check_vertical(lines: &[Vec<char>], row: usize, column: usize, characters: &[char]) -> bool {
    for (i, character) in characters.iter().enumerate() {
        if lines[row + (i + 1)].get(column).unwrap() != character {
            return false;
        }
    }

    true
}

fn check_diagonal_bottom_right(
    lines: &[Vec<char>],
    row: usize,
    column: usize,
    characters: &[char],
) -> bool {
    for (i, character) in characters.iter().enumerate() {
        if lines[row + (i + 1)].get(column + (i + 1)).unwrap() != character {
            return false;
        }
    }

    true
}

fn check_diagonal_bottom_left(
    lines: &[Vec<char>],
    row: usize,
    column: usize,
    characters: &[char],
) -> bool {
    for (i, character) in characters.iter().enumerate() {
        if lines[row + (i + 1)].get(column - (i + 1)).unwrap() != character {
            return false;
        }
    }

    true
}

fn word_search_cross(padded_lines: &[String], first_character_pairs: Vec<(usize, usize)>) -> usize {
    let mut words_found = 0;

    for (x, y) in first_character_pairs {
        let row = x + PADDING_SIZE;
        let column = y + PADDING_SIZE;

        let mut lines_slice = Vec::new();
        for padded_line in padded_lines.iter().take(row + PADDING_SIZE + 1) {
            lines_slice.push(padded_line.chars().collect::<Vec<char>>());
        }

        let top_left = *lines_slice[row - 1].get(column - 1).unwrap();
        let top_right = *lines_slice[row - 1].get(column + 1).unwrap();
        let bottom_left = *lines_slice[row + 1].get(column - 1).unwrap();
        let bottom_right = *lines_slice[row + 1].get(column + 1).unwrap();

        if !is_matching_s_m_pair((top_left, bottom_right)) {
            continue;
        }

        if !is_matching_s_m_pair((top_right, bottom_left)) {
            continue;
        }

        words_found += 1;
    }

    words_found
}

fn is_matching_s_m_pair((left, right): (char, char)) -> bool {
    match left {
        'S' => right == 'M',
        'M' => right == 'S',
        _ => false,
    }
}

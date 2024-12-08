use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

const FILENAME: &str = "assets/input";

fn main() {
    let (order_rules, page_updates) = load_from_file();

    let order_rules_map = get_order_rules_map(order_rules);
    let (correct_order_updates, incorrect_order_updates) =
        get_updates_ordered_pair(page_updates, &order_rules_map);

    // Part 1
    let sum_middle_numbers_correct_order = sum_of_middle_pages(correct_order_updates);

    println!(
        "Sum of middle numbers from correct order updates: {}",
        sum_middle_numbers_correct_order
    );

    // Part 2
    let mut sorted_updates = Vec::new();
    for incorrect_order_update in incorrect_order_updates {
        let mut sorted_update = incorrect_order_update.clone();
        sorted_update.sort_by(|a, b| is_correct_order(a, b, &order_rules_map).cmp(&true));

        sorted_updates.push(sorted_update);
    }

    let sum_middle_numbers_sorted = sum_of_middle_pages(sorted_updates);

    println!(
        "Sum of middle numbers from incorrect order updates (now sorted as correct): {}",
        sum_middle_numbers_sorted
    );
}

fn load_from_file() -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let file: File = File::open(FILENAME).expect("no file found");
    let reader = io::BufReader::new(file);

    let mut page_order_rules = Vec::new();
    let mut page_updates = Vec::new();

    let mut is_adding_page_order_rules = true;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            is_adding_page_order_rules = false;
            continue;
        }

        if is_adding_page_order_rules {
            let rule: Vec<&str> = line.split('|').collect();

            let smaller = rule[0].parse::<usize>().ok().unwrap();
            let bigger = rule[1].parse::<usize>().ok().unwrap();

            page_order_rules.push((smaller, bigger));
        } else {
            let update = line.split(",");

            let pages: Vec<usize> = update
                .filter_map(|page| page.trim().parse::<usize>().ok())
                .collect();

            page_updates.push(pages);
        }
    }

    (page_order_rules, page_updates)
}

fn get_order_rules_map(order_rules: Vec<(usize, usize)>) -> HashMap<usize, Vec<usize>> {
    let mut rules_map = HashMap::new();
    for (smaller, bigger) in order_rules {
        rules_map.entry(smaller).or_default();

        let bigger_arr: &mut Vec<usize> = rules_map.get_mut(&smaller).unwrap();
        bigger_arr.push(bigger);
    }

    rules_map
}

fn get_updates_ordered_pair(
    page_updates: Vec<Vec<usize>>,
    order_rules_map: &HashMap<usize, Vec<usize>>,
) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut correct_order_updates = Vec::new();
    let mut incorrect_order_updates = Vec::new();
    for page_update in page_updates {
        let is_correct_order =
            page_update.is_sorted_by(|a, b| is_correct_order(a, b, order_rules_map));

        if is_correct_order {
            correct_order_updates.push(page_update);
        } else {
            incorrect_order_updates.push(page_update);
        }
    }

    (correct_order_updates, incorrect_order_updates)
}

fn is_correct_order(a: &usize, b: &usize, order_rules_map: &HashMap<usize, Vec<usize>>) -> bool {
    let next = order_rules_map
        .get(b)
        .unwrap_or(&Vec::<usize>::default())
        .clone();

    !next.contains(a)
}

fn sum_of_middle_pages(updates: Vec<Vec<usize>>) -> usize {
    // We assume that the arrays have always an even number of elements
    updates.iter().map(|update| update[update.len() / 2]).sum()
}

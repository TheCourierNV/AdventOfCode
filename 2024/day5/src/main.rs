use std::{cmp::Ordering, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let lines: Vec<&str> = input.lines().collect();

    // 🤮
    let input_sections: Vec<&[&str]> = lines.splitn(2, |line| line.is_empty()).collect();

    for rule in input_sections[0] {
        let page_numbers: Vec<u32> = rule
            .split('|')
            .map(|page| page.parse().expect("Invalid page number in rule!"))
            .collect();

        rules.push((page_numbers[0], page_numbers[1]));
    }

    for update in input_sections[1] {
        updates.push(
            update
                .split(',')
                .map(|page_number| page_number.parse().expect("Invalid page number in update!"))
                .collect(),
        );
    }

    (rules, updates)
}

fn ordering_function_bool(rules: &Vec<(u32, u32)>, a: u32, b: u32) -> bool {
    for rule in rules {
        if rule.1 == a && rule.0 == b {
            return false;
        }
    }

    true
}

fn ordering_function_ordering(rules: &Vec<(u32, u32)>, a: u32, b: u32) -> Ordering {
    for rule in rules {
        if rule.1 == a && rule.0 == b {
            return Ordering::Greater;
        }
        if rule.1 == b && rule.0 == a {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

fn part_one(input: &str) -> u32 {
    let mut result = 0;

    let (rules, updates) = parse_input(input);

    for update in updates {
        if update.is_sorted_by(|a, b| ordering_function_bool(&rules, *a, *b)) {
            result += update[update.len() / 2];
        }
    }

    result
}

fn part_two(input: &str) -> u32 {
    let mut result = 0;

    let (rules, updates) = parse_input(input);

    for mut update in updates {
        if !update.is_sorted_by(|a, b| ordering_function_bool(&rules, *a, *b)) {
            update.sort_by(|a, b| ordering_function_ordering(&rules, *a, *b));

            result += update[update.len() / 2];
        }
    }

    result
}

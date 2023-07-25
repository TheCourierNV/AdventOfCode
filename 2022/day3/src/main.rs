use std::fs;

fn main() {
    println!("Welcome to Ruck-sacker");

    let input = fs::read_to_string("input.txt").expect("Cannot read input.txt");

    println!("{}", get_total_priority_v2(input));
}

fn get_total_priority(input: &String) -> u32 {
    let mut total_priority = 0;

    for line in input.lines() {
        let common_item = get_common_item(line).expect("Invalid rucksack");

        total_priority += get_item_priority(&common_item);

        println!("{}", common_item);
    }

    total_priority
}

fn get_common_item(rucksack: &str) -> Option<char> {
    let compartment_start = rucksack.len() / 2;

    let first_compartment = &rucksack[..compartment_start];
    let second_compartment = &rucksack[compartment_start..];

    for first_compartment_item in first_compartment.chars() {
        for second_compartment_item in second_compartment.chars() {
            if first_compartment_item == second_compartment_item {
                return Some(first_compartment_item);
            }
        }
    }

    None
}

fn get_item_priority(item: &char) -> u32 {
    let mut result = 0;

    if item.is_ascii_uppercase() {
        result += 26;
    }

    let lowercase_item = item.to_ascii_lowercase();

    let alphabet = 'a'..'z';

    for (priority, letter) in alphabet.enumerate() {
        if letter == lowercase_item {
            result += priority as u32;
            break;
        }
    }

    result
}

fn get_common_item_v2(first: &str, second: &str, third: &str) -> Option<char> {
    for potential_item in first.chars() {
        for a in second.chars() {
            for b in third.chars() {
                if potential_item == a && potential_item == b {
                    return Some(potential_item);
                }
            }
        }
    }
    None
}

fn get_total_priority_v2(input: String) -> u32 {
    let mut total_priority = 0;
    let mut all_lines = input.lines();

    loop {
        let line = all_lines.next();

        let first: &str;

        match line {
            None => break,
            Some(rucksack) => {
                first = rucksack;
            }
        }

        if first.is_empty() {
            continue;
        }

        let second = all_lines
            .next()
            .expect("Invalid input: missing two rucksacks at the end");

        let third = all_lines
            .next()
            .expect("Invalid input: missing one rucksack at the end");

        let common_item = get_common_item_v2(first, second, third)
            .expect("Invalid rucksacks: no common element found");

        let item_priority = get_item_priority(&common_item);

        total_priority += item_priority;

        println!("{} = {}", common_item, item_priority);
    }

    total_priority
}

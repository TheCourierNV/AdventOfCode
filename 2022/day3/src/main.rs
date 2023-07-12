use std::fs;

fn main() {
    println!("Welcome to Ruck-sacker");

    let input = fs::read_to_string("input.txt").expect("Cannot read input.txt");

    println!("{}", get_total_priority(input));
}

fn get_total_priority(input: String) -> u32 {
    let mut total_priority = 0;

    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }

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

    let mut priority = 1;

    for letter in alphabet {
        if letter == lowercase_item {
            break;
        }

        priority += 1;
    }

    result += priority;

    result
}

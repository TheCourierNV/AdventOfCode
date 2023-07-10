use std::fs;

fn main() {
    println!("Welcome to Elf-O-Matic");

    let input = fs::read_to_string("input.txt").expect("Cannot read input.txt");

    let answer = get_top_n_elves(input, 3);
    println!("Max calories: {} {} {}", answer[0], answer[1], answer[2]);
    println!("Answer: {}", answer[0] + answer[1] + answer[2]);

    // let answer = get_max_elf(input);
    // println!("Max calories: {}", answer);
}

fn get_top_n_elves(input: String, n: usize) -> Vec<u32> {
    let mut elves = get_all_elves(input);

    let mut result: Vec<u32> = Vec::new();

    elves.sort_unstable();

    for _ in 0..n {
        result.push(elves.pop().expect("Ran out of elves: N was too big"));
    }

    result
}

fn get_all_elves(input: String) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    let input = input.split("\n");

    let mut current_elf = 0;

    for line in input {
        if line.is_empty() {
            result.push(current_elf);

            current_elf = 0;

            continue;
        }

        current_elf += get_calories(line);
    }

    result
}

fn get_max_elf(input: String) -> u32 {
    let mut max_elf = 0;

    let input = input.split("\n");

    let mut current_elf: u32 = 0;
    for line in input {
        if line.is_empty() {
            if max_elf < current_elf {
                max_elf = current_elf
            }

            current_elf = 0;

            continue;
        }

        current_elf += get_calories(line);
    }

    max_elf
}

fn get_calories(input: &str) -> u32 {
    input.parse().expect("Failed to parse calories")
}

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn get_battery_banks(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit_char| digit_char.to_digit(10).expect("Invalid joltage"))
                .collect()
        })
        .collect()
}

fn get_max_joltage(battery_banks: Vec<Vec<u32>>, joltage_length: usize) -> u64 {
    // We calculate the joltage by using a greedy algorithm, following this logic:
    // - Find leftmost biggest digit (as long as it's not too close to the end)
    // - Chop off digits preceding (and including) the one we just found
    // - Repeat

    let mut max_joltage = 0;

    for battery_bank in battery_banks {
        let mut start_idx = -1;

        let mut battery_joltage = 0;

        for calculated_joltage_digits in 0..joltage_length {
            let (max_idx, max_digit) = battery_bank
                .iter()
                // Ignore digits we already selected in previous iterations
                .skip((start_idx + 1) as usize)
                .enumerate()
                // Here we reverse for two reasons:
                // 1) To ensure we .skip elements at the end
                // 2) Because .max_by picks the *rightmost* maximum in case of a tie, while we want the leftmost
                .rev()
                // Ensure we have enough space left at the end to fit the rest of the joltage number
                .skip(joltage_length - 1 - calculated_joltage_digits)
                .max_by(|a, b| a.1.cmp(b.1))
                .unwrap();

            // Tack on the digit we just found at the *end* of the partial result
            battery_joltage = battery_joltage * 10 + *max_digit as u64;

            start_idx += max_idx as i32 + 1;
        }

        max_joltage += battery_joltage;
    }

    max_joltage
}

fn part_one(input: &str) -> u64 {
    get_max_joltage(get_battery_banks(input), 2)
}

fn part_two(input: &str) -> u64 {
    get_max_joltage(get_battery_banks(input), 12)
}

use std::{collections::HashMap, fs, thread};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file!");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn get_stones(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|a| a.parse().expect("Invalid stone number!"))
        .collect()
}

fn part_one(input: &str) -> usize {
    let mut stones = get_stones(input);

    for _ in 0..25 {
        let mut new_stones = Vec::new();

        for stone in &stones {
            // Again, could do this with a base 10 log
            let digit_count = stone.to_string().len() as u32;

            if *stone == 0 {
                new_stones.push(1);
            } else if digit_count % 2 == 0 {
                let upper_digits = stone / 10_u64.pow(digit_count / 2);
                let lower_digits = stone % 10_u64.pow(digit_count / 2);

                new_stones.push(upper_digits);
                new_stones.push(lower_digits);
            } else {
                new_stones.push(stone * 2024);
            }
        }

        stones = new_stones;
    }

    stones.len()
}

fn process_stone(stone: u64, generation: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if generation == 75 {
        return 1;
    }

    if cache.contains_key(&(stone, generation)) {
        return *cache.get(&(stone, generation)).unwrap();
    }

    let digit_count = stone.to_string().len() as u32;
    let total_descendents;

    if stone == 0 {
        total_descendents = process_stone(1, generation + 1, cache);
    } else if digit_count % 2 == 0 {
        let upper_digits = stone / 10_u64.pow(digit_count / 2);
        let lower_digits = stone % 10_u64.pow(digit_count / 2);

        total_descendents = process_stone(upper_digits, generation + 1, cache)
            + process_stone(lower_digits, generation + 1, cache);
    } else {
        total_descendents = process_stone(stone * 2024, generation + 1, cache);
    }

    cache.insert((stone, generation), total_descendents);

    total_descendents
}

fn part_two(input: &str) -> usize {
    let stones = get_stones(input);

    let mut thread_joins = Vec::new();

    for stone in stones {
        thread_joins.push(thread::spawn(move || {
            process_stone(stone, 0, &mut HashMap::new())
        }));
    }

    thread_joins
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .sum()
}

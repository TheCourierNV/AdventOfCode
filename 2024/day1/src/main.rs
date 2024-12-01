use std::{
    cmp::{self},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn get_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let input_words: Vec<&str> = input.split_whitespace().collect();

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for (i, word) in input_words.iter().enumerate() {
        let id = word.parse().expect("Invalid ID in input file!");

        if i % 2 == 0 {
            list1.push(id);
        } else {
            list2.push(id);
        }
    }

    (list1, list2)
}

fn part_one(input: &str) -> u32 {
    let mut result = 0;

    let (mut list1, mut list2) = get_lists(input);

    list1.sort_unstable();
    list2.sort_unstable();

    for i in 0..list1.len() {
        let bigger = cmp::max(list1[i], list2[i]);
        let smaller = cmp::min(list1[i], list2[i]);

        result += bigger - smaller;
    }

    result
}

fn part_two(input: &str) -> u32 {
    let mut result = 0;

    let (list1, list2) = get_lists(input);

    for id in &list1 {
        let mut duplicate_counter = 0;

        for id2 in &list2 {
            if *id == *id2 {
                duplicate_counter += 1;
            }
        }

        result += id * duplicate_counter;
    }

    result
}

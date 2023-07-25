use std::{collections::VecDeque, fs};

fn main() {
    println!("Welcome to Crane-O-Matic");

    let input = fs::read_to_string("input.txt").expect("Cannot read input.txt");

    let stacks = get_final_state_v2(&input);

    for stack in stacks {
        print!("{}", stack[0]);
    }

    println!("");
}

fn get_initial_state(input: &String) -> Vec<VecDeque<char>> {
    let mut state: Vec<VecDeque<char>> = vec![VecDeque::new(); get_stack_count(input)];

    for line in input.lines() {
        if !line.contains("[") {
            break;
        }

        for (i, ch) in line.chars().enumerate() {
            // Le "casse" si trovano sempre in posizioni successive ad un multiplo di 4 (es: 1, 5, 9, 13...)
            if i % 4 == 1 && !ch.is_whitespace() {
                state[i / 4].push_back(ch);
            }
        }
    }

    state
}

fn get_final_state(input: &String) -> Vec<VecDeque<char>> {
    let mut state = get_initial_state(input);

    for line in input.lines() {
        if !line.contains("move") {
            continue;
        }

        let words: Vec<&str> = line.split_whitespace().collect();

        let crate_amount: usize = words[1]
            .parse()
            .expect("Invalid input: could not parse the amount of crates to move");

        let source_stack_index: usize = words[3]
            .parse::<usize>()
            .expect("Invalid input: could not parse the source stack")
            - 1;

        let destination_stack_index: usize = words[5]
            .parse::<usize>()
            .expect("Invalid input: could not parse the destination stack")
            - 1;

        for _ in 0..crate_amount {
            let moved_crate = state[source_stack_index]
                .pop_front()
                .expect("Ran out of crates in a stack");

            state[destination_stack_index].push_front(moved_crate);
        }
    }

    state
}

fn get_final_state_v2(input: &String) -> Vec<VecDeque<char>> {
    let mut state = get_initial_state(input);

    for line in input.lines() {
        if !line.contains("move") {
            continue;
        }

        let words: Vec<&str> = line.split_whitespace().collect();

        let crate_amount: usize = words[1]
            .parse()
            .expect("Invalid input: could not parse the amount of crates to move");

        let source_stack_index: usize = words[3]
            .parse::<usize>()
            .expect("Invalid input: could not parse the source stack")
            - 1;

        let destination_stack_index: usize = words[5]
            .parse::<usize>()
            .expect("Invalid input: could not parse the destination stack")
            - 1;

        let mut moved_crates: Vec<char> = Vec::new();

        for _ in 0..crate_amount {
            let moved_crate = state[source_stack_index]
                .pop_front()
                .expect("Ran out of crates in a stack");

            moved_crates.push(moved_crate);
        }

        // Visto che durante l'inserimento viene invertito l'ordine delle casse inserite, se vogliamo mantenere l'ordine originale dobbiamo invertire l'ordine di inserimento
        moved_crates.reverse();

        for moved_crate in moved_crates {
            state[destination_stack_index].push_front(moved_crate);
        }
    }

    state
}

fn get_stack_count(input: &String) -> usize {
    for line in input.lines() {
        return (line.len() + 1) / 4;
    }

    0
}

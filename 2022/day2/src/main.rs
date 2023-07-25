use std::fs;

#[derive(Debug, PartialEq, Clone)]
enum Moves {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
enum Outcomes {
    Defeat = 0,
    Draw = 3,
    Victory = 6,
}

fn main() {
    println!("Welcome to RPS-Calculator!");

    let input = fs::read_to_string("input.txt").expect("Cannot read input.txt");

    println!("Final score: {}", get_final_score_v2(input));
}

fn get_final_score(input: String) -> u32 {
    let mut current_score = 0;

    for line in input.lines() {
        let enemy_move = get_enemy_move(&line.chars().nth(0).expect("Enemy move not found"));
        let my_move = get_my_move(&line.chars().nth(2).expect("Counter move not found"));

        let outcome = if my_move == enemy_move {
            Outcomes::Draw
        } else if (my_move == Moves::Rock && enemy_move == Moves::Scissors)
            || (my_move == Moves::Paper && enemy_move == Moves::Rock)
            || (my_move == Moves::Scissors && enemy_move == Moves::Paper)
        {
            Outcomes::Victory
        } else {
            Outcomes::Defeat
        };

        println!("Moves: {:?} {:?}", enemy_move, my_move);
        println!("Outcome: {:?}", outcome);

        current_score += my_move as u32 + outcome as u32;
    }

    current_score
}

fn get_final_score_v2(input: String) -> u32 {
    let mut current_score = 0;

    for line in input.lines() {
        let enemy_move = get_enemy_move(&line.chars().nth(0).expect("Enemy move not found"));
        let outcome = get_outcome(&line.chars().nth(2).expect("Outcome not found"));

        let my_move = get_move_from_outcome(&enemy_move, &outcome);

        println!("Moves: {:?} {:?}", enemy_move, my_move);
        println!("Outcome: {:?}", outcome);

        current_score += my_move as u32 + outcome as u32;
    }

    current_score
}

fn get_my_move(input: char) -> Moves {
    match input {
        'X' => Moves::Rock,
        'Y' => Moves::Paper,
        'Z' => Moves::Scissors,
        other => panic!("Found invalid input ({other}) while parsing counter moves"),
    }
}

fn get_enemy_move(input: char) -> Moves {
    match input {
        'A' => Moves::Rock,
        'B' => Moves::Paper,
        'C' => Moves::Scissors,
        other => panic!("Found invalid input ({other}) while parsing enemy moves"),
    }
}

fn get_outcome(input: char) -> Outcomes {
    match input {
        'X' => Outcomes::Defeat,
        'Y' => Outcomes::Draw,
        'Z' => Outcomes::Victory,
        other => panic!("Found invalid input ({other}) while parsing the outcome"),
    }
}

fn get_move_from_outcome(enemy_move: &Moves, outcome: &Outcomes) -> Moves {
    match outcome {
        Outcomes::Draw => enemy_move.clone(),
        Outcomes::Victory => match enemy_move {
            Moves::Rock => Moves::Paper,
            Moves::Paper => Moves::Scissors,
            Moves::Scissors => Moves::Rock,
        },
        Outcomes::Defeat => match enemy_move {
            Moves::Rock => Moves::Scissors,
            Moves::Paper => Moves::Rock,
            Moves::Scissors => Moves::Paper,
        },
    }
}

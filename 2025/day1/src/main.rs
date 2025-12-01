use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn get_rotations(input: &str) -> Vec<i16> {
    input
        .lines()
        .map(|a| {
            let (direction, amount) = a.split_at(1);

            let mut amount = amount.parse().expect("Invalid rotation amount");

            // FIXME: Very ugly, figure out a better way to parse this :sob:
            if direction.starts_with('L') {
                amount *= -1;
            }

            amount
        })
        .collect()
}

fn part_one(input: &str) -> u32 {
    let mut status = 50;
    let mut result = 0;

    let rotations = get_rotations(input);

    for rotation in rotations {
        status += rotation;

        while status < 0 {
            status += 100;
        }
        while status >= 100 {
            status -= 100;
        }

        if status == 0 {
            result += 1;
        }
    }

    result
}

fn part_two(input: &str) -> u32 {
    let mut status = 50;
    let mut result = 0;

    let rotations = get_rotations(input);

    for rotation in rotations {
        let old = status;

        status += rotation;

        // Edge case: Later loop overcounts by one if you turn left immediately
        // after landing on zero; decrement by one to compensate
        if status < 0 && old == 0 {
            result -= 1;
        }

        while status < 0 {
            status += 100;
            result += 1;
        }

        if status == 0 {
            result += 1;
        }

        while status >= 100 {
            result += 1;
            status -= 100;
        }
    }

    result
}

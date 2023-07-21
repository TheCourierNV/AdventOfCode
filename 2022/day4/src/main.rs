use std::fs;

fn main() {
    println!("Welcome to Overlap-B-Gone");

    let input = fs::read_to_string("input.txt").expect("Cannot read input.txt");

    println!("{}", get_overlapping_shifts_count(input));
}

fn get_overlapping_shifts_count(input: String) -> u32 {
    let mut overlapping_shifts = 0;

    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }

        let shifts: Vec<&str> = line.split(",").collect();

        if is_overlapping(get_shift(shifts[0]), get_shift(shifts[1])) {
            overlapping_shifts += 1;
        }
    }

    overlapping_shifts
}

fn is_overlapping(first_shift: (u32, u32), second_shift: (u32, u32)) -> bool {
    let (first_shift_start, first_shift_end) = first_shift;
    let (second_shift_start, second_shift_end) = second_shift;

    let first_is_before_second =
        first_shift_start < second_shift_start && first_shift_end < second_shift_start;
    let fist_is_after_second = first_shift_start > second_shift_end;

    !(first_is_before_second || fist_is_after_second)
}

fn is_fully_overlapping(first_shift: (u32, u32), second_shift: (u32, u32)) -> bool {
    let (first_shift_start, first_shift_end) = first_shift;
    let (second_shift_start, second_shift_end) = second_shift;

    let first_contains_second =
        first_shift_start <= second_shift_start && first_shift_end >= second_shift_end;

    let second_contains_first =
        second_shift_start <= first_shift_start && second_shift_end >= first_shift_end;

    first_contains_second || second_contains_first
}

fn get_shift(raw_shift: &str) -> (u32, u32) {
    let extremities: Vec<&str> = raw_shift.split("-").collect();

    let start = extremities[0]
        .parse()
        .expect("Invalid input: cannot parse a number");
    let end = extremities[1]
        .parse()
        .expect("Invalid input: cannot parse a number");

    (start, end)
}

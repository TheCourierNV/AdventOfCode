use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file!");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn get_calibrations(input: &str) -> Vec<(u64, Vec<u64>)> {
    let mut calibrations = Vec::new();

    for line in input.lines() {
        let calibration_sections: Vec<&str> = line.splitn(2, ':').collect();

        let equation_result = calibration_sections[0]
            .parse()
            .expect("Invalid equation result!");

        let equation_numbers = calibration_sections[1]
            .split_whitespace()
            .map(|number| {
                number
                    .parse()
                    .expect("Invalid number in calibration equation!")
            })
            .collect();

        calibrations.push((equation_result, equation_numbers));
    }

    calibrations
}

fn part_one(input: &str) -> u64 {
    let mut answer = 0;

    let calibrations = get_calibrations(input);

    for (expected_equation_result, equation_numbers) in &calibrations {
        // Each bit keeps track of which operation is being used (1 = mul, 0 = sum)
        // I observed at most 12 numbers in a single equation, so u16 is more than enough
        let mut current_status: u16 = 0;

        while current_status < (1 << equation_numbers.len()) {
            let mut current_total: u64 = 0;

            for (idx, number) in equation_numbers.iter().enumerate() {
                if (current_status & 1 << idx) != 0 {
                    match current_total.checked_mul(*number) {
                        Some(total) => current_total = total,
                        None => break,
                    };
                } else {
                    current_total += number;
                }

                if current_total > *expected_equation_result {
                    break;
                }
            }

            if current_total == *expected_equation_result {
                answer += expected_equation_result;
                break;
            }

            current_status += 1;
        }
    }

    answer
}

fn part_two(input: &str) -> u64 {
    let mut answer = 0;

    let calibrations = get_calibrations(input);

    for (expected_equation_result, equation_numbers) in &calibrations {
        let mut current_status = 0;

        while current_status < 3_u64.pow(equation_numbers.len() as u32) {
            // Same idea as part one, but using a ternary number instead (which makes it a bit more
            // complicated) : 2 = concat, 1 = mul, 0 = sum
            let mut current_total: u64 = 0;

            for (idx, number) in equation_numbers.iter().enumerate() {
                let current_ternary_digit = current_status / 3_u64.pow(idx as u32) % 3;

                if current_ternary_digit == 2 {
                    // I could do this with a base 10 log, but fuck it
                    let number_digit_count = number.to_string().len() as u32;

                    current_total = (current_total * 10_u64.pow(number_digit_count)) + number;
                } else if current_ternary_digit == 1 {
                    match current_total.checked_mul(*number) {
                        Some(total) => current_total = total,
                        None => break,
                    };
                } else {
                    current_total += number;
                }

                if current_total > *expected_equation_result {
                    break;
                }
            }

            if current_total == *expected_equation_result {
                answer += expected_equation_result;
                break;
            }

            current_status += 1;
        }
    }

    answer
}

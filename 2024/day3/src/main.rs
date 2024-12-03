use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file!");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    let potential_muls: Vec<&str> = input.split("mul(").collect();
    let mut result = 0;

    for potential_mul in potential_muls {
        let mut parsed_number = 0;
        let mut other_number = 0;
        let mut found_comma = false;

        for c in potential_mul.chars() {
            if c.is_ascii_digit() {
                parsed_number = parsed_number * 10 + c.to_digit(10).unwrap();
                if parsed_number > 999 {
                    break;
                }
            } else if c == ',' {
                if found_comma {
                    break;
                } else {
                    found_comma = true;
                    other_number = parsed_number;
                    parsed_number = 0;
                }
            } else if c == ')' {
                if found_comma {
                    // This technically does not account for missing numbers, but that case will
                    // count as adding 0 anyway, so still the correct result
                    result += other_number * parsed_number;
                    break;
                }
            } else {
                break;
            }
        }
    }

    result
}

fn part_two(input: &str) -> u32 {
    let potential_muls: Vec<&str> = input.split("mul(").collect();
    let mut result = 0;

    let mut enabled = true;

    for potential_mul in potential_muls {
        let mut parsed_number = 0;
        let mut other_number = 0;
        let mut found_comma = false;

        if enabled {
            for c in potential_mul.chars() {
                if c.is_ascii_digit() {
                    parsed_number = parsed_number * 10 + c.to_digit(10).unwrap();
                    if parsed_number > 999 {
                        break;
                    }
                } else if c == ',' {
                    if found_comma {
                        break;
                    } else {
                        found_comma = true;
                        other_number = parsed_number;
                        parsed_number = 0;
                    }
                } else if c == ')' {
                    if found_comma {
                        // This technically does not account for missing numbers, but that case will
                        // count as adding 0 anyway, so still the correct result
                        println!("{result} += {parsed_number} * {other_number}");
                        result += other_number * parsed_number;
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        let last_do = potential_mul.rfind("do()");
        let last_dont = potential_mul.rfind("don't()");

        match (last_do, last_dont) {
            (Some(do_idx), Some(dont_idx)) => enabled = do_idx > dont_idx,
            (None, Some(_)) => enabled = false,
            (Some(_), None) => enabled = true,
            (None, None) => {}
        }
    }

    result
}

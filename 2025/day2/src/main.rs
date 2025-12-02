use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn get_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|raw_range| {
            let (start, end) = raw_range.split_once('-').expect("Invalid range");

            (
                start.parse().expect("Invalid range start"),
                end.parse().expect("Invalid range end"),
            )
        })
        .collect()
}

fn part_one(input: &str) -> u64 {
    let ranges = get_ranges(input);

    let mut result = 0;

    for (start, end) in ranges {
        for id in start..=end {
            // Yes, this is incorrect for exact powers of 10.
            // But I don't care, since they will still be identified as valid anyways
            let id_length = id.ilog10() + 1;

            let upper_digits = id / (u64::pow(10, id_length / 2));
            let lower_digits = id % (u64::pow(10, id_length / 2));

            if upper_digits == lower_digits {
                result += id;
            }
        }
    }

    result
}

fn part_two(input: &str) -> u64 {
    let ranges = get_ranges(input);
    let mut result = 0;

    for (start, end) in ranges {
        for id in start..=end {
            // Iterate over all the substrings of a given length, and see if they are all the same
            let id_string = id.to_string();
            let id_length = id_string.len();

            'substring_loop: for substring_length in 1..id_length {
                // Optimization: an invalid ID *must* evenly divide into equally sized substrings.
                // Therefore, if id_length is not a multiple of the substring_length, we already
                // know we shouldn't bother checking, but rather immediately retry with with a bigger
                // substring
                if id_length % substring_length == 0 {
                    // Get the first N digits at the start to act as a reference pattern
                    let pattern: Vec<char> = id_string.chars().take(substring_length).collect();

                    let mut chunk = Vec::new();

                    // (Very clumslily) Read an N sized chunk of the string, and check whether it
                    // matches the pattern. If not, we bail out, and try with a bigger substring on
                    // the next iteration.
                    //
                    // Otherwise, we keep reading another chunk and repeat. If we get to the
                    // end of the string without bailing out, we know the ID must be invalid; we
                    // break to avoid counting the same invalid ID multiple times (eg: 222222)
                    for digit in id_string.chars() {
                        chunk.push(digit);

                        if chunk.len() == substring_length {
                            if pattern != chunk {
                                continue 'substring_loop;
                            }

                            chunk.clear();
                        }
                    }

                    // If we get here, we know the ID was invalid!
                    result += id;
                    break;
                }
            }
        }
    }

    result
}

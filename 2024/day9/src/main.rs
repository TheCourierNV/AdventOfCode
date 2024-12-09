use std::fs;

fn main() {
    let input = fs::read_to_string("input2.txt").expect("Could not read input file!");

    println!("Part one: {}", part_one(&input));
    // println!("Part two: {}", part_two(&input));
}

fn parse_hard_drive(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut files = Vec::new();
    let mut spaces = Vec::new();

    for (i, c) in input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10).expect("Invalid block entry in hard drive!"))
        .enumerate()
    {
        if i % 2 == 0 {
            files.push(c);
        } else {
            spaces.push(c);
        }
    }

    (files, spaces)
}

fn part_one(input: &str) -> u64 {
    let (mut files, mut spaces) = parse_hard_drive(input);

    let mut defrag_drive = Vec::new();

    let mut front_file_idx = 0;
    let mut back_file_idx = files.len() - 1;

    let mut space_idx = 0;

    loop {
        while files[front_file_idx] > 0 {
            defrag_drive.push(front_file_idx);
            files[front_file_idx] -= 1;
        }

        while spaces[space_idx] > 0 {
            defrag_drive.push(back_file_idx);
            files[back_file_idx] -= 1;

            spaces[space_idx] -= 1;

            if files[back_file_idx] == 0 {
                back_file_idx -= 1;
            }
        }

        front_file_idx += 1;
        space_idx += 1;

        if front_file_idx >= back_file_idx || space_idx >= spaces.len() {
            while files[back_file_idx] != 0 {
                defrag_drive.push(back_file_idx);
                files[back_file_idx] -= 1;
            }

            break;
        }
    }

    defrag_drive
        .iter()
        .enumerate()
        .map(|(idx, file_id)| idx as u64 * *file_id as u64)
        .sum()
}


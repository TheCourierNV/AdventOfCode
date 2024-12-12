use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file!");

    println!("Part one: {}", part_one(&input));
    // println!("Part two: {}", part_two(&input));
}

fn get_map(input: &str) -> Vec<Vec<char>> {
    let mut map = Vec::new();

    for line in input.lines() {
        map.push(line.chars().collect());
    }

    map
}

fn flood_fill(
    map: &Vec<Vec<char>>,
    position: (usize, usize),
    already_fenced: &mut Vec<Vec<bool>>,
) -> (u64, u64) {
    let (x, y) = position;

    already_fenced[y][x] = true;

    let current_char = map[y][x];

    let mut area = 1;
    let mut perimeter = 0;

    let mut left_contribution = (0, 0);
    let mut right_contribution = (0, 0);
    let mut up_contribution = (0, 0);
    let mut down_contribution = (0, 0);

    if x != 0 && map[y][x - 1] == current_char {
        if !already_fenced[y][x - 1] {
            left_contribution = flood_fill(map, (x - 1, y), already_fenced);
        }
    } else {
        perimeter += 1;
    }

    if x < map[0].len() - 1 && map[y][x + 1] == current_char {
        if !already_fenced[y][x + 1] {
            right_contribution = flood_fill(map, (x + 1, y), already_fenced);
        }
    } else {
        perimeter += 1;
    }

    if y != 0 && map[y - 1][x] == current_char {
        if !already_fenced[y - 1][x] {
            up_contribution = flood_fill(map, (x, y - 1), already_fenced);
        }
    } else {
        perimeter += 1;
    }

    if y < map.len() - 1 && map[y + 1][x] == current_char {
        if !already_fenced[y + 1][x] {
            down_contribution = flood_fill(map, (x, y + 1), already_fenced);
        }
    } else {
        perimeter += 1;
    }

    area += left_contribution.0 + right_contribution.0 + up_contribution.0 + down_contribution.0;
    perimeter +=
        left_contribution.1 + right_contribution.1 + up_contribution.1 + down_contribution.1;

    (area, perimeter)
}

fn part_one(input: &str) -> u64 {
    let map = get_map(input);

    let mut regions = Vec::new();

    let mut already_fenced = vec![vec![false; map[0].len()]; map.len()];

    for (y, line) in map.iter().enumerate() {
        for x in 0..line.len() {
            if already_fenced[y][x] {
                continue;
            }

            regions.push(flood_fill(&map, (x, y), &mut already_fenced));
        }
    }

    regions
        .iter()
        .map(|(area, perimeter)| area * perimeter)
        .sum()
}

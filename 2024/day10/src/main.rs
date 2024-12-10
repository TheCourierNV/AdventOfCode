use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file!");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn read_map(input: &str) -> Vec<Vec<u32>> {
    let mut map = Vec::new();

    for line in input.lines() {
        map.push(
            line.chars()
                .map(|c| c.to_digit(10).expect("Invalid height in map"))
                .collect(),
        );
    }

    map
}

fn recursive_search(map: &Vec<Vec<u32>>, starting_position: (usize, usize)) -> u32 {
    recursive_search_step(
        map,
        &mut vec![vec![false; map[0].len()]; map.len()],
        starting_position,
        0,
    )
}

fn recursive_search_step(
    map: &Vec<Vec<u32>>,
    visited_nines: &mut Vec<Vec<bool>>,
    current_position: (usize, usize),
    current_height: u32,
) -> u32 {
    let (x, y) = current_position;

    let mut partial_result = 0;

    if current_height == 9 && !visited_nines[y][x] {
        visited_nines[y][x] = true;
        return 1;
    }

    if y != 0 && map[y - 1][x] == current_height + 1 {
        partial_result += recursive_search_step(map, visited_nines, (x, y - 1), current_height + 1);
    }

    if y < map.len() - 1 && map[y + 1][x] == current_height + 1 {
        partial_result += recursive_search_step(map, visited_nines, (x, y + 1), current_height + 1);
    }

    if x != 0 && map[y][x - 1] == current_height + 1 {
        partial_result += recursive_search_step(map, visited_nines, (x - 1, y), current_height + 1);
    }

    if x < map[y].len() - 1 && map[y][x + 1] == current_height + 1 {
        partial_result += recursive_search_step(map, visited_nines, (x + 1, y), current_height + 1);
    }

    partial_result
}

fn recursive_search_two(map: &Vec<Vec<u32>>, starting_position: (usize, usize)) -> u32 {
    recursive_search_step_two(map, starting_position, 0)
}

fn recursive_search_step_two(
    map: &Vec<Vec<u32>>,
    current_position: (usize, usize),
    current_height: u32,
) -> u32 {
    let (x, y) = current_position;

    let mut partial_result = 0;

    if current_height == 9 {
        return 1;
    }

    if y != 0 && map[y - 1][x] == current_height + 1 {
        partial_result += recursive_search_step_two(map, (x, y - 1), current_height + 1);
    }

    if y < map.len() - 1 && map[y + 1][x] == current_height + 1 {
        partial_result += recursive_search_step_two(map, (x, y + 1), current_height + 1);
    }

    if x != 0 && map[y][x - 1] == current_height + 1 {
        partial_result += recursive_search_step_two(map, (x - 1, y), current_height + 1);
    }

    if x < map[y].len() - 1 && map[y][x + 1] == current_height + 1 {
        partial_result += recursive_search_step_two(map, (x + 1, y), current_height + 1);
    }

    partial_result
}

fn part_one(input: &str) -> u32 {
    let map = read_map(input);

    let mut result = 0;

    for (y, line) in map.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height == 0 {
                result += recursive_search(&map, (x, y));
            }
        }
    }

    result
}

fn part_two(input: &str) -> u32 {
    let map = read_map(input);

    let mut result = 0;

    for (y, line) in map.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height == 0 {
                result += recursive_search_two(&map, (x, y));
            }
        }
    }

    result
}

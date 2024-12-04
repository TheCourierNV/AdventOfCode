use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file!");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    grid
}

fn part_one(input: &str) -> u32 {
    let mut result = 0;

    let grid = get_grid(input);

    // The starting letter is already taken care of
    let xmas = vec!['M', 'A', 'S'];

    for (y, line) in grid.iter().enumerate() {
        for (x, current_char) in line.iter().enumerate() {
            if *current_char == 'X' {
                // Every combination of offsets is a different direction
                for y_off in -1..=1 {
                    'outer: for x_off in -1..=1 {
                        let mut current_x: i32 = x as i32;
                        let mut current_y: i32 = y as i32;

                        for expected_xmas_char in &xmas {
                            current_x += x_off;
                            current_y += y_off;

                            // Bounds check
                            if current_y < 0
                                || current_y >= grid.len() as i32
                                || current_x < 0
                                || current_x >= line.len() as i32
                            {
                                continue 'outer;
                            }

                            if grid[current_y as usize][current_x as usize] != *expected_xmas_char {
                                continue 'outer;
                            }
                        }

                        //println!("Match at {x}; {y}; offsets {x_off} ; {y_off}");

                        result += 1;
                    }
                }
            }
        }
    }

    result
}

fn part_two(input: &str) -> u32 {
    let mut result = 0;

    let grid = get_grid(input);

    for (y, line) in grid.iter().enumerate() {
        for (x, current_char) in line.iter().enumerate() {
            if *current_char == 'A' {
                // Making sure it's impossible to go out of bounds
                if x == 0 || x == line.len() - 1 || y == 0 || y == grid.len() - 1 {
                    continue;
                }

                // Every possible case:
                if (grid[y - 1][x - 1] == 'M'
                    && grid[y - 1][x + 1] == 'M'
                    && grid[y + 1][x - 1] == 'S'
                    && grid[y + 1][x + 1] == 'S')
                    || (grid[y - 1][x - 1] == 'M'
                        && grid[y + 1][x - 1] == 'M'
                        && grid[y - 1][x + 1] == 'S'
                        && grid[y + 1][x + 1] == 'S')
                    || (grid[y + 1][x - 1] == 'M'
                        && grid[y + 1][x + 1] == 'M'
                        && grid[y - 1][x - 1] == 'S'
                        && grid[y - 1][x + 1] == 'S')
                    || (grid[y - 1][x + 1] == 'M'
                        && grid[y + 1][x + 1] == 'M'
                        && grid[y - 1][x - 1] == 'S'
                        && grid[y + 1][x - 1] == 'S')
                {
                    result += 1;
                }
            }
        }
    }

    result
}

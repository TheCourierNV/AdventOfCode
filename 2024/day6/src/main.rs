use std::fs;

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file!");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn get_map(input: &str) -> (Vec<Vec<bool>>, (usize, usize)) {
    let mut map = Vec::new();
    let mut starting_position = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut map_line = Vec::new();

        for (x, char) in line.chars().enumerate() {
            if char == '^' {
                starting_position = (x, y);
            }

            if char == '#' {
                map_line.push(true);
            } else {
                map_line.push(false);
            }
        }

        map.push(map_line);
    }

    (map, starting_position)
}

fn part_one(input: &str) -> u32 {
    let (map, (mut guard_x, mut guard_y)) = get_map(input);

    let map_size_x = map[0].len();
    let map_size_y = map.len();

    let mut visited_cells = vec![vec![false; map_size_y]; map_size_x];

    let mut current_direction = Direction::Up;

    loop {
        visited_cells[guard_y][guard_x] = true;

        let (next_x, next_y) = match &current_direction {
            Direction::Up => (guard_x as i32, guard_y as i32 - 1),
            Direction::Down => (guard_x as i32, guard_y as i32 + 1),
            Direction::Left => (guard_x as i32 - 1, guard_y as i32),
            Direction::Right => (guard_x as i32 + 1, guard_y as i32),
        };

        if next_y < 0 || next_x < 0 {
            break;
        }

        let next_x = next_x as usize;
        let next_y = next_y as usize;

        if next_y >= map_size_y || next_x >= map_size_x {
            break;
        }

        if map[next_y][next_x] {
            current_direction = match current_direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            }
        } else {
            guard_x = next_x;
            guard_y = next_y;
        }
    }

    visited_cells
        .iter()
        .flatten()
        .filter(|visited| **visited)
        .count() as u32
}

fn is_infinite_loop(
    map: &Vec<Vec<bool>>,
    starting_position: (usize, usize),
) -> (bool, Vec<Vec<Vec<Direction>>>) {
    let map_size_x = map[0].len();
    let map_size_y = map.len();

    let (mut guard_x, mut guard_y) = starting_position;

    let mut visited_cells = vec![vec![vec![]; map_size_y]; map_size_x];

    let mut current_direction = Direction::Up;

    loop {
        if visited_cells[guard_y][guard_x].contains(&current_direction) {
            return (true, visited_cells);
        }

        visited_cells[guard_y][guard_x].push(current_direction.clone());

        let (next_x, next_y) = match &current_direction {
            Direction::Up => (guard_x as i32, guard_y as i32 - 1),
            Direction::Down => (guard_x as i32, guard_y as i32 + 1),
            Direction::Left => (guard_x as i32 - 1, guard_y as i32),
            Direction::Right => (guard_x as i32 + 1, guard_y as i32),
        };

        if next_y < 0 || next_x < 0 {
            return (false, Vec::new());
        }

        let next_x = next_x as usize;
        let next_y = next_y as usize;

        if next_y >= map_size_y || next_x >= map_size_x {
            return (false, Vec::new());
        }

        if map[next_y][next_x] {
            current_direction = match current_direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            }
        } else {
            guard_x = next_x;
            guard_y = next_y;
        }
    }
}

fn debug_print(
    new_map: &Vec<Vec<bool>>,
    starting_position: (usize, usize),
    new_obstacle_position: (usize, usize),
    path: &Vec<Vec<Vec<Direction>>>,
) {
    let (x, y) = new_obstacle_position;

    for (py, line) in new_map.iter().enumerate() {
        for (px, cell) in line.iter().enumerate() {
            if x == px && y == py {
                print!("\x1b[1;31mX\x1b[0m");
            } else if px == starting_position.0 && py == starting_position.1 {
                print!("^");
            } else {
                let to_print = if (path[py][px].contains(&Direction::Left)
                    || path[py][px].contains(&Direction::Right))
                    && (path[py][px].contains(&Direction::Up)
                        || path[py][px].contains(&Direction::Down))
                {
                    '+'
                } else if path[py][px].contains(&Direction::Left)
                    || path[py][px].contains(&Direction::Right)
                {
                    '-'
                } else if path[py][px].contains(&Direction::Up)
                    || path[py][px].contains(&Direction::Down)
                {
                    '|'
                } else {
                    if *cell {
                        'X'
                    } else {
                        ' '
                    }
                };

                print!("{}", to_print);
            }
        }

        println!();
    }
}

fn part_two(input: &str) -> u32 {
    let mut result = 0;

    let (map, starting_position) = get_map(input);

    for (y, line) in map.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            if x == starting_position.0 && y == starting_position.1 {
                continue;
            }

            let mut new_map = map.clone();

            new_map[y][x] = true;

            let (loops, path) = is_infinite_loop(&new_map, starting_position);

            if loops {
                println!("FOUND! {x} ; {y}");

                //debug_print(&new_map, starting_position, (x, y), &path);

                result += 1;
            }
        }
    }

    result
}

/*
fn part_two(input: &str) -> u32 {
    let mut result = 0;

    let (map, (mut guard_x, mut guard_y)) = get_map(input);

    let map_size_x = map[0].len();
    let map_size_y = map.len();

    let mut visited_cells = vec![vec![(false, vec![]); map_size_y]; map_size_x];

    let mut current_direction = Direction::Up;

    // To workaround starting position edge case
    let mut speculative_guard_y = guard_y;
    loop {
        visited_cells[speculative_guard_y][guard_x]
            .1
            .push(Direction::Up);

        if speculative_guard_y + 1 >= map_size_y || map[speculative_guard_y + 1][guard_x] {
            break;
        }

        speculative_guard_y += 1;
    }

    loop {
        visited_cells[guard_y][guard_x]
            .1
            .push(current_direction.clone());

        visited_cells[guard_y][guard_x].0 = true;

        let (next_x, next_y) = match &current_direction {
            Direction::Up => (guard_x as i32, guard_y as i32 - 1),
            Direction::Down => (guard_x as i32, guard_y as i32 + 1),
            Direction::Left => (guard_x as i32 - 1, guard_y as i32),
            Direction::Right => (guard_x as i32 + 1, guard_y as i32),
        };

        if next_y < 0 || next_x < 0 {
            break;
        }

        let next_x = next_x as usize;
        let next_y = next_y as usize;

        if next_y >= map_size_y || next_x >= map_size_x {
            break;
        }

        let next_direction = match current_direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };

        if visited_cells[guard_y][guard_x].1.contains(&next_direction)
            && !visited_cells[next_y][next_x].0
        {
            println!("Potential obstacle: {} ; {}", next_x, next_y);
            result += 1;
        }

        if map[next_y][next_x] {
            current_direction = next_direction;

            let mut speculative_guard_x = guard_x as i32;
            let mut speculative_guard_y = guard_y as i32;

            loop {
                (speculative_guard_x, speculative_guard_y) = match current_direction {
                    Direction::Up => (speculative_guard_x, speculative_guard_y + 1),
                    Direction::Down => (speculative_guard_x, speculative_guard_y - 1),
                    Direction::Left => (speculative_guard_x + 1, speculative_guard_y),
                    Direction::Right => (speculative_guard_x - 1, speculative_guard_y),
                };

                if speculative_guard_x < 0 || speculative_guard_y < 0 {
                    break;
                }

                let speculative_guard_x = speculative_guard_x as usize;
                let speculative_guard_y = speculative_guard_y as usize;

                if speculative_guard_x >= map_size_x
                    || speculative_guard_y >= map_size_y
                    || map[speculative_guard_y][speculative_guard_x]
                {
                    break;
                }

                visited_cells[speculative_guard_y][speculative_guard_x]
                    .1
                    .push(current_direction.clone())
            }
        } else {
            guard_x = next_x;
            guard_y = next_y;
        }
    }

    for line in visited_cells {
        for cell in line {
            let to_print = if cell.0 {
                if (cell.1.contains(&Direction::Left) || cell.1.contains(&Direction::Right))
                    && (cell.1.contains(&Direction::Up) || cell.1.contains(&Direction::Down))
                {
                    '+'
                } else if cell.1.contains(&Direction::Left) || cell.1.contains(&Direction::Right) {
                    '-'
                } else if cell.1.contains(&Direction::Up) || cell.1.contains(&Direction::Down) {
                    '|'
                } else {
                    '.'
                }
            } else {
                if cell.1.is_empty() {
                    ' '
                } else {
                    if (cell.1.contains(&Direction::Left) || cell.1.contains(&Direction::Right))
                        && (cell.1.contains(&Direction::Up) || cell.1.contains(&Direction::Down))
                    {
                        '+'
                    } else if cell.1.contains(&Direction::Left)
                        || cell.1.contains(&Direction::Right)
                    {
                        '_'
                    } else if cell.1.contains(&Direction::Up) || cell.1.contains(&Direction::Down) {
                        '!'
                    } else {
                        '.'
                    }
                }
            };

            print!("{to_print}");
        }

        println!();
    }

    // Cannot do it afterwards; this overcounts (includes cases where it would place an obstacle
    // blocking the path used to get there in the first place)
    //
    /*
    visited_cells
        .iter()
        .flatten()
        .filter(|visited_directions| {})
        .count();
    */

    result
}
*/

use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file!");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn get_antennas(input: &str) -> (HashMap<char, Vec<(u32, u32)>>, (u32, u32)) {
    let mut map: HashMap<char, Vec<(u32, u32)>> = HashMap::new();

    let lines: Vec<&str> = input.lines().collect();
    let mut size_x = 0;

    for (y, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        for (x, symbol) in chars.iter().enumerate() {
            if *symbol != '.' {
                map.entry(*symbol).or_default().push((x as u32, y as u32));
            }

            size_x = chars.len() as u32;
        }
    }

    (map, (size_x, lines.len() as u32))
}

fn part_one(input: &str) -> u32 {
    let (map, (size_x, size_y)) = get_antennas(input);

    let mut antinode_map = vec![vec![false; size_x as usize]; size_y as usize];

    for antennas in map.values() {
        for antenna in antennas {
            for antenna2 in antennas {
                let distance_x: i32 = antenna.0 as i32 - antenna2.0 as i32;
                let distance_y: i32 = antenna.1 as i32 - antenna2.1 as i32;

                if distance_x == 0 && distance_y == 0 {
                    continue;
                }

                let antinode_x = antenna.0 as i32 + distance_x;
                let antinode_y = antenna.1 as i32 + distance_y;

                if antinode_x < 0
                    || antinode_x as u32 >= size_x
                    || antinode_y < 0
                    || antinode_y as u32 >= size_y
                {
                    continue;
                }

                antinode_map[antinode_y as usize][antinode_x as usize] = true;
            }
        }
    }

    antinode_map.iter().flatten().filter(|a| **a).count() as u32
}

fn part_two(input: &str) -> u32 {
    let (map, (size_x, size_y)) = get_antennas(input);

    let mut antinode_map = vec![vec![false; size_x as usize]; size_y as usize];

    for antennas in map.values() {
        for antenna in antennas {
            for antenna2 in antennas {
                let distance_x: i32 = antenna.0 as i32 - antenna2.0 as i32;
                let distance_y: i32 = antenna.1 as i32 - antenna2.1 as i32;

                if distance_x == 0 && distance_y == 0 {
                    continue;
                }

                let mut antinode_x = antenna.0 as i32;
                let mut antinode_y = antenna.1 as i32;

                loop {
                    if antinode_x < 0
                        || antinode_x as u32 >= size_x
                        || antinode_y < 0
                        || antinode_y as u32 >= size_y
                    {
                        break;
                    }

                    antinode_map[antinode_y as usize][antinode_x as usize] = true;

                    antinode_x += distance_x;
                    antinode_y += distance_y;
                }
            }
        }
    }

    antinode_map.iter().flatten().filter(|a| **a).count() as u32
}

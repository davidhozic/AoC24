use std::{collections::HashSet, fs::read_to_string};


#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}


pub fn part_one() {
    let map = parse_input();
    let (visited, _) = visit_map(&map);
    println!("{}", visited.len());
}


pub fn part_two() {
    let map = parse_input();
    let mut map_copy;
    let (mut visited, _) = visit_map(&map);
    let mut is_loop;
    visited.remove(&get_start_position(&map));

    let mut cnt: usize = 0;
    for (row, column) in visited {
        map_copy = map.clone();
        map_copy[row as usize][column as usize] = '#';  // Set a visited position as the obstacle
        (_, is_loop) = visit_map(&map_copy);
        // Test if adding an obstacle created a loop.
        if is_loop {
            cnt += 1;
        };  
    }

    println!("{cnt}");
}


pub fn parse_input() -> Vec<Vec<char>> {
    let input = read_to_string("./inputs/day6.txt").unwrap();
    let lines = input.lines().map(|x| x.chars().collect()).collect();
    lines
}


fn visit_map(map: &Vec<Vec<char>>) -> (HashSet<(isize, isize)>, bool) {
    let mut direction = Direction::UP;

    // Find the starting position
    let (mut row_index, mut column_index) = get_start_position(&map);

    let mut next_row_index;
    let mut next_column_index;

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut visited_with_dir = HashSet::new();
    let mut position;
    let mut pose;
    let column_len = map[0].len();
    let row_len: usize = map.len();

    visited.insert((row_index, column_index));
    visited_with_dir.insert((row_index, column_index, direction));
    let mut is_loop = false;

    // Loop until out of the map
    loop {
        match direction {
            Direction::UP => {
                next_row_index = row_index - 1;
                if next_row_index < 0 {
                    break;
                }

                // Reached an obstacle
                if map[next_row_index as usize][column_index as usize] == '#' {
                    direction = Direction::RIGHT;
                    continue;
                }

                row_index = next_row_index;
            },
            Direction::RIGHT => {
                next_column_index = column_index + 1;
                if next_column_index as usize == column_len {
                    break;
                }

                // Reached an obstacle
                if map[row_index as usize][next_column_index as usize] == '#' {
                    direction = Direction::DOWN;
                    continue;
                }

                column_index = next_column_index;
            },
            Direction::DOWN => {
                next_row_index = row_index + 1;
                if next_row_index as usize == row_len {
                    break;
                }

                // Reached an obstacle
                if map[next_row_index as usize][column_index as usize] == '#' {
                    direction = Direction::LEFT;
                    continue;
                }

                row_index = next_row_index;
            },
            Direction::LEFT => {
                next_column_index = column_index - 1;
                if next_column_index < 0 {
                    break;
                }

                // Reached an obstacle
                if map[row_index as usize][next_column_index as usize] == '#' {
                    direction = Direction::UP;
                    continue;
                }

                column_index = next_column_index;
            },
        }

        position = (row_index, column_index);
        pose = (row_index, column_index, direction);
        visited.insert(position);

        if !visited_with_dir.contains(&pose) {
            visited_with_dir.insert(pose);
        }
        else {
            is_loop = true;
            break;  // Same position with direction reached => clearly a loop
        }
    }
    (visited, is_loop)
}


fn get_start_position(map: &Vec<Vec<char>>) -> (isize, isize) {
    let mut row_index = None;
    let mut column_index = None;
    for (i, line) in map.iter().enumerate() {
        if let Some(x) =  line.iter().position(|x| *x == '^') {
            column_index = Some(x);
            row_index = Some(i);
            break;
        }
    }
    (row_index.unwrap() as isize, column_index.unwrap() as isize)
}

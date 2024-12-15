use std::fs::read_to_string;
use crate::utils::print_map;



pub fn part_one() {
    let (mut map, movements, mut y, mut x) = parse_input();
    'm: for movement in movements {
        // print_map(&map);
        // println!("----");
        match movement {
            '<' => {
                // Line up to the current (x) position
                let line_slice = &map[y][..x];
                // First wall occurrence in direction of movement
                let max_i = line_slice.len() - line_slice.iter().rev().position(|c| *c == '#' || *c == '.').unwrap() - 1;

                let line = &mut map[y];
                
                if line[max_i] == '#' {
                    continue 'm;
                }

                // Copy from left to right
                for i in max_i..x {
                    line[i] = line[i + 1];
                }

                line[x] = '.';
                x -= 1;
            },
            '^' => {
                let column: Vec<_> = map.iter().map(|line| line[x]).collect();
                let column_slice: Vec<_> = map[..y].iter().map(|line| line[x]).collect();
                // First wall occurrence in direction of movement
                let max_i = column_slice.len() - column_slice.iter().rev().position(|c| *c == '#' || *c == '.').unwrap() - 1;
                
                if column[max_i] == '#' {
                    continue 'm;
                }

                // Copy from left to right
                for i in max_i..y {
                    map[i][x] = map[i + 1][x];
                }

                map[y][x] = '.';
                y -= 1;
            },
            '>' => {
                // Line up to the current (x) position
                let line_slice = &map[y][x+1..];
                // First wall occurrence in direction of movement
                let max_i = line_slice.iter().position(|c| *c == '#' || * c == '.').unwrap() + x + 1;

                let line = &mut map[y];

                if line[max_i] == '#' {
                    continue 'm;
                }

                // Copy from left to right
                for i in (x+1..max_i+1).rev() {
                    line[i] = line[i - 1];
                }

                line[x] = '.';
                x += 1;                
            },
            'v' => {
                let column: Vec<_> = map.iter().map(|line| line[x]).collect();
                let column_slice: Vec<_> = map[y+1..].iter().map(|line| line[x]).collect();
                // First wall occurrence in direction of movement
                let max_i = column_slice.iter().position(|c| *c == '#' || *c == '.').unwrap() + y + 1;
                
                if column[max_i] == '#' {
                    continue 'm;
                }

                // Copy from left to right
                for i in (y+1..max_i+1).rev() {
                    map[i][x] = map[i - 1][x];
                }

                map[y][x] = '.';
                y += 1;
            },
            _ => unreachable!()
        };
    }

    // print_map(&map);
    // println!("----");

    // Calculate sum of Goods Positioning System coorinates
    let sum = map.iter().enumerate().map(|(y, line)|{
        line.iter().enumerate().map(move |(x, c)| (*c == 'O') as usize * (100 * y + x)).sum::<usize>()
    }).sum::<usize>();
    println!("{sum}");
}


pub fn part_two() {
    fn move_updown(map: &mut Vec<Vec<char>>, direction: isize, y: usize, x: usize, modify: bool) -> bool {
        let new_y = (y as isize + direction) as usize;
        let can_move = match map[new_y][x] {
            '.' => {
                if modify {
                    map[new_y][x] = map[y][x];
                    map[y][x] = '.';
                }
                true
            },
            '[' => {
                let can_move = move_updown(map, direction, new_y, x + 1, false) && move_updown(map, direction, new_y, x, false);
                if modify && can_move {
                    move_updown(map, direction, new_y, x + 1, true);
                    move_updown(map, direction, new_y, x, true);
                    map[new_y][x] = map[y][x];
                    map[y][x] = '.';
                }
                can_move
            },
            ']' => {
                let can_move = move_updown(map, direction, new_y, x -1 , false) && move_updown(map, direction, new_y, x, false);
                if modify && can_move {
                    move_updown(map, direction, new_y, x - 1, true);
                    move_updown(map, direction, new_y, x, true);
                    map[new_y][x] = map[y][x];
                    map[y][x] = '.';
                }
                can_move
            }
            '#' => {
                false
            }
            _ => unreachable!(),
        };

        can_move
    }

    let (mut map, movements, mut y, mut x) = parse_input_2();
    'm: for movement in movements {
        // print_map(&map);
        // println!("----");
        match movement {
            '<' => {
                // Line up to the current (x) position
                let line_slice = &map[y][..x];
                // First wall occurrence in direction of movement
                let max_i = line_slice.len() - line_slice.iter().rev().position(|c| *c == '#' || *c == '.').unwrap() - 1;

                let line = &mut map[y];
                
                if line[max_i] == '#' {
                    continue 'm;
                }

                // Copy from left to right
                for i in max_i..x {
                    line[i] = line[i + 1];
                }

                line[x] = '.';
                x -= 1;
            },
            '^' => {
                if move_updown(&mut map, -1, y, x, true) {
                    y -= 1;
                }
            },
            '>' => {
                // Line up to the current (x) position
                let line_slice = &map[y][x+1..];
                // First wall occurrence in direction of movement
                let max_i = line_slice.iter().position(|c| *c == '#' || * c == '.').unwrap() + x + 1;

                let line = &mut map[y];

                if line[max_i] == '#' {
                    continue 'm;
                }

                // Copy from left to right
                for i in (x+1..max_i+1).rev() {
                    line[i] = line[i - 1];
                }

                line[x] = '.';
                x += 1;                
            },
            'v' => {
                if move_updown(&mut map, 1, y, x, true) {
                    y += 1;
                }
            },
            _ => unreachable!()
        };
    }

    // print_map(&map);
    // println!("----");

    // Calculate sum of Goods Positioning System coorinates
    let sum = map.iter().enumerate().map(|(y, line)|{
        line.iter().enumerate().map(move |(x, c)| (*c == '[') as usize * (100 * y + x)).sum::<usize>()
    }).sum::<usize>();
    println!("{sum}");
}


pub fn parse_input() -> (Vec<Vec<char>>, Vec<char>, usize, usize) {
    let input = read_to_string("./inputs/day15.txt").unwrap();
    let (map, movements) = input.split_once("\n\n").unwrap();
    let map: Vec<Vec<_>> = map.lines().map(|x| x.chars().collect()).collect();
    let (mut y, mut x) = (0, 0);
    for (r, line) in map.iter().enumerate() {
        for (c, character) in line.iter().enumerate() {
            if *character == '@' {
                x = c;
                y = r;
                break;
            }
        }
    }

    (
        map,
        movements.replace("\n", "").chars().collect(),
        y,
        x
    )        
}

pub fn parse_input_2() -> (Vec<Vec<char>>, Vec<char>, usize, usize) {
    let input = read_to_string("./inputs/day15.txt").unwrap();
    let (map, movements) = input.split_once("\n\n").unwrap();
    let map = map.replace("#", "##").replace("O", "[]").replace(".", "..").replace("@", "@.");
    let map: Vec<Vec<_>> = map.lines().map(|x| x.chars().collect()).collect();
    let (mut y, mut x) = (0, 0);
    for (r, line) in map.iter().enumerate() {
        for (c, character) in line.iter().enumerate() {
            if *character == '@' {
                x = c;
                y = r;
                break;
            }
        }
    }

    (
        map,
        movements.replace("\n", "").chars().collect(),
        y,
        x
    )        
}


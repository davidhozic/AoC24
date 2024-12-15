use std::fs::read_to_string;
use crate::utils::print_map;



pub fn part_one() {
    let (mut map, movements, mut y, mut x) = parse_input();
    'm: for movement in movements {
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
                print_map(&map);
                println!("----");
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
                print_map(&map);
                println!("----");
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
                print_map(&map);
                println!("----");
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
                print_map(&map);
                println!("----");
            },
            _ => unreachable!()
        };
    }
}


pub fn part_two() {

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
        movements.chars().collect(),
        y,
        x
    )        
}

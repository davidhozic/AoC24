use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

use crate::utils::print_map;


pub fn part_one() {
    let map = parse_input();
    let mut antennas = HashMap::new();
    let mut coordinates;
    let map_limits = (map.len() as isize, map[0].len() as isize);
    let mut map_mod = map.clone();


    // Create a map of all the antennas coordinates
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if !c.is_alphanumeric() {  // Ignore empty spots
                continue;
            }
            coordinates = antennas.entry(c).or_insert(Vec::new());
            coordinates.push((y, x));
        }
    }

    let mut antinode_coords = HashSet::new();
    for (_, coords) in &antennas {
        // Iterate over all antenna pairs
        for (coord_i, (y1, x1)) in coords.iter().enumerate() {
            // Start at the (p1's index) + 1 because we only want unique pairs (regardless of orientation)
            for (y2, x2) in &coords[coord_i+1..] {
                let dy = *y2 as isize - *y1 as isize;
                let dx = *x2 as isize - *x1 as isize;
                let norm = ((dy.pow(2) + dx.pow(2)) as f64).sqrt().ceil() as usize;
                if norm >= 2 {
                    let an1 = (*y1 as isize - dy, *x1 as isize - dx);
                    let an2 = (*y2 as isize + dy, *x2 as isize + dx);
                    if  an1.1 >= 0 && an1.0 >= 0 && an1.1 < map_limits.1 && an1.0 < map_limits.0 {
                        antinode_coords.insert(an1);
                        let (y, x) = an1;
                        map_mod[y as usize][x as usize] = '#'
                    }

                    if an2.1 >= 0 && an2.0 >= 0 && an2.1 < map_limits.1 && an2.0 < map_limits.0 {
                        antinode_coords.insert(an2);
                        let (y, x) = an2;
                        map_mod[y as usize][x as usize] = '#'
                    }
                }
            }
        }
    }
    print_map(&map_mod);
    println!("{}", antinode_coords.len());
}


pub fn part_two() {

}


pub fn parse_input() -> Vec<Vec<char>> {
    let input = read_to_string("./inputs/day8.txt").unwrap();
    input.lines().map(|line| line.chars().collect()).collect()
}


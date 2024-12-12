use std::{collections::HashSet, fs::read_to_string};
use crate::utils::{print_map, benchmark};


const DIR: &[isize] = &[-1, 0, 1];

fn find_region(y_start: usize, x_start: usize, map: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let search_char = map[y_start][x_start];
    let mut points = HashSet::new();
    points.insert((y_start, x_start));
    let mut inserted = true;
    while inserted {
        inserted = false;

        for (y, x) in points.clone() {
            for y_diff in DIR {
                // Outside of map
                if *y_diff == -1 && y == 0 || *y_diff == 1 && y == map.len() - 1 {
                    continue;
                }
                
                let new_y = (y as isize + *y_diff) as usize;
                for x_diff in DIR {
                    // Outside of map
                    if *x_diff == -1 && x == 0 || *x_diff == 1 && x == map[0].len() - 1 {
                        continue;
                    }
                    let new_x = (x as isize + *x_diff) as usize;
                    if map[new_y][new_x] == search_char && !points.contains(&(new_y, new_x)){
                        points.insert((new_y, new_x));
                        inserted = true;
                    }
                }
            }
        }
    }

    points
}


fn find_perimeter(region: &HashSet<(usize, usize)>) -> usize {
    let mut perimeter = 0;
    let region: HashSet<_> = region.iter().map(|(l, r)| (*l as isize, *r as isize)).collect();

    for (y, x) in &region {
        for y_diff in DIR.iter().map(|x| *x as isize) {
            for x_diff in DIR.iter().map(|x| *x as isize) {
                perimeter += region.contains(&(y + y_diff, x + x_diff)) as usize;
            }    
        }
    }
    perimeter
}


pub fn part_one() {
    let map: Vec<Vec<char>> = parse_input();
    let mut regions = Vec::new();
    let mut visited = HashSet::new();

    let mut region;

    // Iterate through the entire map and run region exploration from each index outward
    // in all directions.
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            // Skip the positions that were already explored. We don't want duplicated regions.
            if visited.contains(&(y, x)) {
                continue;
            }

            region = find_region(y, x, &map);
            visited = visited.union(&region).map(|x| *x).collect();
            regions.push(region);
        }
    }
    println!("{:?}", regions[0]);
    println!("{}", find_perimeter(&regions[0]) )
}


pub fn part_two() {

}


pub fn parse_input() -> Vec<Vec<char>> {
    read_to_string("./inputs/day12.txt").unwrap()
        .lines().map(|map| map.chars().collect()).collect()
}

use std::{collections::{HashMap, HashSet}, fs::read_to_string};
use crate::utils::print_map;


pub fn part_one() {
    const DIRECTIONNS: [isize; 3] = [-1, 0, 1];

    let map: Vec<Vec<u32>> = parse_input();
    // print_map(&map);

    // Initiate map of trailheads
    let mut trailheads: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
    for (row_i, row) in map.iter().enumerate() {
        for (column_i, element) in row.iter().enumerate() {
            if *element == 0 {
                trailheads.insert((row_i as isize, column_i as isize), vec![(row_i as isize, column_i as isize)]);
            }
        }
    }


    let mut trailheads_ends: HashMap<(isize, isize), HashSet<(isize, isize)>> = HashMap::with_capacity(trailheads.len());
    let mut trailheads_to_remove = Vec::with_capacity(trailheads.len());

    // While any path that started from any trailhead exists
    while trailheads.len() > 0 {
        for (trailhead, paths) in trailheads.iter_mut() {
            // Paths for each trail head
            let mut new_paths = Vec::new();

            // Find new paths by exploring up, down, left, and right. Start from the already explored paths (their last known point)
            for (y, x) in paths.iter() {
                // Find neighbords with gradient 1
                for dy in DIRECTIONNS {
                    for dx in DIRECTIONNS {
                        let (ny, nx) = (*y + dy, *x + dx);
                        if dx.abs() == dy.abs()  // Diagonal directions aren't allowed
                            || ny < 0 || nx < 0 || ny as usize == map.len() || nx as usize == map[0].len() {  // Outside of map
                            continue;
                        }

                        let new_value = map[ny as usize][nx as usize];
                        // We found a path that doesn't lead to the end
                        if new_value != map[*y as usize][*x as usize] + 1 {
                            continue;
                        }

                        // We reached the end of the current path
                        if new_value == 9 {
                            let set = trailheads_ends.entry(*trailhead).or_insert(HashSet::new());
                            set.insert((ny, nx));
                        }

                        // Found point either on the current path or a new path
                        new_paths.push((ny, nx));
                    }
                }
            }

            // No new direction was found from any existing path => Stop the search for this trailhead
            if new_paths.len() == 0 {
                trailheads_to_remove.push(*trailhead);
            }

            // Update the newly discovered paths/directions to the new 
            paths.clear();
            paths.append(&mut new_paths);
        }

        // Cleanup trailheads that reached the end
        for trailhead in &trailheads_to_remove {
            trailheads.remove(trailhead).unwrap();
        }
    }

    // println!("{:?}", trailheads_ends);
    let score: usize = trailheads_ends.values().map(|x| x.len()).sum();
    println!("Sum of all trailhead scores is {score}");
}


pub fn part_two() {
    const DIRECTIONNS: [isize; 3] = [-1, 0, 1];

    let map: Vec<Vec<u32>> = parse_input();
    // print_map(&map);

    // Initiate map of trailheads
    let mut trailheads: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
    for (row_i, row) in map.iter().enumerate() {
        for (column_i, element) in row.iter().enumerate() {
            if *element == 0 {
                trailheads.insert((row_i as isize, column_i as isize), vec![(row_i as isize, column_i as isize)]);
            }
        }
    }


    let mut trailheads_ends: HashMap<(isize, isize), usize> = HashMap::with_capacity(trailheads.len());
    let mut trailheads_to_remove = Vec::with_capacity(trailheads.len());

    // While any path that started from any trailhead exists
    while trailheads.len() > 0 {
        for (trailhead, paths) in trailheads.iter_mut() {
            // Paths for each trail head
            let mut new_paths = Vec::new();

            // Find new paths by exploring up, down, left, and right. Start from the already explored paths (their last known point)
            for (y, x) in paths.iter() {
                // Find neighbords with gradient 1
                for dy in DIRECTIONNS {
                    for dx in DIRECTIONNS {
                        let (ny, nx) = (*y + dy, *x + dx);
                        if dx.abs() == dy.abs()  // Diagonal directions aren't allowed
                            || ny < 0 || nx < 0 || ny as usize == map.len() || nx as usize == map[0].len() {  // Outside of map
                            continue;
                        }

                        let new_value = map[ny as usize][nx as usize];
                        // We found a path that doesn't lead to the end
                        if new_value != map[*y as usize][*x as usize] + 1 {
                            continue;
                        }

                        // We reached the end of the current path
                        if new_value == 9 {
                            trailheads_ends.insert(*trailhead, trailheads_ends.get(trailhead).unwrap_or(&0) + 1);
                        }

                        // Found point either on the current path or a new path
                        new_paths.push((ny, nx));
                    }
                }
            }

            // No new direction was found from any existing path => Stop the search for this trailhead
            if new_paths.len() == 0 {
                trailheads_to_remove.push(*trailhead);
            }

            // Update the newly discovered paths/directions to the new 
            paths.clear();
            paths.append(&mut new_paths);
        }

        // Cleanup trailheads that reached the end
        for trailhead in &trailheads_to_remove {
            trailheads.remove(trailhead).unwrap();
        }
    }

    // println!("{:?}", trailheads_ends);
    let score: usize = trailheads_ends.values().sum();
    println!("Sum of all trailhead ratings is {score}");
}


pub fn parse_input() -> Vec<Vec<u32>> {
    read_to_string("./inputs/day10.txt").unwrap().lines().map(|x| x.chars().map(|c| c.to_digit(10).unwrap_or(11)).collect()).collect()
}

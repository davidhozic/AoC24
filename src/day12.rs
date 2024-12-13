use std::{collections::HashSet, fs::read_to_string};


fn find_region(y_start: usize, x_start: usize, map: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let search_char = map[y_start][x_start];
    let mut points = HashSet::new();
    points.insert((y_start, x_start));
    let mut inserted = true;
    while inserted {
        inserted = false;

        for (y, x) in points.clone() {
            for &(y_diff, x_diff) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                // Outside of map                
                let new_y = y as isize + y_diff;
                let new_x = x as isize + x_diff;
                if new_y < 0 || new_y as usize >= map.len() || new_x < 0 || new_x as usize >= map[0].len() {
                    continue;
                }

                if map[new_y as usize][new_x as usize] == search_char && !points.contains(&(new_y as usize, new_x as usize)){
                    points.insert((new_y as usize, new_x as usize));
                    inserted = true;
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
        for &(y_diff, x_diff) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            perimeter += (!region.contains(&(y + y_diff, x + x_diff))) as usize;
        }
    }
    perimeter
}

fn find_sides(region: &HashSet<(usize, usize)>) -> usize {
    let mut perimeter: usize = 0;
    let region: HashSet<_> = region.iter().map(|(l, r)| (*l as isize, *r as isize)).collect();
    let mut fence = HashSet::new();

    for (y, x) in &region {
        for &(y_diff, x_diff) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let fence_part = (y + y_diff, x + x_diff);
            if !region.contains(&fence_part) {
                fence.insert(fence_part);
            }
        }
    }

    for (y, x) in &fence {
        let left = (*y, x - 1);
        let right = (*y, x + 1);

        if fence.contains(&left) && fence.contains(&right) {
            continue;
        }

        let up = (y - 1, *x);
        let down = (y + 1, *x);


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

    println!("{}", regions.into_iter().map(|region| region.len() * find_perimeter(&region)).sum::<usize>())
}


pub fn part_two() {
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

    for region in &regions {
        println!("{:?} {} {}", region, region.len(), find_sides(&region));
    }
    println!("{}", regions.into_iter().map(|region| region.len() * find_sides(&region)).sum::<usize>())
}


pub fn parse_input() -> Vec<Vec<char>> {
    read_to_string("./inputs/day12.txt").unwrap()
        .lines().map(|map| map.chars().collect()).collect()
}

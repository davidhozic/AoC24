use std::{fs::{read_to_string, File}, io::Write, usize};


pub fn part_one() {
    const ROWS: isize = 103;
    const COLUMNS: isize = 101;

    let mut robots = parse_input();

    // Run simulation
    for _ in 0..100 {  // For each second
        for robot_i in 0..robots.len() {
            let (mut x, mut y, vx, vy) = robots[robot_i];
            x = (x + vx + COLUMNS) % COLUMNS;
            y = (y + vy + ROWS) % ROWS;
            robots[robot_i] = (x, y, vx, vy);
        }
    }

    // Count in quadrants
    let mut counts = [[0; 2]; 2];
    for (x, y, _, _) in robots {
        let middle = (ROWS / 2, COLUMNS / 2);
        if y == middle.0 || x == middle.1 {  // Middle robots aren't in any quadrant
            continue;
        }

        counts[(y > middle.0) as usize][(x > middle.1) as usize] += 1;
    }

    // Multiply the quadrant counts together
    println!("{}", counts.into_iter().flatten().reduce(|p, n| p * n).unwrap());
}


pub fn part_two() {
    const ROWS: isize = 103;
    const COLUMNS: isize = 101;

    let mut robots = parse_input();

    // Run simulation
    let mut file = File::create("./output.txt").unwrap();

    for i in 0..10_000 {  // For each second
        let mut map = [[0usize; COLUMNS as usize]; ROWS as usize];
        for robot_i in 0..robots.len() {
            let (mut x, mut y, vx, vy) = robots[robot_i];
            x = (x + vx + COLUMNS) % COLUMNS;
            y = (y + vy + ROWS) % ROWS;
            robots[robot_i] = (x, y, vx, vy);
            map[y as usize][x as usize] = 1;
        }


        let mut map_new = [['.'; COLUMNS as usize]; ROWS as usize];
        let mut to_print = false;
        // Create a new map with '#' where the individual element is 
        // part of a larget cluster.
        // Here we take a window of 5x5 from each element and then check if the number of robots
        // in this cluster is larger than 10; this way we eliminate useless robots
        // that make it hard to see any real patterns.
        for r in 0..map.len() - 10 {
            for c in 0..map[0].len() - 10 {
                if map[r..r+5].iter().map(|row| row[c..c+5].iter().sum::<usize>()).sum::<usize>() > 10 {
                    map_new[r][c] = '#';
                    to_print = true;
                }
            }
        }

        // Print the new map if any larger clusters were found.
        // The answer is then obtained through (human) visual inspection.
        if to_print {
            file.write_all("---------------\n".as_bytes()).unwrap();
            for line in map_new {
                file.write_all(format!("{line:3?}\n").as_bytes()).unwrap();
            }

            // Print the index. The number of seconds that elapsed is i + 1 (for the puzzle answer).
            file.write_all(format!("=============\n{i}\n=============\n").as_bytes()).unwrap();
        }
    }
}


pub fn parse_input() -> Vec<(isize, isize, isize, isize)> {
    let input = read_to_string("./inputs/day14.txt").unwrap();
    input.lines().map(|line| {
        let (p, v) = line.split_once(" ").unwrap();
        let p = p.split_once("=").unwrap().1;
        let v = v.split_once("=").unwrap().1;
        let (x, y) = p.split_once(",").unwrap();
        let (vx, vy) = v.split_once(",").unwrap();

        (x.parse().unwrap(), y.parse().unwrap(), vx.parse().unwrap(), vy.parse().unwrap())
    }).collect()
}


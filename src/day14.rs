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
        const REGION_SIZE: usize = 10;
        // Create a new map with '#' where the individual element is 
        // part of a larget cluster.
        // Here we take a window of REGION_SIZExREGION_SIZE from each element and then check if the number of robots
        // in this cluster is larger than 10; this way we eliminate useless robots
        // that make it hard to see any real patterns.
        for r in (0..map.len() - REGION_SIZE).step_by(REGION_SIZE) {
            for c in (0..map[0].len() - REGION_SIZE).step_by(REGION_SIZE) {
                if map[r..r+REGION_SIZE].iter().map(|row| row[c..c+REGION_SIZE].iter().sum::<usize>()).sum::<usize>() > REGION_SIZE * REGION_SIZE / 2 {
                    map_new[r][c] = '#';
                    to_print = true;
                }
            }
        }

        // Print the new map if any larger clusters were found.
        // This is our tree
        if to_print {
            file.write_all("---------------\n".as_bytes()).unwrap();
            for line in map {
                let line = line.map(|x| if x == 0 {' '} else {'#'});
                file.write_all(format!("{line:3?}\n").as_bytes()).unwrap();
            }

            // Print the number of iterations (puzzle answer);
            file.write_all(format!("=============\n{}\n=============\n", i + 1).as_bytes()).unwrap();
            println!("Num seconds for tree: {}", i + 1);
            break;
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


use std::fs::read_to_string;


/// Parses and returns two vectors. The left contains numbers from the left column,
/// the right contains numbers from the right column.
pub fn parse_input() -> (Vec<usize>, Vec<usize>) {
    let input = read_to_string("./inputs/day1.txt").unwrap();

    let mut left_list: Vec<usize> = Vec::new();
    let mut right_list: Vec<usize> = Vec::new();

    // Parse the left and right numbers
    for line in input.lines() {
        let (l, r) = line.split_once(" ").map(|(l, r)| (l.trim(), r.trim())).unwrap();
        left_list.push(l.parse().unwrap());
        right_list.push(r.parse().unwrap());
    }

    (left_list, right_list)
}

/// Part one: Calculate the distances between smallest numbers in the left and right lists (pairs);
pub fn part_one() {
    // Read the puzzle input
    let (mut left_list, mut right_list) = parse_input();

    // Pair smallest from the left with smallest from the right and sum
    left_list.sort();
    right_list.sort();

    let mut total_distance: usize = 0;
    for (left, right) in left_list.into_iter().zip(right_list) {
        total_distance += left.abs_diff(right);
    }

    println!("Total distance is {total_distance}");
}


/// Part two: Calculate the similarity score, which is a sum of multiplications between a number in the left
/// list and the number of times it appears in the right list.
pub fn part_two() {
    let (left_list, right_list) = parse_input();

    let mut score: usize = 0;
    for num in left_list {
        score += num * right_list.iter().filter(|x| **x == num).count();
    }

    println!("Total score is {score}");
}

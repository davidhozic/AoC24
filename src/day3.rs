use std::fs::read_to_string;
use regex::Regex;


const PATTERN_MUL: &str = r"mul\(([0-9]+),\s{0,}([0-9]+)\)";
const PATTERN_MUL_CONDITIONAL: &str = r"mul\(([0-9]+),\s{0,}([0-9]+)\)|(do\(\))|(don't\(\))";


pub fn parse_input() -> String  {
    read_to_string("./inputs/day3.txt").unwrap()
}


/// Evaluates the uncorrupted multiplication instructions and
/// sums up the results.
pub fn part_one() {
    let input = parse_input();
    let re = Regex::new(PATTERN_MUL).unwrap();
    let mut pleft: usize;
    let mut pright: usize;
    let mut sum = 0;

    for capture in re.captures_iter(&input) {
        let (_, [left, right]) = capture.extract();        
        pleft = left.parse().unwrap();
        pright = right.parse().unwrap();
        sum += pleft * pright;
    }

    println!("The sum is {sum}.");
}

/// Evaluates the uncorrupted multiplication instructions and
/// sums up the results. Additionaly, takes into account the
/// ``do`` and ``don't`` commands.
pub fn part_two() {
    let input = parse_input();
    let re = Regex::new(PATTERN_MUL_CONDITIONAL).unwrap();
    let mut pleft: usize;
    let mut pright: usize;
    let mut sum = 0;

    let mut enabled = true;
    for capture in re.captures_iter(&input) {
        if capture.get(3).is_some() {  // Group that matches enabled
            enabled = true;
        }
        else if capture.get(4).is_some() {  // Group that matches disabled
            enabled = false;
        }
        else if enabled {  // Normal multiplication
            pleft = capture.get(1).unwrap().as_str().parse().unwrap();
            pright = capture.get(2).unwrap().as_str().parse().unwrap();
            sum += pleft * pright;
        }
    }
    println!("The sum is {sum}.");
}

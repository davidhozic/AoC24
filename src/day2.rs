use std::fs::read_to_string;


pub fn parse_input() -> Vec<Vec<isize>>  {
    let input = read_to_string("./inputs/day2.txt").unwrap();
    input.lines()
        .map(|line| line.split_whitespace().map(|x: &str| x.parse().unwrap()).collect())
        .collect()
}


/// Checks if the ``report`` is safe.
fn is_report_safe<'a, T: Iterator<Item=&'a isize>>(mut report: T) -> bool {
    let mut diff;
    let mut abs_diff;

    let _0 = report.next().unwrap();
    let _1 = report.next().unwrap();
    diff = _1 - _0;
    abs_diff = diff.abs();
    if abs_diff < 1 || abs_diff > 3 {
        return false;
    }

    let direction = diff / abs_diff;
    let mut prev_level = _1;

    for level in report {
        diff = level - prev_level;
        abs_diff = diff.abs();
        if abs_diff < 1 || abs_diff > 3 || diff / abs_diff != direction {
            return false;
        }

        prev_level = level;
    }

    return true;
}



/// Checks if the report is safe, while allowing one wrong level
fn is_report_safe_dampened(report: Vec<isize>) -> bool {
    for i in 0..report.len() {
        // Iterate, enumerate (gets (inex, element)), filter by index, turn back to iterator over elements.
        if is_report_safe(report.iter().enumerate().filter(|(ii, _)| *ii != i).map(|(_, x)| x)) {
            return true;
        }
    }
    return false;
}


/// Counts number of reports that are safe. A report is safe if the levels are monotonically increasing or decreasing
/// and the adjacent numbers differ at least 1 and at most 3.
pub fn part_one() {
    // Read the puzzle input
    let reports = parse_input();
    let mut num_safe: usize = 0;
    for report in reports {
        if is_report_safe(report.iter()){
            num_safe += 1;
        }
    }

    println!("Number of safe reports: {num_safe}");
}


/// Counts number of reports that are safe with damping enabled. When damping is enabled, a single element
/// can be removed to make the report safe.
pub fn part_two() {
    // Read the puzzle input
    let reports = parse_input();
    let mut num_safe: usize = 0;
    for report in reports {
        if is_report_safe_dampened(report){
            num_safe += 1;
        }
    }

    println!("Number of safe reports after damping: {num_safe}");
}

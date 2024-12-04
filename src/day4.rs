use std::fs::read_to_string;


pub fn parse_input() -> String  {
    read_to_string("./inputs/day4.txt").unwrap()
}



fn find_substring_dir(haystack: &str, needle: &str, column_direction: isize, row_direction: isize) -> usize {
    let mut count: usize = 0;
    let needle: Vec<_> = needle.chars().collect();
    let chars: Vec<_> = haystack.chars().collect();
    let haystack_line_len = haystack.lines().next().unwrap().len() as isize;
    let mut offset: usize = 0;
    let mut offseted_idx: isize;
    for i in 0..chars.len() {
        loop {
            offseted_idx = 
                i as isize +
                offset as isize * column_direction +
                haystack_line_len * offset as isize * row_direction;

            if offseted_idx < 0 || offseted_idx as usize >= chars.len() {
                break;
            }

            if chars[offseted_idx as usize] == needle[offset] {
                offset += 1;
            }
            else if chars[offseted_idx as usize] == '\n' {
                break;
            }
            else {
                offset = 0;
                break;
            }

            // Found needle
            if offset == needle.len() {
                offset = 0;
                count += 1;
                break;
            }
        }
    }

    count
}


fn find_substring(haystack: &str, needle: &str) -> usize {
    // Create 2D map
    let n = find_substring_dir(haystack, needle, 0, 1);
    println!("{n}");
    n
}


pub fn part_one() {
   let input = parse_input();
   find_substring(&input, "XMAS");
}

pub fn part_two() {
    
}

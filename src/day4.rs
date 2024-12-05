use std::collections::HashMap;
use std::fs::read_to_string;


pub fn parse_input() -> String  {
    read_to_string("./inputs/day4.txt").unwrap()
}



/// Counts appearance of a ``needle`` in ``haystack`` in the given direction.
fn find_substring_dir(haystack: &str, needle: &str, column_direction: isize, row_direction: isize) -> usize {
    let mut count: usize = 0;
    let needle: Vec<_> = needle.chars().collect();
    let chars: Vec<_> = haystack.chars().collect();
    let haystack_line_len = haystack.lines().next().unwrap().len() as isize + 1;
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
            // else if chars[offseted_idx as usize] == '\n' {
            //     break;
            // }
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


/// Finds indices of middle ``needle`` character inside ``haystack``.
fn find_substring_dir_middle_idx(haystack: &str, needle: &str, column_direction: isize, row_direction: isize) -> Vec<usize> {
    let needle: Vec<_> = needle.chars().collect();
    let chars: Vec<_> = haystack.chars().collect();
    let haystack_line_len = haystack.lines().next().unwrap().len() as isize + 1;
    let needle_half_len: isize = needle.len() as isize / 2;
    let mut offset: usize = 0;
    let mut offseted_idx: isize;
    let mut indices = Vec::new();

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
            // else if chars[offseted_idx as usize] == '\n' {
            //     break;
            // }
            else {
                offset = 0;
                break;
            }

            // Found needle
            if offset == needle.len() {
                offset = 0;

                let middle_index = 
                    offseted_idx - 
                    needle_half_len * column_direction - 
                    haystack_line_len * needle_half_len * row_direction;
                
                indices.push(middle_index as usize);
                break;
            }
        }
    }

    indices
}


/// Searches of appearance of XMAS strings in any direction.
pub fn part_one() {
   let input = parse_input();
   let mut n= 0;
    for ro in -1..2 {
        for co in -1..2 {
            n += find_substring_dir(&input, "XMAS", co, ro);
        }
    }
   println!("{n}");
}


/// Searches of appearance of two MAS in a X shape in any direction.
pub fn part_two() {
    let input = parse_input();
    let directions: [isize; 2] = [-1, 1];
    let mut counts = HashMap::new();
    let mut indices;
    for ro in &directions {
        for co in &directions {
            indices = find_substring_dir_middle_idx(&input, "MAS", *co, *ro);
            for idx in indices {
                counts.insert(idx, counts.get(&idx).unwrap_or(&0) + 1);
            }
        }
    }
    
    let mut total_cnt = 0;
    for v in counts.values() {
        if *v == 2 {
            total_cnt += 1;
        }
    }
    println!("{total_cnt}");
}

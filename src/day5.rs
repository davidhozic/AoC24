use std::fs::read_to_string;


pub fn parse_input() -> (Vec<(usize, usize)>, Vec<usize>)  {
    let input = read_to_string("./inputs/day5.txt").unwrap();
    let mut split  = input.split("\n\n").take(2);
    let (rules, updates) = (split.next().unwrap().trim(), split.next().unwrap().trim());
    let rules_vec: Vec<(usize, usize)> = Vec::new();
    let updates_vec = Vec::new();
    for rule in rules.lines() {
        let (l, r) = rule.split_once('|').unwrap();
        rules_vec.push((l.parse().unwrap(), r.parse().unwrap()));
    }
    (rules_vec, updates_vec)
}



pub fn part_one() {
    parse_input();
}


pub fn part_two() {
    
}

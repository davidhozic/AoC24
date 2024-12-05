use std::fs::read_to_string;


pub fn parse_input() -> (Vec<(usize, usize)>, Vec<Vec<usize>>)  {
    let input = read_to_string("./inputs/day5.txt").unwrap();
    let mut split  = input.split("\n\n").take(2);
    let (rules, updates) = (split.next().unwrap().trim(), split.next().unwrap().trim());
    let mut rules_vec: Vec<(usize, usize)> = Vec::new();
    let mut updates_vec: Vec<Vec<_>> = Vec::new();
    for rule in rules.lines() {
        let (l, r) = rule.split_once('|').unwrap();
        rules_vec.push((l.parse().unwrap(), r.parse().unwrap()));
    }

    for update in updates.lines() {
        updates_vec.push(update.split(",").map(|x| x.parse().unwrap()).collect());
    }

    (rules_vec, updates_vec)
}


/// Sums up the middle elements of the valid updates.
pub fn part_one() {
    let mut sum = 0;
    let (rules, updates) = parse_input();
    'update_loop: for update in updates {
        for (l, r) in rules.iter() {
            let lindex = update.iter().position(|x| x == l);
            let rindex = update.iter().position(|x| x == r);
            if lindex.is_none() || rindex.is_none() {  // Rule doesn't apply in this case
                continue;  // Skip to next rule
            }

            if lindex.unwrap() > rindex.unwrap() {  // Left appears afer right rule => invalid update
                continue 'update_loop;  // Skip to the next update.
            }
        }

        // All the rules were validated.
        let middle = update[update.len() / 2];
        sum += middle;
    }

    println!("{sum}");
}


/// Finds incorrect updates and fixes them, then it sums up the middle index
/// of the fixed updates that were originally incorrect.
pub fn part_two() {
    let mut sum = 0;
    let mut corrected = false;
    let (rules, updates) = parse_input();
    
    for mut update in updates {
        while correct_update(&rules, &mut update) {  // Keep correcting rule by rule until there is nothing to correct.
            corrected = true;
        }

        if corrected {
            let middle = update[update.len() / 2];
            sum += middle;
            corrected = false;
        }
    }

    println!("{sum}");
}

/// Corrects the update based on the rules
/// As soon as the first correction is made, ``true`` is returned, indicating
/// everything should be rechecked from start (since a fix can break other rules).
fn correct_update(rules: &Vec<(usize, usize)>, update: &mut Vec<usize>) -> bool {
    let mut updated = false;
    for (l, r) in rules.iter() {
        let lindex = update.iter().position(|x| x == l);
        let rindex = update.iter().position(|x| x == r);
        if lindex.is_none() || rindex.is_none() {  // Rule doesn't apply in this case
            continue;  // Skip to next rule
        }

        let lindex = lindex.unwrap();
        let rindex = rindex.unwrap();
        if lindex > rindex {  // Left appears afer right rule => invalid update
            // Correct the update
            (update[rindex], update[lindex]) = (update[lindex], update[rindex]);
            updated = true;
        }
    }
    updated
}

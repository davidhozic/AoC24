use std::{collections::HashMap, fs::read_to_string};


fn blink_stones_1(n_blinks: usize) -> usize {
    let mut stones = parse_input();

    for blink in 0..n_blinks {
        let mut new_stones = stones.clone();
        for (i, stone) in stones.iter().enumerate() {
            match stone {
                0 => {
                    new_stones[i] = 1;
                },
                n if ((n.ilog10() + 1) % 2 == 0) => {
                    let div = 10usize.pow((n.ilog10() + 1) / 2);
                    new_stones[i] = n / div;
                    new_stones.push(n % div);
                }
                n => {
                    new_stones[i] = n * 2024;
                }
            };
        }
        stones = new_stones;
    }
    stones.len()
}


fn blink_stones_2(stone: usize, n_blinks: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    let cache_key = &(stone, n_blinks);
    if let Some(count) = cache.get(cache_key) {
        return *count;
    }

    if n_blinks == 0 {
        return 1;
    }

    let cnt = match &stone {
        0 => blink_stones_2(1, n_blinks - 1, cache),
        n if ((n.ilog10() + 1) % 2 == 0) => {
            let div = 10usize.pow((n.ilog10() + 1) / 2);
            blink_stones_2(n / div, n_blinks - 1, cache) + blink_stones_2(n % div, n_blinks - 1, cache)
        }
        n => {
            blink_stones_2(n * 2024, n_blinks - 1, cache)
        }
    };

    cache.insert(*cache_key, cnt);
    cnt
}


pub fn part_one() {
    println!("Number of stones: {}", blink_stones_1(25));
}


pub fn part_two() {
    let stones = parse_input();
    let mut cache = HashMap::new();
    println!("Number of stones: {}", stones.into_iter().map(|stone| blink_stones_2(stone, 75, &mut cache)).sum::<usize>());
}


pub fn parse_input() -> Vec<usize> {
    let input = read_to_string("./inputs/day11.txt").unwrap();
    input.split_whitespace().map(|x| x.parse().unwrap()).collect()
}


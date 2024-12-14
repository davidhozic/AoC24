use std::fs::read_to_string;




pub fn part_one() {
    let machines = parse_input_1();
    let mut tokens = 0;
    for (bnt_a, bnt_b, prize) in &machines {
        // println!("A: {bnt_a:?}, B: {bnt_b:?}, Prize: {prize:?}");
        for ia in 0..=100 {
            for ib in (0..=100).rev() {
                let result_x = ib * bnt_b.0 + ia * bnt_a.0;
                let result_y = ib * bnt_b.1 + ia * bnt_a.1;

                if (result_x, result_y) == *prize {
                    // println!("A: {ia}, B: {ib}");
                    tokens += ia * 3 + ib * 1;
                    break;
                }
            }
        }
    };

    println!("{tokens}");
}


pub fn part_two() {
    let machines = parse_input_2();
    let mut tokens = 0;

    for (bnt_a, bnt_b, prize) in &machines {
        let (xa, ya) = *bnt_a;
        let (xb, yb) = *bnt_b;
        let (x, y) = *prize;

        let denom = xa as isize * yb as isize - ya as isize * xb as isize;
        let ia_numerator = x as isize * yb as isize - y as isize * xb as isize;
        let ib_numerator = y as isize * xa as isize - x as isize * ya as isize;

        if ia_numerator % denom != 0 || ib_numerator % denom != 0 {
            continue;
        }

        let ia = ia_numerator / denom;
        let ib = ib_numerator / denom;
        tokens += ia as usize * 3 + ib as usize;
    }

    println!("{tokens}");
}


pub fn parse_input_1() -> Vec<((isize, isize), (isize, isize), (isize, isize))> {
    let data = read_to_string("./inputs/day13.txt").unwrap();

    let parsed = data.split("\n\n")  // Machines
    .map(|machine| {
        let mut lines = machine.split("\n");
        let bnt_a = lines.next().unwrap()
            .split_once(": ").unwrap().1
            .split_once(", ").unwrap();

        let bnt_b = lines.next().unwrap()
            .split_once(": ").unwrap().1
            .split_once(", ").unwrap();

        let (_, price_location)  = lines.next().unwrap().split_once(": ").unwrap();
        let (x, y) = price_location.split_once(", ").unwrap();

        (
            (bnt_a.0.split_once("+").unwrap().1.parse().unwrap(),  // BA X
            bnt_a.1.split_once("+").unwrap().1.parse().unwrap()),  // BA y
            (bnt_b.0.split_once("+").unwrap().1.parse().unwrap(),  // BB X
            bnt_b.1.split_once("+").unwrap().1.parse().unwrap()),  // BB y
            (x.split_once("=").unwrap().1.parse().unwrap(), y.split_once("=").unwrap().1.parse().unwrap())
        )
    }).collect();

    parsed
}


pub fn parse_input_2() -> Vec<((usize, usize), (usize, usize), (usize, usize))> {
    let data = read_to_string("./inputs/day13.txt").unwrap();

    let parsed = data.split("\n\n")  // Machines
    .map(|machine| {
        let mut lines = machine.split("\n");
        let bnt_a = lines.next().unwrap()
            .split_once(": ").unwrap().1
            .split_once(", ").unwrap();

        let bnt_b = lines.next().unwrap()
            .split_once(": ").unwrap().1
            .split_once(", ").unwrap();

        let (_, price_location)  = lines.next().unwrap().split_once(": ").unwrap();
        let (x, y) = price_location.split_once(", ").unwrap();

        (
            (bnt_a.0.split_once("+").unwrap().1.parse().unwrap(),  // BA X
            bnt_a.1.split_once("+").unwrap().1.parse().unwrap()),  // BA y
            (bnt_b.0.split_once("+").unwrap().1.parse().unwrap(),  // BB X
            bnt_b.1.split_once("+").unwrap().1.parse().unwrap()),  // BB y
            (
                x.split_once("=").unwrap().1.parse::<usize>().unwrap() + 10000000000000,
                y.split_once("=").unwrap().1.parse::<usize>().unwrap() + 10000000000000
            )
        )
    }).collect();

    parsed
}

use std::fs::read_to_string;


/// Receives a list of operands and the expected result when correct operators are used.
/// Then sums up the expected results of the cases that can be solved.
/// Each operator permutation is tested. Since there are only two operators (+ and *), the permutations
/// are being tracked in a [`usize`] variable whose bits represend the individual operator.
pub fn part_one() {
    let expressions = parse_input();
    let mut operators: usize;
    let mut max_operators: usize;
    let mut expr_result: usize;
    let mut valid_sum: usize = 0;

    for (expected_result, operands) in expressions {
        operators = 0;  // Each bit represents the operator (0 == summation, 1 == multiplication)
        max_operators = 2usize.pow((operands.len() - 1) as u32); // Value of bit whose index is equal to len

        loop {
            expr_result = operands[0];
            for shift in 1..operands.len() {
                let op: usize = operators & (1 << (shift - 1));
                if op == 0 {  // Plus
                    expr_result = expr_result + operands[shift];
                }
                else {  // Mul
                    expr_result = expr_result * operands[shift];
                }
            }
            if expr_result == expected_result {  // The operators give the desired test value
                valid_sum += expected_result;
                break;
            }

            operators = (operators + 1) % max_operators;
            if operators == 0 {  // Reached the end of expression permutations.
                break;
            }
        }
    }

    println!("{valid_sum}");
}


pub fn part_two() {

}


pub fn parse_input() -> Vec<(usize, Vec<usize>)> {
    let input = read_to_string("./inputs/day7.txt").unwrap();
    input.lines().map(
        |line| {
            let (result, operands) = line.split_once(":").unwrap();
            (
                result.parse().unwrap(),
                operands.split_whitespace()
                    .map(|x| x.parse().unwrap()).collect()
            )
        }
    ).collect()
}

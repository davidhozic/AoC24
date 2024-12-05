mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use utils::*;


fn main() {
    day5::part_one();
    benchmark(day5::part_two, 1000);
}

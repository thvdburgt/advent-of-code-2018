extern crate advent_of_code_2018_day_6;

use advent_of_code_2018_day_6::*;

const SAMPLE_INPUT: &str = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

#[test]
fn part_1_example() {
    assert_eq!(solve_puzzle_part_1(SAMPLE_INPUT), 17);
}

#[test]
fn part_2_example() {
    assert_eq!(solve_puzzle_part_2(SAMPLE_INPUT, 32), 16);
}

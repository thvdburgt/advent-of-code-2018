extern crate advent_of_code_2018_day_3;

use advent_of_code_2018_day_3::*;

const SAMPLE_CLAIMS: &str = "\
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2\
";

#[test]
fn part_1_example() {
    assert_eq!(solve_puzzle_part_1(SAMPLE_CLAIMS), 4);
}

#[test]
fn part_2_example() {
    assert_eq!(solve_puzzle_part_2(SAMPLE_CLAIMS), 3);
}

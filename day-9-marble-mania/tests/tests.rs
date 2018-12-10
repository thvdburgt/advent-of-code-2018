extern crate advent_of_code_2018_day_9;

use advent_of_code_2018_day_9::*;

#[test]
fn part_1_example() {
    assert_eq!(solve_puzzle_part_1(9, 25), 32);
}

#[test]
fn part_1_additional_examples() {
    assert_eq!(solve_puzzle_part_1(10, 1618), 8317);
    assert_eq!(solve_puzzle_part_1(13, 7999), 146373);
    assert_eq!(solve_puzzle_part_1(17, 1104), 2764);
    assert_eq!(solve_puzzle_part_1(21, 6111), 54718);
    assert_eq!(solve_puzzle_part_1(30, 5807), 37305);
}

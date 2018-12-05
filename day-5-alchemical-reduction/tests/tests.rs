extern crate advent_of_code_2018_day_5;

use advent_of_code_2018_day_5::*;

#[test]
fn part_1_examples() {
    assert_eq!(solve_puzzle_part_1("aA"), 0);
    assert_eq!(solve_puzzle_part_1("abBA"), 0);
    assert_eq!(solve_puzzle_part_1("abAB"), 4);
    assert_eq!(solve_puzzle_part_1("aabAAB"), 6);
    assert_eq!(solve_puzzle_part_1("dabAcCaCBAcCcaDA"), 10);
}

#[test]
fn part_2_examples() {
    assert_eq!(solve_puzzle_part_2("dabAcCaCBAcCcaDA"), 4);
}

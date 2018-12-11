extern crate advent_of_code_2018_day_11;

use advent_of_code_2018_day_11::*;

#[test]
fn part_1_example() {
    assert_eq!(solve_puzzle_part_1(3, 0), "1,1");
    assert_eq!(solve_puzzle_part_1(300, 18), "33,45");
    assert_eq!(solve_puzzle_part_1(300, 42), "21,61");
}

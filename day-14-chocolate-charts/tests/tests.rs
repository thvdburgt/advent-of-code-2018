extern crate advent_of_code_2018_day_14;

use advent_of_code_2018_day_14::*;

#[test]
fn part_1_example() {
    assert_eq!(solve_puzzle_part_1(9), "5158916779");
    assert_eq!(solve_puzzle_part_1(5), "0124515891");
    assert_eq!(solve_puzzle_part_1(18), "9251071085");
    assert_eq!(solve_puzzle_part_1(2018), "5941429882");
}

#[test]
fn part_2_example() {
    assert_eq!(solve_puzzle_part_2(&[5, 1, 5, 8, 9]), 9);
    assert_eq!(solve_puzzle_part_2(&[0, 1, 2, 4, 5]), 5);
    assert_eq!(solve_puzzle_part_2(&[9, 2, 5, 1, 0]), 18);
    assert_eq!(solve_puzzle_part_2(&[5, 9, 4, 1, 4]), 2018);
}

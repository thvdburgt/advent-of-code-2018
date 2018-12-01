extern crate advent_of_code_2018_day_1;

use advent_of_code_2018_day_1::*;

#[test]
fn part_1_example() {
    assert_eq!(solve_puzzle_part_1("+1, +1, +1"), 3);
    assert_eq!(solve_puzzle_part_1("+1, +1, -2"), 0);
    assert_eq!(solve_puzzle_part_1("-1, -2, -3"), -6);
}

#[test]
fn part_2_example() {
    assert_eq!(solve_puzzle_part_2("+1, -1"), 0);
    assert_eq!(solve_puzzle_part_2("+3, +3, +4, -2. -4"), 10);
    assert_eq!(solve_puzzle_part_2("-6, +3, +8, +5. -6"), 5);
    assert_eq!(solve_puzzle_part_2("+7, +7, -2, -7, -4"), 14);
}

extern crate advent_of_code_2018_day_8;

use advent_of_code_2018_day_8::*;

// 2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
// A----------------------------------
//     B----------- C-----------
//                      D-----
const SAMPLE_INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

#[test]
fn part_1_example() {
    assert_eq!(solve_puzzle_part_1(SAMPLE_INPUT), 138);
}

#[test]
fn part_1_own_example() {
    // 2 3 1 3 0 1 10 10 11 12 1 1 0 1 99 2 1 1 2
    // A-----------------------------------------
    //     B------------------ C-----------
    //         X-----              D-----
    let input = "2 3 1 3 0 1 10 10 11 12 1 1 0 1 99 2 1 1 2";
    assert_eq!(solve_puzzle_part_1(input), 148);
}

#[test]
fn part_2_example() {
    assert_eq!(solve_puzzle_part_2(SAMPLE_INPUT), 66);
}

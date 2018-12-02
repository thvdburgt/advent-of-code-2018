extern crate advent_of_code_2018_day_2;

use advent_of_code_2018_day_2::*;

#[test]
fn part_1_example() {
    let ids = "abcdef bababc abbcde abcccd aabcdd abcdee ababab";
    assert_eq!(solve_puzzle_part_1(ids), 12);
}

#[test]
fn part_2_example() {
    let ids = "abcde fghij klmno pqrst fguij axcye wvxyz";
    assert_eq!(solve_puzzle_part_2(ids), Some("fgij".to_string()));
}

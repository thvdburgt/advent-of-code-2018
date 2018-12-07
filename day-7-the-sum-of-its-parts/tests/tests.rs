extern crate advent_of_code_2018_day_7;

use advent_of_code_2018_day_7::*;

const SAMPLE_INPUT: &str = "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

#[test]
fn part_1_example() {
    assert_eq!(solve_puzzle_part_1(SAMPLE_INPUT), "CABDFE");
}

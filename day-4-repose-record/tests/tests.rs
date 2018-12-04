extern crate advent_of_code_2018_day_4;

use advent_of_code_2018_day_4::*;

const SAMPLE_RECORDS: &str = "\
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up\
";

#[test]
fn part_1_example() {
    assert_eq!(solve_puzzle_part_1(SAMPLE_RECORDS), 10 * 24);
}
#[test]
fn part_2_example() {
    assert_eq!(solve_puzzle_part_2(SAMPLE_RECORDS), 99 * 45);
}

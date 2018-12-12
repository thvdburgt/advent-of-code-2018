extern crate advent_of_code_2018_day_12;

use advent_of_code_2018_day_12::*;

const INPUT_SAMPLE: &str = "\
initial state: #..#.#..##......###...###

..... => .
...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

#[test]
fn part_1_example() {
    assert_eq!(solve_puzzle_part_1(INPUT_SAMPLE), 325);;
}

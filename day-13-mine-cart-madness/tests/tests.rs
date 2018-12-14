extern crate advent_of_code_2018_day_13;

use advent_of_code_2018_day_13::*;

const SAMPLE_INPUT1: &str = r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";

#[test]
fn part_1_example() {
    assert_eq!(solve_puzzle_part_1(SAMPLE_INPUT1), (7, 3));
}

const SAMPLE_INPUT2: &str = r"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";

#[test]
fn part_2_example() {
    assert_eq!(solve_puzzle_part_2(SAMPLE_INPUT2), (6, 4));
}

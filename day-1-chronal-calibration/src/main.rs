use std::fs::File;
use std::io::prelude::*;

extern crate advent_of_code_2018_day_1 as advent;

fn main() {
    let mut file = File::open("input").expect("Could not open input");
    let mut input = String::new();
    file.read_to_string(&mut input).expect(
        "Could not read input",
    );

    let input = input.trim();
    println!(
        "the answer for part 1 is {}",
        advent::solve_puzzle_part_1(input)
    );
    println!(
        "the answer for part 2 is {}",
        advent::solve_puzzle_part_2(input)
    );
}

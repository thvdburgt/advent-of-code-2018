use std::io::{self, Read};

extern crate advent_of_code_2018_day_5 as advent;

fn main() -> io::Result<()> {
    // read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input.trim();

    // solve the puzzles
    let part_1_answer = advent::solve_puzzle_part_1(input);
    println!("the answer for part 1 is {}", part_1_answer);

    let part_2_answer = advent::solve_puzzle_part_2(input);
    println!("the answer for part 2 is {}", part_2_answer);

    Ok(())
}

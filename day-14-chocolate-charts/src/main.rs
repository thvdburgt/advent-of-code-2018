extern crate advent_of_code_2018_day_14 as advent;

fn main() -> std::io::Result<()> {
    let default_input = 165_061;

    let input = std::env::args()
        .nth(1)
        .map(|x| x.parse().unwrap())
        .unwrap_or(default_input);

    // solve the puzzles
    let part_1_answer = advent::solve_puzzle_part_1(input);
    println!("the answer for part 1 is {}", part_1_answer);

    let input: Vec<u32> = input
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let part_2_answer = advent::solve_puzzle_part_2(&input);
    println!("the answer for part 2 is {}", part_2_answer);

    Ok(())
}

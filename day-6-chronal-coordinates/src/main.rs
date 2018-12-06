extern crate advent_of_code_2018_day_6 as advent;

fn main() -> std::io::Result<()> {
    // determine input filepath
    let input_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input".to_string());

    // read input
    let input = std::fs::read_to_string(input_path)?;
    let input = input.trim();

    // solve the puzzles
    let part_1_answer = advent::solve_puzzle_part_1(input);
    println!("the answer for part 1 is {}", part_1_answer);

    let part_2_answer = advent::solve_puzzle_part_2(input);
    println!("the answer for part 2 is {}", part_2_answer);

    Ok(())
}

extern crate advent_of_code_2018_day_10 as advent;

fn main() -> std::io::Result<()> {
    // determine input filepath
    let input_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input".to_string());

    // read input
    let input = std::fs::read_to_string(input_path)?;
    let input = input.trim();

    // solve the puzzles
    let (message, duration) = advent::solve_puzzle(input);
    println!("{}", message);
    println!("duration: {}", duration);

    Ok(())
}

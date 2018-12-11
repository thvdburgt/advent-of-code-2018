extern crate advent_of_code_2018_day_11 as advent;

fn main() -> std::io::Result<()> {
    let default_serial = 5719;

    let serial = std::env::args()
        .nth(1)
        .map(|x| x.parse().unwrap())
        .unwrap_or(default_serial);

    // solve the puzzles
    let part_1_answer = advent::solve_puzzle_part_1(300, serial);
    println!("the answer for part 1 is {}", part_1_answer);

    let part_2_answer = advent::solve_puzzle_part_2(300, serial);
    println!("the answer for part 2 is {}", part_2_answer);

    Ok(())
}

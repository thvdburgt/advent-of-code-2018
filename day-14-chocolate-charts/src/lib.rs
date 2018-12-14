#[allow(dead_code)]
fn print_scoreboard(scoreboard: &[u32], elf1: usize, elf2: usize) {
    let s = scoreboard
        .iter()
        .map(|d| d.to_string())
        .enumerate()
        .map(|(i, s)| {
            if i == elf1 {
                "(".to_owned() + &s + ")"
            } else if i == elf2 {
                "[".to_owned() + &s + "]"
            } else {
                " ".to_owned() + &s + " "
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", s);
}

pub fn solve_puzzle_part_1(n: usize) -> String {
    let mut scoreboard = vec![3u32, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;

    while scoreboard.len() < n + 10 {
        let sum = scoreboard[elf1] + scoreboard[elf2];
        assert!(sum <= 18);

        if sum >= 10 {
            scoreboard.push(sum / 10);
        }

        scoreboard.push(sum % 10);

        elf1 = (elf1 + 1 + scoreboard[elf1] as usize) % scoreboard.len();
        elf2 = (elf2 + 1 + scoreboard[elf2] as usize) % scoreboard.len();
    }

    scoreboard[n..n + 10]
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join("")
}

pub fn solve_puzzle_part_2(digits: &[u32]) -> usize {
    let mut scoreboard = vec![3u32, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;

    loop {
        let sum = scoreboard[elf1] + scoreboard[elf2];
        assert!(sum <= 18);

        if sum >= 10 {
            scoreboard.push(sum / 10);
            if scoreboard.ends_with(digits) {
                return scoreboard.len() - digits.len();
            }
        }

        scoreboard.push(sum % 10);
        if scoreboard.ends_with(digits) {
            return scoreboard.len() - digits.len();
        }

        elf1 = (elf1 + 1 + scoreboard[elf1] as usize) % scoreboard.len();
        elf2 = (elf2 + 1 + scoreboard[elf2] as usize) % scoreboard.len();
    }
}

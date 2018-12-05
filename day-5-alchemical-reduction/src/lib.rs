use std::collections::HashSet;

fn trigger(s: &str) -> String {
    s.chars().fold(String::with_capacity(s.len()), |mut s, c| {
        match s.chars().last() {
            Some(l) if c != l && c.to_ascii_lowercase() == l.to_ascii_lowercase() => {
                s.pop();
            }
            _ => s.push(c),
        }
        s
    })
}

pub fn solve_puzzle_part_1(input: &str) -> usize {
    trigger(input).len()
}

pub fn solve_puzzle_part_2(input: &str) -> usize {
    let units: HashSet<_> = input.chars().map(|c| c.to_ascii_lowercase()).collect();

    units
        .iter()
        .map(|&u| {
            input
                .chars()
                .filter(|c| c.to_ascii_lowercase() != u)
                .collect::<String>()
        })
        .map(|s| trigger(&s).len())
        .min()
        .unwrap()
}

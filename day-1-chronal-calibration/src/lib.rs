pub fn solve_puzzle_part_1(input: &str) -> i32 {
    input
        .split_whitespace()
        .map(|s| s.trim_matches(|c: char| !(c.is_numeric() || c == '+' || c == '-')))
        .map(|s| s.parse::<i32>().unwrap())
        .sum()
}

pub fn solve_puzzle_part_2(input: &str) -> i32 {
    use std::collections::HashSet;

    let freq_changes: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.trim_matches(|c: char| !(c.is_numeric() || c == '+' || c == '-')))
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut found_freqs = HashSet::new();
    let mut freq = 0;

    loop {
        for freq_change in &freq_changes {
            if !found_freqs.insert(freq) {
                return freq;
            }
            freq += freq_change;
        }
    }
}

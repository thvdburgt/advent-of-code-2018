fn react(s: &str) -> (bool, String) {
    let mut result = String::new();
    let mut reacted = false;

    let mut prev_char = None;
    for c in s.chars() {
        let unit_type = c.to_ascii_lowercase();
        let unit_polarity = c.is_ascii_uppercase();
        let mut keep_char = true;

        if let Some((prev_type, prev_polarity)) = prev_char {
            if unit_type == prev_type && prev_polarity != unit_polarity {
                reacted = true;
                result.pop();
                keep_char = false;
                prev_char = None;
            }
        }

        if keep_char {
            result.push(c);
            prev_char = Some((unit_type, unit_polarity));
        }
    }

    (reacted, result)
}

fn trigger(s: &str) -> String {
    let mut s = s.to_string();
    while let (true, new_s) = react(&s) {
        s = new_s;
    }

    s
}

pub fn solve_puzzle_part_1(input: &str) -> usize {
    trigger(input).len()
}

fn remove_unit(s: &str, unit: char) -> String {
    assert!(unit.is_ascii_lowercase());

    s.chars()
        .filter(|c| c.to_ascii_lowercase() != unit)
        .collect()
}

pub fn solve_puzzle_part_2(input: &str) -> usize {
    let unit_types: Vec<_> = (b'a'..=b'z').map(|b| b as char).collect();

    unit_types
        .iter()
        .map(|&u| remove_unit(input, u))
        .map(|s| trigger(&s))
        .map(|s| s.len())
        .min()
        .unwrap()
}

use regex::Regex;
use std::collections::HashMap;

mod pot_row;

use crate::pot_row::PotRow;
type Notes = HashMap<[bool; 5], bool>;

fn parse_input(input: &str) -> (PotRow, Notes) {
    let mut lines = input.lines();

    let char_to_pot = |c: char| match c {
        '.' => false,
        '#' => true,
        _ => panic!(),
    };

    // initial state
    let initial_state = {
        let initial_state_re = Regex::new("initial state: ([.#]+)").unwrap();
        let caps = initial_state_re.captures(lines.next().unwrap()).unwrap();
        let initial_state = caps.get(1).unwrap().as_str();
        PotRow::new(initial_state.chars().map(char_to_pot))
    };

    // skip empty line
    lines.next();

    // notes
    let notes = {
        let notes_re = Regex::new("([.#]{5}) => ([.#])").unwrap();
        lines
            .map(|line| {
                let caps = notes_re.captures(line).unwrap();

                let pre = caps.get(1).unwrap().as_str();
                let pre: Vec<_> = pre.chars().map(char_to_pot).collect();
                let pre = [pre[0], pre[1], pre[2], pre[3], pre[4]];

                let post = caps.get(2).unwrap().as_str();
                let post = post.chars().nth(0).map(char_to_pot).unwrap();

                (pre, post)
            })
            .collect()
    };

    (initial_state, notes)
}

pub fn solve_puzzle_part_1(input: &str) -> isize {
    let (mut pot_row, notes) = parse_input(input);
    pot_row.grow_n_times(&notes, 20);
    pot_row.pot_with_plant_index_sum()
}

pub fn solve_puzzle_part_2(input: &str) -> isize {
    let (mut pot_row, notes) = parse_input(input);
    pot_row.grow_n_times(&notes, 50_000_000_000usize);
    pot_row.pot_with_plant_index_sum()
}

mod cart;

use std::collections::HashSet;

use crate::cart::{Cart, Direction};

fn parse_track(input: &str) -> (Vec<Vec<char>>, Vec<Cart>) {
    let mut carts = Vec::new();

    let mut track = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '|' | '-' | '\\' | '/' | '+' => row.push(c),
                '^' => {
                    carts.push(Cart::new(x, y, Direction::North));
                    row.push('|')
                }
                '>' => {
                    carts.push(Cart::new(x, y, Direction::East));
                    row.push('-')
                }
                'v' => {
                    carts.push(Cart::new(x, y, Direction::South));
                    row.push('|')
                }
                '<' => {
                    carts.push(Cart::new(x, y, Direction::West));
                    row.push('-')
                }
                ' ' => row.push(' '),
                _ => panic!(),
            }
        }
        track.push(row);
    }
    (track, carts)
}

#[allow(dead_code)]
fn track_with_carts(track: &[Vec<char>], carts: &[Cart]) -> Vec<Vec<char>> {
    let mut track = track.to_vec();

    for cart in carts {
        let c = match &cart.direction {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
        };
        track[cart.y][cart.x] = c;
    }

    track
}

#[allow(dead_code)]
fn print_track(track: &[Vec<char>]) {
    for row in track {
        println!("{}", row.iter().collect::<String>());
    }
}

fn move_all(carts: &mut Vec<Cart>, track: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let mut all_crashed_carts_indices = HashSet::new();
    let mut crash_locations = HashSet::new();

    for i in 0..carts.len() {
        if all_crashed_carts_indices.contains(&i) {
            continue;
        }

        carts[i].move_once(&track);
        let location = (carts[i].x, carts[i].y);

        // get all indices of all carts this one crashed into
        let crashed_carts_indices: HashSet<usize> = carts
            .iter()
            .enumerate()
            .filter(|(_, c)| (c.x, c.y) == location)
            .filter(|(j, _)| *j != i)
            .filter(|(j, _)| !all_crashed_carts_indices.contains(j))
            .map(|(j, _)| j)
            .collect();

        if !crashed_carts_indices.is_empty() {
            crash_locations.insert(location);
            all_crashed_carts_indices.insert(i);
            all_crashed_carts_indices.extend(crashed_carts_indices);
        }
    }

    *carts = carts
        .iter()
        .enumerate()
        .filter(|(i, _)| !all_crashed_carts_indices.contains(i))
        .map(|(_i, cart)| *cart)
        .collect();
    carts.sort();

    crash_locations
}

pub fn solve_puzzle_part_1(input: &str) -> (usize, usize) {
    let (track, mut carts) = parse_track(input);
    carts.sort();
    // print_track(&track_with_carts(&track, &carts));

    loop {
        let crash_locations = move_all(&mut carts, &track);
        if !crash_locations.is_empty() {
            assert_eq!(crash_locations.len(), 1);
            let (x, y) = crash_locations.into_iter().next().unwrap();

            // let mut track = track_with_carts(&track, &carts);
            // track[y][x] = 'X';
            // print_track(&track);
            return (x, y);
        }
        // print_track(&track_with_carts(&track, &carts));
    }
}

pub fn solve_puzzle_part_2(input: &str) -> (usize, usize) {
    let (track, mut carts) = parse_track(input);
    carts.sort();
    // print_track(&track_with_carts(&track, &carts));

    while carts.len() > 1 {
        move_all(&mut carts, &track);
        // print_track(&track_with_carts(&track, &carts));
    }

    assert!(carts.len() == 1);

    // let mut track = track_with_carts(&track, &carts);
    // track[carts[0].y][carts[0].x] = 'X';
    // print_track(&track);

    (carts[0].x, carts[0].y)
}

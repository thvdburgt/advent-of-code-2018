use std::collections::VecDeque;

pub fn solve_puzzle_part_1(nr_of_players: usize, nr_of_marbles: usize) -> usize {
    assert!(nr_of_players > 0);

    let mut scores = vec![0; nr_of_players];
    let mut circle = VecDeque::with_capacity(nr_of_marbles);
    circle.push_back(0);

    for marble in 1..=nr_of_marbles {
        if marble % 23 == 0 {
            for _ in 0..6 {
                let m = circle.pop_back().unwrap();
                circle.push_front(m);
            }

            scores[marble % nr_of_players] += marble + circle.pop_back().unwrap();
        } else {
            for _ in 0..2 {
                let m = circle.pop_front().unwrap();
                circle.push_back(m);
            }
            circle.push_front(marble);
        }
    }

    *scores.iter().max().unwrap()
}

pub fn solve_puzzle_part_2(nr_of_players: usize, nr_of_marbles: usize) -> usize {
    solve_puzzle_part_1(nr_of_players, nr_of_marbles * 100)
}

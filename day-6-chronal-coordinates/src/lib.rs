fn parse_coordinates(s: &str) -> Vec<(usize, usize)> {
    s.lines()
        .map(|c| {
            let mut c = c.split(',').map(str::trim);
            let x = c.next().unwrap().parse().unwrap();
            let y = c.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect()
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    let dx = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
    let dy = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };

    dx + dy
}

fn closest_coordinate(location: (usize, usize), coords: &[(usize, usize)]) -> Option<usize> {
    use std::cmp::Ordering;

    // loop over each coordinate, find the min distance to this location and if this is minimum is
    // unique
    let (min, unique) = coords
        .iter()
        .map(|&coord| manhattan_distance(coord, location))
        .enumerate()
        .fold(
            (None, false),
            |(min, unique): (Option<(usize, usize)>, bool), dist| {
                let (i, dist) = dist;
                if let Some((min_i, min)) = min {
                    match dist.cmp(&min) {
                        Ordering::Less => (Some((i, dist)), true),
                        Ordering::Equal => (Some((i, dist)), false),
                        Ordering::Greater => (Some((min_i, min)), unique),
                    }
                } else {
                    (Some((i, dist)), true)
                }
            },
        );

    if let Some((min_i, _min)) = min {
        if unique {
            return Some(min_i);
        }
    }
    None
}

pub fn solve_puzzle_part_1(input: &str) -> usize {
    let coords = parse_coordinates(input);

    let mut grid = {
        let (max_x, max_y) = coords.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
            (std::cmp::max(max_x, *x), std::cmp::max(max_y, *y))
        });
        let grid_row = vec![None; max_x + 1];
        vec![grid_row; max_y + 1]
    };

    for (y, row) in grid.iter_mut().enumerate() {
        for (x, pos) in row.iter_mut().enumerate() {
            *pos = closest_coordinate((x, y), &coords);
        }
    }

    // count occurences
    (0..coords.len())
        .map(|n| {
            let mut count = 0;
            for (y, row) in grid.iter().enumerate() {
                for (x, pos) in row.iter().enumerate() {
                    if *pos == Some(n) {
                        // check if it is not on the side (making the area infinite)
                        if y == 0 || y == grid.len() - 1 || x == 0 || x == row.len() - 1 {
                            return 0;
                        } else {
                            count += 1
                        }
                    }
                }
            }
            count
        })
        .max()
        .unwrap()
}

fn sum_distances_less_than(
    location: (usize, usize),
    coords: &[(usize, usize)],
    total: usize,
) -> bool {
    let sum_distances: usize = coords
        .iter()
        .map(|&coord| manhattan_distance(coord, location))
        .sum();
    sum_distances < total
}

pub fn solve_puzzle_part_2(input: &str, less_than_distance: usize) -> usize {
    let coords = parse_coordinates(input);

    let mut grid = {
        let (max_x, max_y) = coords.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
            (std::cmp::max(max_x, *x), std::cmp::max(max_y, *y))
        });
        let grid_row = vec![0; max_x + 1];
        vec![grid_row; max_y + 1]
    };

    for (y, row) in grid.iter_mut().enumerate() {
        for (x, pos) in row.iter_mut().enumerate() {
            *pos = if sum_distances_less_than((x, y), &coords, less_than_distance) {
                1
            } else {
                0
            }
        }
    }

    grid.iter().map(|row| row.iter().sum::<usize>()).sum()
}

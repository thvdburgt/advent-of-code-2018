mod fuelcellgrid;

use crate::fuelcellgrid::FuelCellGrid;

pub fn solve_puzzle_part_1(size: usize, serial: i32) -> String {
    let grid = FuelCellGrid::new(size, serial);

    let (x, y) = grid.best3x3();
    format!("{},{}", x + 1, y + 1)
}

pub fn solve_puzzle_part_2(size: usize, serial: i32) -> String {
    let grid = FuelCellGrid::new(size, serial);

    let (size, (x, y)) = grid.best_square();
    format!("{},{},{}", x, y, size)
}

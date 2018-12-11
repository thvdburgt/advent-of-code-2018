pub struct FuelCellGrid {
    size: usize,
    grid: Vec<Vec<i32>>, // Summed-area table
}

impl FuelCellGrid {
    pub fn new(size: usize, serial: i32) -> Self {
        assert!(size >= 3);

        let mut grid: Vec<Vec<i32>> = Vec::with_capacity(size);

        for y in 1..=size {
            let mut row_sum: i32 = 0;
            let mut row = Vec::with_capacity(size);
            for x in 1..=size {
                let power = Self::calculate_power(x, y, serial);
                let mut summed_area = power + row_sum;
                if let Some(prev_row) = grid.last() {
                    summed_area += prev_row[x - 1]
                }
                row_sum += power;
                row.push(summed_area);
            }

            assert_eq!(row.len(), size);
            grid.push(row);
        }

        assert_eq!(grid.len(), size);
        Self { size, grid }
    }

    fn calculate_power(x: usize, y: usize, serial: i32) -> i32 {
        let rack_id = x as i32 + 10;

        let mut power_level = rack_id * y as i32;
        power_level += serial;
        power_level *= rack_id;
        power_level %= 1000;
        power_level /= 100;
        power_level - 5
    }

    fn square_sum(&self, size: usize, x: usize, y: usize) -> i32 {
        assert!(size > 0);
        assert!(x + size <= self.size);
        assert!(y + size <= self.size);

        let shift = size - 1;
        let mut sum = self.grid[y + shift][x + shift];
        if x > 0 {
            sum -= self.grid[y + shift][x - 1];
        }
        if y > 0 {
            sum -= self.grid[y - 1][x + shift];
        }
        if y > 0 && x > 0 {
            sum += self.grid[y - 1][x - 1];
        }

        sum
    }

    fn best_square_of_size(&self, size: usize) -> (i32, (usize, usize)) {
        assert!(size > 0);
        assert!(size <= self.size);

        let mut best = None;
        for y in 0..=self.size - size {
            for x in 0..=self.size - size {
                let sum = self.square_sum(size, x, y);
                best = match best {
                    None => Some((sum, (x, y))),
                    Some((best, _pos)) if sum > best => Some((sum, (x, y))),
                    _ => best,
                }
            }
        }

        best.unwrap()
    }

    pub fn best3x3(&self) -> (usize, usize) {
        self.best_square_of_size(3).1
    }

    pub fn best_square(&self) -> (usize, (usize, usize)) {
        let (size, (_max, pos)) = (1..=self.size)
            .map(|size| (size, self.best_square_of_size(size)))
            .max_by_key(|(_, (max, _))| *max)
            .unwrap();

        (size, pos)
    }
}

#[test]
fn power_level_examples() {
    assert_eq!(FuelCellGrid::calculate_power(3, 5, 8), 4);
    assert_eq!(FuelCellGrid::calculate_power(122, 79, 57), -5);
    assert_eq!(FuelCellGrid::calculate_power(217, 196, 39), 0);
    assert_eq!(FuelCellGrid::calculate_power(101, 153, 71), 4);
}

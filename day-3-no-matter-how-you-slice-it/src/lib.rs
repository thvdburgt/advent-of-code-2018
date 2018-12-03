use std::collections::HashSet;

struct Claim {
    id: u32,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

use std::str::FromStr;
impl FromStr for Claim {
    type Err = ();

    // #1272 @ 444,951: 11x11
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        // id
        let id = words
            .next()
            .ok_or(())?
            .trim_start_matches('#')
            .parse()
            .map_err(|_| ())?;
        // @
        if words.next().ok_or(())? != "@" {
            return Err(());
        }
        // x, y
        let mut xy = words.next().ok_or(())?.trim_end_matches(':').split(',');
        let x = xy.next().ok_or(())?.parse::<usize>().map_err(|_| ())?;
        let y = xy.next().ok_or(())?.parse::<usize>().map_err(|_| ())?;
        if xy.next() != None {
            return Err(());
        }
        // width, height
        let mut width_height = words.next().ok_or(())?.split('x');
        let width = width_height
            .next()
            .ok_or(())?
            .parse::<usize>()
            .map_err(|_| ())?;
        let height = width_height
            .next()
            .ok_or(())?
            .parse::<usize>()
            .map_err(|_| ())?;

        let claim = Self {
            id,
            x,
            y,
            width,
            height,
        };

        Ok(claim)
    }
}

#[derive(Clone, PartialEq)]
enum SquareInch {
    Unclaimed,
    Claimed(u32),
    MultipleClaims,
}

struct Fabric {
    size: usize,
    fabric: Vec<Vec<SquareInch>>,
}

use SquareInch::*;
impl Fabric {
    fn new() -> Self {
        let size: usize = 1000;
        let mut fabric = Vec::with_capacity(size);
        let row = vec![Unclaimed; size];
        for _ in 0..size {
            fabric.push(row.to_vec())
        }

        Self { size, fabric }
    }

    fn grow(&mut self, size: usize) {
        assert!(size > self.size);
        for row in &mut self.fabric {
            for _ in 0..(self.size - size) {
                row.push(Unclaimed);
            }
        }

        let new_row = vec![Unclaimed; size];
        for _ in 0..(self.size - size) {
            self.fabric.push(new_row.to_vec());
        }

        assert!(self.fabric.len() == size);
    }

    fn make_claim(&mut self, c: &Claim) -> HashSet<u32> {
        if c.x + c.width > self.size {
            self.grow(c.x + c.width);
        }
        if c.y + c.height > self.size {
            self.grow(c.y + c.height);
        }

        let mut set = HashSet::new();

        for row in c.y..(c.y + c.height) {
            for col in c.x..(c.x + c.width) {
                let square_inch = &mut self.fabric[row][col];
                *square_inch = match square_inch {
                    Unclaimed => Claimed(c.id),
                    Claimed(id) => {
                        set.insert(*id);
                        MultipleClaims
                    }
                    MultipleClaims => {
                        set.insert(c.id);
                        MultipleClaims
                    }
                }
            }
        }

        if !set.is_empty() {
            set.insert(c.id);
        }

        set
    }
}

pub fn solve_puzzle_part_1(input: &str) -> usize {
    let mut fabric = Fabric::new();
    for line in input.lines() {
        let claim = line.parse().unwrap();
        fabric.make_claim(&claim);
    }

    fabric
        .fabric
        .iter()
        .flatten()
        .filter(|square_inch| **square_inch == MultipleClaims)
        .count()
}

pub fn solve_puzzle_part_2(input: &str) -> u32 {
    let mut fabric = Fabric::new();
    let claims: Vec<Claim> = input.lines().map(|line| line.parse().unwrap()).collect();

    let overlapping_ids: HashSet<u32> = claims
        .iter()
        .flat_map(|claim| fabric.make_claim(claim))
        .collect();

    for claim in claims {
        if !overlapping_ids.contains(&claim.id) {
            return claim.id;
        }
    }

    panic!("no non-overlapping claim found");
}

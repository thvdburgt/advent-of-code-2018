use std::collections::HashSet;

use claim::Claim;

#[derive(Clone, PartialEq)]
pub enum Unit {
    Unclaimed,
    Claimed(u32),
    MultipleClaims,
}

pub struct Fabric {
    size: usize,
    pub fabric: Vec<Vec<Unit>>,
}

impl Fabric {
    pub fn new() -> Self {
        let size = 1000;
        let mut fabric = Vec::with_capacity(size);

        let row = vec![Unit::Unclaimed; size];
        for _ in 0..size {
            fabric.push(row.to_vec())
        }

        Self { size, fabric }
    }

    fn grow(&mut self, size: usize) {
        assert!(size > self.size);
        for row in &mut self.fabric {
            for _ in 0..(self.size - size) {
                row.push(Unit::Unclaimed);
            }
        }

        let new_row = vec![Unit::Unclaimed; size];
        for _ in 0..(self.size - size) {
            self.fabric.push(new_row.to_vec());
        }

        assert!(self.fabric.len() == size);
    }

    pub fn make_claim(&mut self, c: &Claim) -> HashSet<u32> {
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
                    Unit::Unclaimed => Unit::Claimed(c.id),
                    Unit::Claimed(id) => {
                        set.insert(*id);
                        Unit::MultipleClaims
                    }
                    Unit::MultipleClaims => {
                        set.insert(c.id);
                        Unit::MultipleClaims
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

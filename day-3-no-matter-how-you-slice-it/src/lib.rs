mod claim;
mod fabric;

use claim::Claim;
use fabric::{Fabric, Unit};

use std::collections::HashSet;

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
        .filter(|square_inch| **square_inch == Unit::MultipleClaims)
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

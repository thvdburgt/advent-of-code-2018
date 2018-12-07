use std::str::FromStr;

use crate::step::Step;

pub struct Instruction {
    pub step: Step,
    pub requirement: char,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Step C must be finished before step A can begin.
        let mut s = s.split_whitespace().skip(1);
        let requirement = s.next().ok_or(())?.parse().map_err(|_| ())?;
        let mut s = s.skip(5);
        let step = s.next().ok_or(())?.parse().map_err(|_| ())?;

        if s.nth(2).is_some() {
            return Err(());
        }

        Ok(Self {
            step: Step(step),
            requirement,
        })
    }
}

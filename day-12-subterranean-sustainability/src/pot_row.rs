use std::collections::{HashMap, VecDeque};
use std::fmt;

use super::Notes;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct PotRow {
    pots: VecDeque<bool>,
    pot_zero_index: isize,
}

impl PotRow {
    pub fn new<I: Iterator<Item = bool>>(initial_pots: I) -> Self {
        Self {
            pots: initial_pots.collect(),
            pot_zero_index: 0,
        }
    }

    pub fn pot_with_plant_index_sum(&self) -> isize {
        self.pots
            .iter()
            .enumerate()
            .filter(|(_i, &plant)| plant)
            .map(|(i, _plant)| i as isize - self.pot_zero_index as isize)
            .sum()
    }

    fn normalise(&mut self) {
        {
            // make sure there are 5 empty pots at the front
            let empty_front = self.pots.iter().take_while(|&&plant| !plant).count();
            self.pot_zero_index += 5 - empty_front as isize;
            if empty_front < 5 {
                for _ in 0..(5 - empty_front) {
                    self.pots.push_front(false);
                }
            } else if empty_front > 5 {
                for _ in 0..(empty_front - 5) {
                    assert_eq!(self.pots.pop_front(), Some(false));
                }
            }
        }

        {
            // make sure there are 5 empty pots at the back
            let empty_back = self
                .pots
                .iter()
                .rev()
                .take_while(|&&plant| !plant)
                .count();
            if empty_back < 5 {
                for _ in 0..(5 - empty_back) {
                    self.pots.push_back(false);
                }
            } else if empty_back > 5 {
                for _ in 0..(empty_back - 5) {
                    assert_eq!(self.pots.pop_back(), Some(false));
                }
            }
        }
    }

    fn grow(&mut self, notes: &Notes) {
        self.normalise();

        // pots of the next generation
        let mut new_pots = VecDeque::with_capacity(self.pots.len());
        new_pots.push_back(false);
        new_pots.push_back(false);

        // check every 5 pots if it should grow
        for window in self
            .pots
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .as_slice()
            .windows(5)
        {
            if let Some(&post) = notes.get(window) {
                new_pots.push_back(post);
            } else {
                new_pots.push_back(false);
            }
        }

        self.pots = new_pots;
    }

    pub fn grow_n_times(&mut self, notes: &Notes, n: usize) {
        let mut seen: HashMap<VecDeque<bool>, (usize, isize)> = HashMap::new();

        for gen in 1..=n {
            self.grow(&notes);
            if let Some((prev_gen, prev_pot_zero_index)) = seen.get(&self.pots) {
                // fast forward current state
                let gens_to_go = n - gen;
                let diff_in_gen = gen - prev_gen;
                let diff_in_zero_index = self.pot_zero_index - prev_pot_zero_index;

                // first fast forward to the generation with the same state that is closest to n
                let steps = gens_to_go / diff_in_gen;
                self.pot_zero_index += steps as isize * diff_in_zero_index;

                // and do the rest
                self.grow_n_times(notes, gens_to_go % diff_in_gen);

                break;
            } else {
                seen.insert(self.pots.clone(), (gen, self.pot_zero_index));
            }
        }
    }
}

impl fmt::Display for PotRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, &pot) in self.pots.iter().enumerate() {
            if i as isize == self.pot_zero_index {
                write!(f, " ")?
            }
            write!(f, "{}", if pot { '#' } else { '.' })?;
        }
        write!(f, " ({})", self.pot_zero_index)
    }
}

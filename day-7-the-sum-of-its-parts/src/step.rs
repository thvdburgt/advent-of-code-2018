use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Step(pub char);

impl Step {
    pub fn duration(self, base_duration: usize) -> usize {
        base_duration + (self.0 as usize - 'A' as usize) + 1
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}
impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

use crate::step::Step;
use std::fmt;

pub struct Worker {
    _nr: usize,
    pub step: Option<Step>,
    pub seconds_left: usize,
}
impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(Step(step)) = self.step {
            write!(f, "{}", step)?
        } else {
            write!(f, ".")?
        }
        write!(f, "[{:02}]", self.seconds_left)
    }
}

impl Worker {
    pub fn new(nr: usize) -> Self {
        Self {
            _nr: nr,
            step: None,
            seconds_left: 0,
        }
    }

    pub fn give_job(&mut self, step: Step, base_duration: usize) {
        assert!(self.step.is_none());
        self.step = Some(step);
        self.seconds_left = step.duration(base_duration);
    }

    pub fn tick(&mut self) -> Option<Step> {
        self.seconds_left = self.seconds_left.saturating_sub(1);

        let mut result = None;
        if self.seconds_left == 0 {
            result = self.step;
            self.step = None
        }
        result
    }
}

use lazy_static::lazy_static;

use std::str::FromStr;

use regex::Regex;

type Position = (i32, i32);
type Velocity = (i32, i32);

pub struct Star {
    position: Position,
    velocity: Velocity,
}

impl Star {
    pub fn tick(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }

    pub fn tock(&mut self) {
        self.position.0 -= self.velocity.0;
        self.position.1 -= self.velocity.1;
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }
}

impl FromStr for Star {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
        }

        let captures: Vec<_> = RE
            .find_iter(s)
            .map(|capture| capture.as_str())
            .map(|s| s.parse::<i32>().map_err(|_| ()))
            .collect();

        let position = (captures[0]?, captures[1]?);
        let velocity = (captures[2]?, captures[3]?);

        Ok(Star { position, velocity })
    }
}

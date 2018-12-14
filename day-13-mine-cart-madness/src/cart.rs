use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Turn {
    Left,
    Right,
    Straight,
}

impl Turn {
    pub fn next(&mut self) {
        *self = match self {
            Left => Straight,
            Right => Left,
            Straight => Right,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Cart {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
    next_turn: Turn,
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        self.y
            .cmp(&other.y)
            .then(self.x.cmp(&other.x))
            .then(self.direction.cmp(&other.direction))
            .then(self.next_turn.cmp(&other.next_turn))
    }
}

use self::Direction::*;
use self::Turn::*;
impl Cart {
    pub fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self {
            x,
            y,
            direction,
            next_turn: Left,
        }
    }

    fn turn(&mut self, turn: Turn) {
        match turn {
            Left => {
                self.direction = match &self.direction {
                    North => West,
                    East => North,
                    South => East,
                    West => South,
                }
            }
            Right => {
                self.direction = match &self.direction {
                    North => East,
                    East => South,
                    South => West,
                    West => North,
                }
            }
            _ => {}
        }
    }

    pub fn move_once(&mut self, track: &[Vec<char>]) {
        match &self.direction {
            North => self.y -= 1,
            East => self.x += 1,
            South => self.y += 1,
            West => self.x -= 1,
        }

        match track[self.y][self.x] {
            '|' => {
                assert!(self.direction != East && self.direction != West);
            }
            '-' => {
                assert!(self.direction != North && self.direction != South);
            }
            '/' => {
                match &self.direction {
                    North | South => self.turn(Right),
                    East | West => self.turn(Left),
                };
            }
            '\\' => {
                match &self.direction {
                    North | South => self.turn(Left),
                    East | West => self.turn(Right),
                };
            }
            '+' => {
                self.turn(self.next_turn);
                self.next_turn.next();
            }
            _ => {}
        }
    }
}

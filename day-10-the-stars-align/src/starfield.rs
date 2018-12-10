use std::fmt;

use crate::star::Star;

pub struct StarField(Vec<Star>);

impl StarField {
    pub fn new<I: IntoIterator<Item = Star>>(stars: I) -> Self {
        StarField(stars.into_iter().collect())
    }

    pub fn tick(&mut self) {
        self.0.iter_mut().for_each(|star| star.tick());
    }

    pub fn tock(&mut self) {
        self.0.iter_mut().for_each(|star| star.tock());
    }

    pub fn width(&self) -> usize {
        let minx: i32 = self.0.iter().map(|s| s.position().0).min().unwrap();
        let maxx: i32 = self.0.iter().map(|s| s.position().0).max().unwrap();
        (maxx - minx + 1) as usize
    }

    pub fn height(&self) -> usize {
        let miny: i32 = self.0.iter().map(|s| s.position().1).min().unwrap();
        let maxy: i32 = self.0.iter().map(|s| s.position().1).max().unwrap();
        (maxy - miny + 1) as usize
    }
}

impl fmt::Display for StarField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut starfield: Vec<Vec<char>> = vec![vec![' '; self.width()]; self.height()];
        let minx: i32 = self.0.iter().map(|s| s.position().0).min().unwrap();
        let miny: i32 = self.0.iter().map(|s| s.position().1).min().unwrap();

        for star in &self.0 {
            starfield[(star.position().1 - miny) as usize][(star.position().0 - minx) as usize] = '#';
        }

        for row in starfield {
            for pos in row {
                write!(f, "{}", pos)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

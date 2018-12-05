pub struct Claim {
    pub id: u32,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

use std::str::FromStr;
impl FromStr for Claim {
    type Err = ();

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

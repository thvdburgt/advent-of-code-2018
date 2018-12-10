mod star;
mod starfield;

use crate::starfield::StarField;

pub fn solve_puzzle(input: &str) -> (String, usize) {
    let mut starfield = StarField::new(input.lines().map(|l| l.parse().unwrap()));

    let mut size = starfield.width() * starfield.height();
    let mut duration = 0;
    // keep moving the stars while the dimensions of the starfield is shrinking, if it starts
    // growing again we have gone one step to far.
    loop {
        starfield.tick();
        let new_size = starfield.width() * starfield.height();

        if new_size > size {
            starfield.tock();
            break;
        } else {
            size = new_size;
            duration += 1;
        }
    }

    (starfield.to_string(), duration)
}

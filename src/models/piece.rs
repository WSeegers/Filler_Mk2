use crate::models::point;

use point::Point;

use rand::prelude::*;
use std::fmt;

const EMPTY: char = '.';
const OCCUPIED: char = '*';

pub struct Piece {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<bool>,
    density: u32,
}

impl Piece {
    pub fn get(&self, p: Point) -> bool {
        match self.cells.get((self.width * p.y + p.x) as usize) {
            Some(c) => *c,
            None => panic!("Cells incorrectly initialized"),
        }
    }

    fn mutate_1(&mut self, x: u32, y: u32) {
        let mut rng = thread_rng();

        if x == 0 || y == 0 || x >= self.width - 1 || y >= self.height - 1 {
            return;
        }

        for _i in 0..9 {
            if rng.gen_range(
                0,
                (self.density * (self.width + self.height) / (self.width * self.height)) + 1,
            ) != 0
            {
                continue;
            }
            let dx = rng.gen_range(-1, 2);
            let dy = rng.gen_range(-1, 2);
            let x = (x as i32 + dx) as u32;
            let y = (y as i32 + dy) as u32;
            if self.cells[(y * self.width + x) as usize] == true {
                continue;
            }
            self.cells[(y * self.width + x) as usize] = true;
            self.density += 1;
            self.mutate_1(x, y);
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Piece {} {}:", self.height, self.width)?;
        for y in 0..(self.height) {
            for x in 0..(self.width) {
                let c = self.cells[(y * self.width + x) as usize];
                let c = match c {
                    true => OCCUPIED,
                    false => EMPTY,
                };
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

pub struct PieceBag {
    width_range: [u32; 2],
    height_range: [u32; 2],
}

impl PieceBag {
    pub fn new(width_range: [u32; 2], height_range: [u32; 2]) -> PieceBag {
        let mut p = PieceBag {
            width_range,
            height_range,
        };
        p.width_range.sort();
        p.height_range.sort();
        p
    }

    pub fn next(&self) -> Piece {
        let mut rng = thread_rng();

        let width = rng.gen_range(self.width_range[0], self.width_range[1]);
        let height = rng.gen_range(self.height_range[0], self.height_range[1]);

        let mut p = Piece {
            width,
            height,
            cells: vec![false; (width * height) as usize],
            density: 1,
        };

        let x = rng.gen_range(1, p.width - 1);
        let y = rng.gen_range(1, p.height - 1);
        p.cells[(y * p.width + x) as usize] = true;
        p.mutate_1(x, y);

        p
    }
}

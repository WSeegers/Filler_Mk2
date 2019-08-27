use super::point::Point;

use rand::prelude::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;

const EMPTY: char = '.';
const OCCUPIED: char = '*';

const RANGE_DEFAULT: [usize; 2] = [3, 8];

#[derive(Debug, Clone)]
pub struct Piece {
    width: usize,
    height: usize,
    cells: Vec<bool>,
    density: usize,
}

impl Piece {
    pub fn new(width: usize, height: usize, cells: Vec<bool>) -> Self {
        assert_eq!(width * height, cells.len());

        let p = Piece {
            width,
            height,
            cells,
            density: 1,
        };
        p
    }

    pub fn new_blank(width: usize, height: usize) -> Self {
        Piece::new(width, height, vec![false; width * height])
    }

    pub fn get(&self, p: Point) -> bool {
        match self.cells.get((self.width as i32 * p.y + p.x) as usize) {
            Some(c) => *c,
            None => panic!("Cells incorrectly initialized or out of bounds"),
        }
    }

    // This fn should act only as a placeholder till better function is made
    fn mutate(&mut self, x: usize, y: usize) -> &mut Self {
        let mut rng = thread_rng();

        if x == 0 || y == 0 || x >= self.width - 1 || y >= self.height - 1 {
            return self;
        }

        for _i in 0..8 {
            let likelyhood =
                (self.density * (self.width + self.height) / (self.width * self.height)) + 1;
            if rng.gen_range(0, likelyhood) != 0 {
                continue;
            }
            let dx = rng.gen_range(-1, 2);
            let dy = rng.gen_range(-1, 2);
            let x = (x as i32 + dx) as usize;
            let y = (y as i32 + dy) as usize;
            if self.cells[(y * self.width + x) as usize] == true {
                continue;
            }
            self.cells[(y * self.width + x) as usize] = true;
            self.density += 1;
            self.mutate(x, y);
        }
        self
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
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

impl Serialize for Piece {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("Person", 3)?;
        s.serialize_field("width", &self.width)?;
        s.serialize_field("height", &self.height)?;

        let cells: Vec<u8> = self
            .cells
            .iter()
            .map(|cell| match cell {
                true => 1,
                false => 0,
            })
            .collect();
        s.serialize_field("cells", &cells)?;
        s.end()
    }
}

pub struct PieceBag {
    width_range: [usize; 2],
    height_range: [usize; 2],
}

impl PieceBag {
    pub fn default() -> PieceBag {
        PieceBag {
            width_range: RANGE_DEFAULT,
            height_range: RANGE_DEFAULT,
        }
    }

    pub fn new(width_range: [usize; 2], height_range: [usize; 2]) -> PieceBag {
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

        let mut p = Piece::new_blank(width, height);

        let x = rng.gen_range(1, p.width - 1);
        let y = rng.gen_range(1, p.height - 1);
        p.cells[(y * p.width + x) as usize] = true;
        p.mutate(x, y);

        p
    }
}

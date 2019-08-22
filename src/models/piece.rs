use super::point::Point;

use rand::prelude::*;
use std::fmt;

use serde_json::json;

const EMPTY: char = '.';
const OCCUPIED: char = '*';

#[derive(Debug, Clone)]
pub struct Piece {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<bool>,
    density: u32,
    x_range: [u32; 2],
    y_range: [u32; 2],
}

impl Piece {
    pub fn new(width: u32, height: u32, cells: Vec<bool>) -> Self {
        assert_eq!(width * height, cells.len() as u32);

        let mut p = Piece {
            width,
            height,
            cells,
            density: 1,
            x_range: [width, 0],
            y_range: [height, 0],
        };
        p.set_range();
        p
    }

    pub fn new_blank(width: u32, height: u32) -> Self {
        Piece::new(width, height, vec![false; (width * height) as usize])
    }

    pub fn get(&self, p: Point) -> bool {
        match self.cells.get((self.width as i32 * p.y + p.x) as usize) {
            Some(c) => *c,
            None => panic!("Cells incorrectly initialized or out of bounds"),
        }
    }

    pub fn as_json(&self) -> String {
        let mut v = Vec::new();
        for val in self.cells.iter() {
            match val {
                true => v.push(1),
                false => v.push(0)
            };
        }

        let ret = json!({
            "w": self.width,
            "h": self.height,
            "cells": v
        });
        let ret = String::from(ret.to_string());
        ret
    }

    // This funciton should act only as a placeholder till better function is made
    fn mutate_1(&mut self, x: u32, y: u32) -> &mut Self {
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
            let x = (x as i32 + dx) as u32;
            let y = (y as i32 + dy) as u32;
            if self.cells[(y * self.width + x) as usize] == true {
                continue;
            }
            self.cells[(y * self.width + x) as usize] = true;
            self.density += 1;
            self.mutate_1(x, y);
        }
        self
    }

    fn set_range(&mut self) -> &mut Self {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.cells[(y * self.width + x) as usize] {
                    true => {
                        self.x_range = [self.x_range[0].min(x), self.x_range[1].max(x)];
                        self.y_range = [self.y_range[0].min(y), self.y_range[1].max(y)];
                    }
                    false => (),
                }
            }
        }
        self
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

        let mut p = Piece::new_blank(width, height);

        let x = rng.gen_range(1, p.width - 1);
        let y = rng.gen_range(1, p.height - 1);
        p.cells[(y * p.width + x) as usize] = true;
        p.mutate_1(x, y).set_range();

        p
    }
}

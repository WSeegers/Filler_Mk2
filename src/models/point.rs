pub use std::convert::TryFrom;
use std::ops::Add;

use serde::{Serialize};

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl TryFrom<&String> for Point {
    type Error = String;

    /// '1 2' -> Point {x: 1, y: 2}
    fn try_from(s: &String) -> Result<Self, String> {
        let coordinates: Vec<&str> = s.trim().split(" ").collect();

        let cy = match coordinates.get(0) {
            Some(s) => s,
            None => return Err(format!("Bad input: {}", s)),
        };
        let cx = match coordinates.get(1) {
            Some(s) => s,
            None => return Err(format!("Bad input: {}", s)),
        };
        let x = match cx.parse::<i32>() {
            Ok(i) => i,
            Err(_) => return Err(format!("Invalid x coordinate: {}", s)),
        };
        let y = match cy.parse::<i32>() {
            Ok(i) => i,
            Err(_) => return Err(format!("Invalid y coordinate: {}", s)),
        };

        Ok(Point { x, y })
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

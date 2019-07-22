use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Point {
	pub x: u32,
	pub y: u32,
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

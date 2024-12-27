use super::point::Point;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
	pub center: Point,
	pub radius: f64,
}

impl Circle {
	pub fn new(center: Point, radius: f64) -> Self {
		Self { center, radius }
	}
}

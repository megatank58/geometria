use std::{
	fmt::Display,
	ops::{Add, Sub},
};

use crate::{datatypes::angle::Angle, util::is_eq};

#[derive(Debug, Copy, Clone)]
pub struct Point {
	pub x: f64,
	pub y: f64,
}

impl Point {
	pub fn new(x: f64, y: f64) -> Self {
		Self { x, y }
	}

	pub fn abs(&self) -> f64 {
		(self.x.powf(2.0) + self.y.powf(2.0)).powf(0.5)
	}

	pub fn angle(&self) -> Angle {
		Angle::new((self.y / self.x).atan())
	}

	pub fn rotate(&self, angle: Angle) -> Self {
		let abs = self.abs();

		let angle = self.angle() + angle;

		Point::new(abs * angle.cos(), abs * angle.sin())
	}

	pub fn shift(&self, distance: f64, angle: Angle) -> Self {
		Point::new(self.x + distance * angle.cos(), self.y + distance * angle.sin())
	}
}

impl Add for Point {
	type Output = Point;

	fn add(self, rhs: Self) -> Self::Output {
		Point::new(self.x + rhs.x, self.y + rhs.y)
	}
}

impl Sub for Point {
	type Output = Point;

	fn sub(self, rhs: Self) -> Self::Output {
		Point::new(self.x + rhs.x, self.y + rhs.y)
	}
}

impl PartialEq for Point {
	fn eq(&self, other: &Self) -> bool {
		is_eq(self.x, other.x) && is_eq(self.y, other.y)
	}
}

impl Display for Point {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

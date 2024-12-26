use std::ops::{Add, Div, Mul, Sub};

use crate::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle {
	pub value: f64,
}

impl Angle {
	pub fn new(value: f64) -> Self {
		Self { value }
	}

	pub fn from_deg(value: f64) -> Self {
		PI * value / 180.0
	}

	pub fn cos(&self) -> f64 {
		self.value.cos()
	}

	pub fn sin(&self) -> f64 {
		self.value.sin()
	}

	pub fn tan(&self) -> f64 {
		self.value.tan()
	}
}

impl Add for Angle {
	type Output = Angle;

	fn add(self, rhs: Self) -> Self::Output {
		Angle::new(self.value + rhs.value)
	}
}

impl Sub for Angle {
	type Output = Angle;

	fn sub(self, rhs: Self) -> Self::Output {
		Angle::new(self.value - rhs.value)
	}
}

impl<T: Into<f64>> Mul<T> for Angle {
	type Output = Angle;

	fn mul(self, rhs: T) -> Self::Output {
		Angle::new(self.value * rhs.into())
	}
}

impl<T: Into<f64>> Div<T> for Angle {
	type Output = Angle;

	fn div(self, rhs: T) -> Self::Output {
		Angle::new(self.value / rhs.into())
	}
}

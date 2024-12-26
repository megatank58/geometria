use crate::{datatypes::radian::Radian, elements::point::Point};

pub const ORIGIN: Point = Point { x: 0.0, y: 0.0 };
pub const PI: Radian = Radian {
	value: std::f64::consts::PI,
};

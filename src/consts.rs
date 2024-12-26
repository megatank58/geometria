use crate::{datatypes::angle::Angle, elements::point::Point};

pub const ORIGIN: Point = Point { x: 0.0, y: 0.0 };
pub const PI: Angle = Angle {
	value: std::f64::consts::PI,
};

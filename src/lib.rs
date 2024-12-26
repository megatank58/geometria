pub mod consts;
pub mod datatypes;
pub mod elements;
mod util;

#[cfg(test)]
mod tests {
	use crate::{
		consts::{ORIGIN, PI},
		datatypes::angle::Angle,
		elements::{line::Line, plane::Plane, point::Point},
		util::is_eq,
	};

	#[test]
	fn util() {
		assert!(is_eq(0.1 + 0.2, 0.3))
	}

	#[test]
	fn point() {
		let p = Point::new(3.0, 4.0);

		assert_eq!(p.angle(), Angle::new(0.9272952180016122));
	}

	#[test]
	fn line() {
		let a = Point::new(0.0, 0.0);
		let b = Point::new(3.0, 4.0);

		let line = Line::from_points(a, b);

		assert!(Plane::is_intersecting(line, b));

		let l1 = Line::new(1.0, 5.0);
		let l2 = Line::new(1.0, 10.0);

		assert_eq!(Plane::distance(l1, l2), 3.5355339059327373);
	}

	#[test]
	fn intersection() {
		let a = Point::new(0.0, 0.0);
		let b = Point::new(3.0, 4.0);
		let c = Point::new(6.0, 8.0);

		let line = Line::from_points(a, b);

		assert!(Plane::is_intersecting(c, line));
	}

	#[test]
	fn rotation() {
		let a = Point::new(0.0, 0.0);
		let b = Point::new(3.0, 4.0);

		let line = Line::from_points(a, b);

		b.rotate(PI / 2);
		line.rotate(PI / 2, ORIGIN);
	}
}

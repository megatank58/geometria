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
		assert!(is_eq(0.1 + 0.2, 0.3));
	}

	#[test]
	fn point() {
		let p = Point::new(3.0, 4.0);

		assert_eq!(p.angle(), Angle::new(0.9272952180016122));
	}

	#[test]
	fn line() {
		let l1 = Line::new(1.0, 5.0);
		let l2 = Line::new(1.0, 10.0);

		assert_eq!(Plane::distance(l1, l2), 3.5355339059327373);
	}

	#[test]
	fn intersection() {
		let a = Point::new(3.0, 4.0);
		let b = Point::new(6.0, 8.0);

		let line = Line::from_points(ORIGIN, a);

		assert!(Plane::intersect(b, line).is_some());

		let l1 = Line::new(-2.0 / 3.0, 8.0 / 3.0);
		let l2 = Line::new(1.0 / 5.0, 9.0 / 5.0);
		let l3 = Line::new(-3.0 / 4.0, 11.0 / 4.0);

		assert_eq!(
			Plane::intersect(l1, l2).points(),
			Plane::intersect(l2, l3).points()
		);

		let l1 = Line::new(1.0, 0.0);
		let l2 = Line::new(-1.0, 0.0);

		assert_eq!(
			Plane::intersect(l1, l2).angles()[0],
			PI/2
		);
	}

	#[test]
	fn rotation() {
		let a = Point::new(3.0, 4.0);
		let b = Point::new(-4.0, 3.0);

		let l1 = Line::from_points(ORIGIN, a);
		let l2 = Line::from_points(ORIGIN, b);

		assert_eq!(a.rotate(PI / 2), b);
		assert_eq!(l1.rotate(PI / 2, ORIGIN), l2);
	}

	#[test]
	fn foot_of_perpendicular() {
		let line = Line::new(1.0, 0.0);

		let a = Point::new(-3.0, 3.0);
		let b = Point::new(0.0, 0.0);

		assert_eq!(Plane::foot_of_perpendicular(a, line).unwrap(), b);
	}

	#[test]
	fn image() {
		let l1 = Line::new(1.0, 0.0);

		let a = Point::new(-3.0, 3.0);
		let b = Point::new(3.0, -3.0);

		assert_eq!(Plane::image(a, l1).unwrap().to_point(), b);

		let l2 = Line::new(1.0, 2.0);

		assert_eq!(Plane::image(l1, l2).unwrap().to_line(), Line::new(1.0, 4.0));
	}
}

use std::error::Error;

use crate::{consts::PI, datatypes::angle::Angle, util::is_eq};

use super::{line::Line, point::Point, Element};

pub struct Plane {}

impl Plane {
	pub fn intersect<A: Into<Element>, B: Into<Element>>(a: A, b: B) -> Intersection {
		match (a.into(), b.into()) {
			(Element::Point(a), Element::Point(b)) => {
				if a == b {
					Intersection::Points(vec![(a, Angle::new(0.0))])
				} else {
					Intersection::None
				}
			}
			(Element::Point(p), Element::Line(line)) | (Element::Line(line), Element::Point(p)) => {
				if is_eq(p.y, line.slope * p.x + line.y_intercept) {
					Intersection::Points(vec![(p, Angle::new(0.0))])
				} else {
					Intersection::None
				}
			}
			(Element::Line(l1), Element::Line(l2)) => {
				if !is_eq(l1.slope, l2.slope) {
					let x = (l2.y_intercept - l1.y_intercept) / (l1.slope - l2.slope);
					let y = l1.slope * x + l1.y_intercept;

					let angle = Angle::new(
						((l1.slope - l2.slope) / (1.0 + l1.slope * l2.slope))
							.abs()
							.atan(),
					);
					Intersection::Points(vec![(Point::new(x, y), angle)])
				} else if is_eq(l1.y_intercept, l2.y_intercept) {
					Intersection::Infinite
				} else {
					Intersection::None
				}
			}
			(Element::Point(p), Element::Circle(circle))
			| (Element::Circle(circle), Element::Point(p)) => {
				if is_eq(Plane::distance(circle.center, p), circle.radius) {
					Intersection::Points(vec![(p, Angle::new(0.0))])
				} else {
					Intersection::None
				}
			}
			(Element::Line(line), Element::Circle(circle))
			| (Element::Circle(circle), Element::Line(line)) => {
				if is_eq(Plane::distance(circle.center, line), circle.radius) {
					let p = Plane::foot_of_perpendicular(circle.center, line).unwrap();

					Intersection::Points(vec![(p, PI / 2)])
				} else if Plane::distance(circle.center, line) < circle.radius {
					let p = Plane::foot_of_perpendicular(circle.center, line).unwrap();

					let perpendicular_distance = Plane::distance(p, circle.center);

					let half_chord_length =
						(circle.radius.powf(2.0) - perpendicular_distance.powf(2.0)).sqrt();

					let p1 = p.shift(half_chord_length, line.angle());
					let p2 = p.shift(-half_chord_length, line.angle());

					let angle = Angle::new((perpendicular_distance / half_chord_length).atan());

					Intersection::Points(vec![(p1, angle), (p2, angle)])
				} else {
					Intersection::None
				}
			}
			(Element::Circle(..), Element::Circle(..)) => todo!(),
		}
	}

	pub fn distance<A: Into<Element>, B: Into<Element>>(a: A, b: B) -> f64 {
		match (a.into(), b.into()) {
			(Element::Point(a), Element::Point(b)) => {
				((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).sqrt()
			}
			(Element::Point(p), Element::Line(line)) | (Element::Line(line), Element::Point(p)) => {
				(line.slope * p.x + line.y_intercept - p.y).abs()
					/ (1.0 + line.slope.powf(2.0)).sqrt()
			}
			(Element::Line(l1), Element::Line(l2)) => {
				if !(Plane::intersect(l1, l2).is_some()) {
					(l1.y_intercept - l2.y_intercept).abs() / (1.0 + l1.slope.powf(2.0)).sqrt()
				} else {
					0.0
				}
			}
			_ => unimplemented!(),
		}
	}

	pub fn foot_of_perpendicular<A: Into<Element>, B: Into<Element>>(
		a: A,
		b: B,
	) -> Result<Point, Box<dyn Error>> {
		match (a.into(), b.into()) {
			(Element::Point(..), Element::Point(b)) => Ok(b),
			(Element::Point(p), Element::Line(line)) => {
				let c = - (line.slope * p.x - p.y + line.y_intercept)/(line.slope.powf(2.0) + 1.0);

				Ok(Point::new(p.x + line.slope * c, p.y - c))
			},
			(Element::Line(..), Element::Point(..)) => Err("attempt to drop perpendicular from line to point, did you mean to drop from point to line?".into()),
			(Element::Line(..), Element::Line(..)) => Err("attempt to drop perpendicular from line to line".into()),
			_ => unimplemented!()
		}
	}

	pub fn image<A: Into<Element>, B: Into<Element>>(
		a: A,
		b: B,
	) -> Result<Element, Box<dyn Error>> {
		match (a.into(), b.into()) {
			(Element::Point(a), Element::Point(b)) => {
				Ok(Point::new(2.0 * b.x - a.x, 2.0 * b.y - a.y).into())
			}
			(Element::Point(p), Element::Line(line)) => {
				let c = -2.0 * (line.slope * p.x - p.y + line.y_intercept)
					/ (line.slope.powf(2.0) + 1.0);

				Ok(Point::new(p.x + line.slope * c, p.y - line.slope * c).into())
			}
			(Element::Line(..), Element::Point(..)) => Err(
				"attempt to image of line from point, did you mean to image of point on line?"
					.into(),
			),
			(Element::Line(l1), Element::Line(l2)) => {
				if l1.slope != l2.slope {
					Err("attempt to drop perpendicular from line to line".into())
				} else {
					Ok(Line::new(l1.slope, l2.y_intercept * 2.0 - l1.y_intercept).into())
				}
			}
			_ => unimplemented!()
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Intersection {
	None,
	Points(Vec<(Point, Angle)>),
	Infinite,
}

impl Intersection {
	pub fn is_some(&self) -> bool {
		match self {
			Intersection::None => false,
			_ => true,
		}
	}

	pub fn is_none(&self) -> bool {
		match self {
			Intersection::None => true,
			_ => false,
		}
	}

	pub fn points(&self) -> Vec<Point> {
		match self {
			Intersection::Points(points) => points.iter().map(|f| f.0).collect::<Vec<Point>>(),
			t => panic!("attempt to extract values from {t:?}"),
		}
	}

	pub fn angles(&self) -> Vec<Angle> {
		match self {
			Intersection::Points(points) => points.iter().map(|f| f.1).collect::<Vec<Angle>>(),
			t => panic!("attempt to extract values from {t:?}"),
		}
	}

	pub fn values(&self) -> Vec<(Point, Angle)> {
		match self {
			Intersection::Points(points) => points.to_vec(),
			t => panic!("attempt to extract values from {t:?}"),
		}
	}
}

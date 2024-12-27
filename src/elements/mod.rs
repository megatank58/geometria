use circle::Circle;
use line::Line;
use point::Point;

pub mod line;
pub mod plane;
pub mod point;
pub mod circle;

#[derive(Debug, Clone, Copy)]
pub enum Element {
	Point(Point),
	Line(Line),
	Circle(Circle)
}

impl Element {
	pub fn to_point(&self) -> Point {
		match self {
			Element::Point(point) => *point,
			Element::Line(..) => panic!("expected to resolve as point, found line"),
			Element::Circle(..) => panic!("expected to resolve as point, found circle"),
		}
	}

	pub fn to_line(&self) -> Line {
		match self {
			Element::Point(..) => panic!("expected to resolve as line, found point"),
			Element::Line(line) => *line,
			Element::Circle(..) => panic!("expected to resolve as line, found circle"),
		}
	}

	pub fn to_circle(&self) -> Circle {
		match self {
			Element::Point(..) => panic!("expected to resolve as circle, found line"),
			Element::Line(..) => panic!("expected to resolve as circle, found line"),
			Element::Circle(circle) => *circle,
		}
	}
}

impl From<Point> for Element {
	fn from(value: Point) -> Self {
		Element::Point(value)
	}
}

impl From<Line> for Element {
	fn from(value: Line) -> Self {
		Element::Line(value)
	}
}

impl From<Circle> for Element {
	fn from(value: Circle) -> Self {
		Element::Circle(value)
	}
}

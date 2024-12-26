use line::Line;
use point::Point;

pub mod line;
pub mod plane;
pub mod point;

#[derive(Debug, Clone, Copy)]
pub enum Element {
	Point(Point),
	Line(Line),
}

impl Element {
	pub fn to_point(&self) -> Point {
		match self {
			Element::Point(point) => *point,
			Element::Line(..) => panic!("expected to resolve as point, found line"),
		}
	}

	pub fn to_line(&self) -> Line {
		match self {
			Element::Point(..) => panic!("expected to resolve as line, found point"),
			Element::Line(line) => *line,
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

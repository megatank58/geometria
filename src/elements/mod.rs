use line::Line;
use point::Point;

pub mod point;
pub mod line;
pub mod plane;

#[derive(Debug, Clone, Copy)]
pub enum Element {
    Point(Point),
    Line(Line)
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
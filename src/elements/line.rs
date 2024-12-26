use std::fmt::Display;

use crate::datatypes::radian::Radian;

use super::point::Point;
#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub slope: f64,
    pub y_intercept: f64,
}

impl Line {
    pub fn new(slope: f64, y_intercept: f64) -> Self {
        Self { slope, y_intercept }
    }

    pub fn from_points(a: Point, b: Point) -> Self {
        let slope = (a.y - b.y) / (a.x - b.x);

        let y_intercept = -slope * a.x + a.y;

        Self { slope, y_intercept }
    }

    pub fn from_angle(r: f64, angle: Radian, p: Point) -> Self {
        let x = r * angle.value.cos();
        let y = r * angle.value.sin();

        Line::from_points(p, Point::new(x, y))
    }

    pub fn angle(&self) -> Radian {
        Radian::new(self.slope.atan())
    }

    pub fn rotate(&self, angle: Radian, point: Point) -> Self {
        let p = self.y_intercept / (1.0 + self.slope.powf(2.0)).sqrt();

        let angle = self.angle() + angle;

        let new_p = p
            + point.x * (angle.cos() - self.angle().cos())
            + point.y * (angle.sin() - self.angle().sin());

        Line::new(angle.tan(), new_p / angle.sin())
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "y = {}x + {}", self.slope, self.y_intercept)
    }
}
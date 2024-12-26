use std::fmt::Display;

use crate::datatypes::radian::Radian;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn abs(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).powf(0.5)
    }

    pub fn angle(&self) -> Radian {
        Radian::new((self.y / self.x).atan())
    }

    pub fn rotate(&self, angle: Radian) -> Self {
        let abs = self.abs();

        let angle = self.angle() + angle;

        Point::new(abs * angle.cos(), abs * angle.sin())
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

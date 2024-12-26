use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Radian {
    pub value: f64,
}

impl Radian {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn cos(&self) -> f64 {
        self.value.cos()
    }

    pub fn sin(&self) -> f64 {
        self.value.sin()
    }

    pub fn tan(&self) -> f64 {
        self.value.tan()
    }
}

impl Add for Radian {
    type Output = Radian;

    fn add(self, rhs: Self) -> Self::Output {
        Radian::new(self.value + rhs.value)
    }
}

impl Sub for Radian {
    type Output = Radian;

    fn sub(self, rhs: Self) -> Self::Output {
        Radian::new(self.value - rhs.value)
    }
}

impl Mul<f64> for Radian {
    type Output = Radian;

    fn mul(self, rhs: f64) -> Self::Output {
        Radian::new(self.value * rhs)
    }
}

impl Mul<i64> for Radian {
    type Output = Radian;

    fn mul(self, rhs: i64) -> Self::Output {
        Radian::new(self.value * rhs as f64)
    }
}

impl Div<f64> for Radian {
    type Output = Radian;

    fn div(self, rhs: f64) -> Self::Output {
        Radian::new(self.value / rhs)
    }
}

impl Div<i64> for Radian {
    type Output = Radian;

    fn div(self, rhs: i64) -> Self::Output {
        Radian::new(self.value / rhs as f64)
    }
}

use std::ops::{AddAssign, Div, Mul};

mod fft;
mod dft;

#[derive(Default)]
pub struct Vec2 {
    x: f64,
    y: f64
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}


impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self {x, y}
    }
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn phase(&self) -> f64 {
        self.y.atan2(self.x)
    }
}


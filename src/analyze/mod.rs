use std::ops::{AddAssign, Div, Mul};

pub mod fft;
pub mod dft;

#[derive(Default)]
pub struct Vec2 {
    x: f32,
    y: f32
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
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
    pub fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn phase(&self) -> f32 {
        self.y.atan2(self.x)
    }
}


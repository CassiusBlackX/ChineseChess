use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vec2d {
    pub x: i8,
    pub y: i8,
}

impl Add for Vec2d {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

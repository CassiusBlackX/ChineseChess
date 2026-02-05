use super::board::{BOARD_HEIGHT, BOARD_WIDTH};
use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vec2d {
    pub x: i8,
    pub y: i8,
}

#[macro_export]
macro_rules! vec2d {
    ($x:expr, $y:expr) => {
        crate::vec2d::Vec2d { x: $x, y: $y }
    };
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

// There is no chance to use references!
// impl Add for &Vec2d {
//     type Output = Vec2d;
//     fn add(self, rhs: Self) -> Self::Output {
//         Vec2d {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }
//
// impl Add<&Vec2d> for Vec2d {
//     type Output = Self;
//     fn add(self, rhs: &Vec2d) -> Self::Output {
//         Self {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }

impl AddAssign for Vec2d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

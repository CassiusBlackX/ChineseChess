use crate::vec2d::Vec2d;

use super::board::{BOARD_HEIGHT, BOARD_WIDTH};
use std::{
    ops::{Add, AddAssign},
    usize,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr) => {
        crate::position::Position { x: $x, y: $y }
    };
}

impl Add<Vec2d> for Position {
    type Output = Self;
    fn add(self, rhs: Vec2d) -> Self::Output {
        let new_x = self.x + rhs.x as usize;
        let new_y = self.y + rhs.y as usize;
        assert!(new_x >= 0 && new_x < BOARD_WIDTH,);
        assert!(new_y >= 0 && new_y <= BOARD_HEIGHT);
        pos!(new_x, new_y)
    }
}

impl AddAssign<Vec2d> for Position {
    fn add_assign(&mut self, rhs: Vec2d) {
        let new_x = self.x + rhs.x as usize;
        let new_y = self.y + rhs.y as usize;
        assert!(new_x >= 0 && new_x < BOARD_WIDTH,);
        assert!(new_y >= 0 && new_y <= BOARD_HEIGHT);
        self.x = new_x;
        self.y = new_y;
    }
}

use std::collections::HashSet;

use crate::vec2d::Vec2d;

use super::board::{BOARD_HEIGHT, BOARD_WIDTH};
use std::{
    ops::{Add, AddAssign},
    usize,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
        let new_x = self.x as i8+ rhs.x ;
        let new_y = self.y as i8+ rhs.y ;
        assert!(new_x >= 0 && new_x < BOARD_WIDTH as i8,);
        assert!(new_y >= 0 && new_y <= BOARD_HEIGHT as i8);
        pos!(new_x as usize, new_y as usize)
    }
}

impl AddAssign<Vec2d> for Position {
    fn add_assign(&mut self, rhs: Vec2d) {
        let new_x = self.x as i8 + rhs.x;
        let new_y = self.y as i8 + rhs.y;
        assert!(new_x >= 0 && new_x < BOARD_WIDTH as i8,);
        assert!(new_y >= 0 && new_y <= BOARD_HEIGHT as i8);
        self.x = new_x as usize;
        self.y = new_y as usize;
    }
}

pub fn intersection<'a>(a: &'a[Position], b: &'a[Position]) -> Vec<&'a Position> {
   let (smaller, larger) = if a.len() <= b.len() {(a, b)} else {(b, a)};
   let set: HashSet<_> = smaller.iter().collect();
   larger.iter().filter(|x| set.contains(x)).collect()
}

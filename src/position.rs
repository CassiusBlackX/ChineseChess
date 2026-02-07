use std::collections::HashSet;

use crate::vec2d::Vec2d;

use super::board::{BOARD_HEIGHT, BOARD_WIDTH};
use std::{
    fmt::Display,
    ops::{Add, AddAssign},
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

impl Position {
    pub fn checked_add_vec2d(&self, direction: Vec2d) -> Option<Self> {
        let new_x = self.x as i8 + direction.x;
        let new_y = self.y as i8 + direction.y;
        if new_x < 0 || new_x >= BOARD_WIDTH as i8 {
            return None;
        }
        if new_y < 0 || new_y >= BOARD_HEIGHT as i8 {
            return None;
        }
        Some(*self + direction)
    }
}

impl Add<Vec2d> for Position {
    type Output = Self;
    fn add(self, rhs: Vec2d) -> Self::Output {
        let new_x = self.x as i8 + rhs.x;
        let new_y = self.y as i8 + rhs.y;
        assert!(
            new_x >= 0 && new_x < BOARD_WIDTH as i8,
            "new_x: {} out of range",
            new_x
        );
        assert!(
            new_y >= 0 && new_y <= BOARD_HEIGHT as i8,
            "new_y: {} out of range",
            new_y
        );
        pos!(new_x as usize, new_y as usize)
    }
}

impl AddAssign<Vec2d> for Position {
    fn add_assign(&mut self, rhs: Vec2d) {
        let new_x = self.x as i8 + rhs.x;
        let new_y = self.y as i8 + rhs.y;
        assert!(
            new_x >= 0 && new_x < BOARD_WIDTH as i8,
            "new_x: {} out of range",
            new_x
        );
        assert!(
            new_y >= 0 && new_y <= BOARD_HEIGHT as i8,
            "new_y: {} out of range",
            new_y
        );
        self.x = new_x as usize;
        self.y = new_y as usize;
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn intersection<'a>(a: &'a [Position], b: &'a [Position]) -> Vec<&'a Position> {
    let (smaller, larger) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    let set: HashSet<_> = smaller.iter().collect();
    larger.iter().filter(|x| set.contains(x)).collect()
}

pub fn intersection_option(a: &[Position], b: &[Option<Position>]) -> Vec<Position> {
    let set_a: HashSet<&Position> = a.iter().collect();
    b.iter()
        .filter_map(|opt| opt.as_ref())
        .filter(|&pos| set_a.contains(pos))
        .copied()
        .collect()
}

pub fn intersection_options(a: &[Option<Position>], b: &[Option<Position>]) -> Vec<Position> {
    let set_a: HashSet<&Position> = a.iter().filter_map(|opt| opt.as_ref()).collect();
    b.iter()
        .filter_map(|opt| opt.as_ref())
        .filter(|&pos| set_a.contains(pos))
        .copied()
        .collect()
}

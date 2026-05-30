use std::collections::HashSet;
use std::fmt::Display;
use std::ops::{Add, AddAssign};

use crate::vec2d::Vec2d;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn checked_add_vec2d(&self, direction: Vec2d, width: usize, height: usize) -> Option<Self> {
        let new_x = self.x as i8 + direction.x;
        let new_y = self.y as i8 + direction.y;
        if new_x < 0 || new_x >= width as i8 {
            return None;
        }
        if new_y < 0 || new_y >= height as i8 {
            return None;
        }
        Some(Self {
            x: new_x as usize,
            y: new_y as usize,
        })
    }
}

impl Add<Vec2d> for Position {
    type Output = Self;
    fn add(self, rhs: Vec2d) -> Self::Output {
        Self {
            x: (self.x as i8 + rhs.x) as usize,
            y: (self.y as i8 + rhs.y) as usize,
        }
    }
}

impl AddAssign<Vec2d> for Position {
    fn add_assign(&mut self, rhs: Vec2d) {
        self.x = (self.x as i8 + rhs.x) as usize;
        self.y = (self.y as i8 + rhs.y) as usize;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2d::Vec2d;

    #[test]
    fn checked_add_respects_bounds() {
        let pos = Position { x: 4, y: 5 };
        assert!(pos.checked_add_vec2d(Vec2d { x: 1, y: 0 }, 9, 10).is_some());
        assert!(pos.checked_add_vec2d(Vec2d { x: 5, y: 0 }, 9, 10).is_none());
    }
}

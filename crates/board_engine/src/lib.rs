pub mod grid;
pub mod position;
pub mod vec2d;

pub use grid::Grid;
pub use position::Position;
pub use vec2d::Vec2d;

pub type Player = i8;

#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr) => {
        $crate::Position { x: $x, y: $y }
    };
}

#[macro_export]
macro_rules! vec2d {
    ($x:expr, $y:expr) => {
        $crate::Vec2d { x: $x, y: $y }
    };
}

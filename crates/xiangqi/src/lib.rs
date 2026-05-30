pub mod adapter;
pub mod board;
pub mod chess;
pub mod game;

#[cfg(target_arch = "wasm32")]
mod wasm_api;

pub use adapter::XiangqiAdapter;
pub use board::{Board, WalkErr, BOARD_HEIGHT, BOARD_WIDTH};
pub use game::Game;

pub use board_engine::{pos, vec2d, Grid, Player, Position, Vec2d};

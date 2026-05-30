pub mod adapter;
pub mod board;
pub mod game;
pub mod win;

#[cfg(target_arch = "wasm32")]
mod wasm_api;

pub use adapter::GomokuAdapter;
pub use board::{Cell, BOARD_HEIGHT, BOARD_WIDTH};
pub use game::Game;

pub use board_engine::{pos, Grid, Player, Position};

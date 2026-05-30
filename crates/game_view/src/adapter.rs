use board_engine::Player;

use crate::dto::{CoordDto, SnapshotDto};

pub enum ViewInput {
    Snapshot,
    Reset,
    Click { x: usize, y: usize },
    TryMove {
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
    },
    LegalMoves { x: usize, y: usize },
}

pub enum ViewOutput {
    Snapshot(SnapshotDto),
    Moves(Vec<CoordDto>),
    Error(String),
}

pub trait GameViewAdapter {
    fn handle(&mut self, input: ViewInput) -> ViewOutput;
    fn board_width(&self) -> usize;
    fn board_height(&self) -> usize;
    fn current_turn(&self) -> Player;
    fn game_title(&self) -> &str;
}

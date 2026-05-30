use board_engine::Player;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CoordDto {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct PieceDto {
    pub id: i8,
    pub x: usize,
    pub y: usize,
    pub side: Player,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SnapshotDto {
    pub width: usize,
    pub height: usize,
    pub turn: Player,
    pub selected: Option<CoordDto>,
    pub legal_moves: Vec<CoordDto>,
    pub pieces: Vec<PieceDto>,
    pub game_over: bool,
    pub winner: Player,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_check_side: Option<Player>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_move: Option<CoordDto>,
}

// Backward-compatible alias used by existing code paths.
pub type MoveDto = CoordDto;

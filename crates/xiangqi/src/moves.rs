use board_engine::Position;

use crate::{
    board::Board,
    rules::{apply_move_on_board, is_move_safe, pseudo_moves_for_piece},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub piece_id: i8,
    pub from: Position,
    pub to: Position,
}

pub fn all_legal_moves(board: &mut Board, side: i8) -> Vec<Move> {
    let mut pieces = Vec::new();
    for (x, y) in board.board_status().iter_coords() {
        let id = board.board_status().get(x, y).unwrap_or(0);
        if id != 0 && id.signum() == side {
            pieces.push((id, Position { x, y }));
        }
    }

    let mut moves = Vec::new();
    for (id, from) in pieces {
        for to in pseudo_moves_for_piece(board, id) {
            if is_move_safe(board, id, from, to) {
                moves.push(Move {
                    piece_id: id,
                    from,
                    to,
                });
            }
        }
    }

    moves.sort_by_key(|mv| (mv.piece_id, mv.from.x, mv.from.y, mv.to.x, mv.to.y));
    moves
}

pub fn apply_move(board: &mut Board, mv: Move) -> bool {
    apply_move_on_board(board, mv.piece_id, mv.from, mv.to)
}

pub fn is_checkmate_on_board(board: &Board, side: i8) -> bool {
    let mut probe = board.clone();
    if !crate::rules::is_side_in_check(&mut probe, side) {
        return false;
    }
    let mut search = board.clone();
    all_legal_moves(&mut search, side).is_empty()
}

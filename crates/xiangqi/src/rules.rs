use board_engine::{Position, Vec2d};

use crate::{
    board::Board,
    chess::{BLACK_KING_ID, RED_KING_ID},
};

pub fn find_king_pos(board: &Board, side: i8) -> Option<Position> {
    let king_id = if side > 0 { RED_KING_ID } else { BLACK_KING_ID };
    let king = board.get_piece(king_id)?;
    if !king.is_alive() {
        return None;
    }
    Some(king.get_pos())
}

pub fn kings_face_each_other(board: &Board) -> bool {
    let Some(red_king_pos) = find_king_pos(board, 1) else {
        return false;
    };
    let Some(black_king_pos) = find_king_pos(board, -1) else {
        return false;
    };

    if red_king_pos.x != black_king_pos.x {
        return false;
    }

    let x = red_king_pos.x;
    let min_y = red_king_pos.y.min(black_king_pos.y);
    let max_y = red_king_pos.y.max(black_king_pos.y);
    for y in (min_y + 1)..max_y {
        if board.board_status().get(x, y).unwrap_or(0) != 0 {
            return false;
        }
    }
    true
}

pub fn pseudo_moves_for_piece(board: &mut Board, id: i8) -> Vec<Position> {
    board
        .walk_options(id)
        .iter()
        .filter_map(|opt| *opt)
        .collect()
}

pub fn apply_move_on_board(board: &mut Board, id: i8, from: Position, to: Position) -> bool {
    let direction = Vec2d {
        x: to.x as i8 - from.x as i8,
        y: to.y as i8 - from.y as i8,
    };
    board.walk(id, direction).is_ok()
}

pub fn is_move_safe(board: &Board, id: i8, from: Position, to: Position) -> bool {
    let mut simulated = board.clone();
    if !apply_move_on_board(&mut simulated, id, from, to) {
        return false;
    }
    !is_side_in_check(&mut simulated, id.signum())
}

pub fn is_side_in_check(board: &mut Board, side: i8) -> bool {
    if kings_face_each_other(board) {
        return true;
    }

    let Some(king_pos) = find_king_pos(board, side) else {
        return true;
    };

    let mut enemy_ids = Vec::new();
    for (x, y) in board.board_status().iter_coords() {
        let id = board.board_status().get(x, y).unwrap_or(0);
        if id != 0 && id.signum() == -side {
            enemy_ids.push(id);
        }
    }

    for enemy_id in enemy_ids {
        let moves = pseudo_moves_for_piece(board, enemy_id);
        if moves.iter().any(|m| m.x == king_pos.x && m.y == king_pos.y) {
            return true;
        }
    }

    false
}

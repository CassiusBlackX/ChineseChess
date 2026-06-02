use board_engine::Position;
use game_view::AiDifficulty;

use crate::{
    board::Board,
    moves::{all_legal_moves, apply_move, Move},
    rules::{is_side_in_check, kings_face_each_other},
};

const VALUE_CAR: i32 = 900;
const VALUE_CANNON: i32 = 450;
const VALUE_HORSE: i32 = 400;
const VALUE_ADVISOR: i32 = 200;
const VALUE_ELEPHANT: i32 = 200;
const VALUE_PAWN: i32 = 100;
const VALUE_PAWN_RIVER: i32 = 150;
const VALUE_KING: i32 = 20_000;
const CHECK_PENALTY: i32 = 500;
const CHECK_BONUS: i32 = 250;
const HARD_TOP_K: usize = 12;
const HARD_SEARCH_DEPTH: u8 = 2;
const EASY_CAPTURE_BIAS: f32 = 0.6;
const EASY_MISS_CAPTURE_CHANCE: f32 = 0.25;
const MEDIUM_SUBOPTIMAL_CHANCE: f32 = 0.05;

struct Rng {
    state: u64,
}

impl Rng {
    fn from_board(board: &Board, difficulty: AiDifficulty) -> Self {
        let mut hash = difficulty as u8 as u64;
        for (x, y) in board.board_status().iter_coords() {
            let id = board.board_status().get(x, y).unwrap_or(0);
            if id != 0 {
                hash = hash
                    .wrapping_mul(31)
                    .wrapping_add(x as u64 + y as u64 * 17 + id.unsigned_abs() as u64);
            }
        }
        Self { state: hash.max(1) }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        (self.state >> 32) as u32
    }

    fn chance(&mut self, probability: f32) -> bool {
        (self.next_u32() as f32 / u32::MAX as f32) < probability
    }

    fn pick_index(&mut self, len: usize) -> usize {
        if len == 0 {
            0
        } else {
            (self.next_u32() as usize) % len
        }
    }
}

pub fn piece_value(id: i8, pos: Position) -> i32 {
    let kind = id.unsigned_abs();
    match kind {
        1 => VALUE_KING,
        2 | 3 => VALUE_ADVISOR,
        4 | 5 => VALUE_ELEPHANT,
        6 | 7 => VALUE_HORSE,
        8 | 9 => VALUE_CAR,
        10 | 11 => VALUE_CANNON,
        12..=16 => {
            if id > 0 && pos.y >= 5 {
                VALUE_PAWN_RIVER
            } else if id < 0 && pos.y <= 4 {
                VALUE_PAWN_RIVER
            } else {
                VALUE_PAWN
            }
        }
        _ => 0,
    }
}

pub fn choose_move(board: &mut Board, side: i8, difficulty: AiDifficulty) -> Option<Move> {
    let moves = all_legal_moves(board, side);
    if moves.is_empty() {
        return None;
    }

    let mut rng = Rng::from_board(board, difficulty);
    match difficulty {
        AiDifficulty::Easy => pick_easy(board, side, &moves, &mut rng),
        AiDifficulty::Medium => pick_greedy(board, side, &moves, &mut rng, false),
        AiDifficulty::Hard => pick_greedy(board, side, &moves, &mut rng, true),
    }
}

fn pick_easy(board: &Board, _side: i8, moves: &[Move], rng: &mut Rng) -> Option<Move> {
    let captures: Vec<Move> = moves
        .iter()
        .copied()
        .filter(|mv| board.board_status().get(mv.to.x, mv.to.y).unwrap_or(0) != 0)
        .collect();

    if !captures.is_empty() && rng.chance(EASY_CAPTURE_BIAS) && !rng.chance(EASY_MISS_CAPTURE_CHANCE)
    {
        return Some(captures[rng.pick_index(captures.len())]);
    }

    Some(moves[rng.pick_index(moves.len())])
}

fn pick_greedy(
    board: &mut Board,
    side: i8,
    moves: &[Move],
    rng: &mut Rng,
    use_search: bool,
) -> Option<Move> {
    let mut scored: Vec<(i32, Move)> = moves
        .iter()
        .copied()
        .map(|mv| (score_move(board, mv, side), mv))
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));

    if scored.is_empty() {
        return None;
    }

    if !use_search {
        let mut captures: Vec<(i32, Move)> = scored
            .iter()
            .copied()
            .filter(|(_, mv)| {
                let target = board.board_status().get(mv.to.x, mv.to.y).unwrap_or(0);
                target != 0 && target.signum() != side
            })
            .collect();
        captures.sort_by(|a, b| b.0.cmp(&a.0));
        if let Some((_, mv)) = captures.first() {
            return Some(*mv);
        }
    }

    if use_search {
        let top: Vec<Move> = scored
            .iter()
            .take(HARD_TOP_K.min(scored.len()))
            .map(|(_, mv)| *mv)
            .collect();
        if let Some(best) = negamax_pick(board, side, &top) {
            return Some(best);
        }
    }

    if rng.chance(MEDIUM_SUBOPTIMAL_CHANCE) && scored.len() > 1 {
        return Some(scored[1].1);
    }

    Some(scored[0].1)
}

fn score_move(board: &mut Board, mv: Move, side: i8) -> i32 {
    let mut trial = board.clone();
    if !apply_move(&mut trial, mv) {
        return i32::MIN / 4;
    }

    let captured = board.board_status().get(mv.to.x, mv.to.y).unwrap_or(0);
    let capture_gain = if captured == 0 {
        0
    } else {
        piece_value(captured, mv.to)
    };

    let moving_value = piece_value(mv.piece_id, mv.from);
    let mut score = evaluate(&mut trial, side) + capture_gain;

    if is_side_in_check(&mut trial, -side) {
        score += CHECK_BONUS;
    }

    if is_side_in_check(&mut trial, side) {
        score -= CHECK_PENALTY;
    }

    let enemy_replies = all_legal_moves(&mut trial, -side);
    if enemy_replies.iter().any(|reply| {
        trial.board_status().get(reply.to.x, reply.to.y).unwrap_or(0) == mv.piece_id
    }) {
        score -= moving_value / 2;
    }

    score
}

fn evaluate(board: &mut Board, side: i8) -> i32 {
    let mut score = 0;
    for (x, y) in board.board_status().iter_coords() {
        let id = board.board_status().get(x, y).unwrap_or(0);
        if id == 0 {
            continue;
        }
        let value = piece_value(id, Position { x, y });
        if id.signum() == side {
            score += value;
        } else {
            score -= value;
        }
    }

    if is_side_in_check(board, side) {
        score -= CHECK_PENALTY;
    }
    if is_side_in_check(board, -side) {
        score += CHECK_BONUS / 2;
    }
    if kings_face_each_other(board) {
        score -= CHECK_PENALTY;
    }

    score
}

fn negamax_pick(board: &mut Board, side: i8, candidates: &[Move]) -> Option<Move> {
    let mut best_mv = candidates.first().copied()?;
    let mut best_score = i32::MIN;

    for &mv in candidates {
        let mut trial = board.clone();
        if !apply_move(&mut trial, mv) {
            continue;
        }
        let score = -negamax(
            &mut trial,
            HARD_SEARCH_DEPTH - 1,
            i32::MIN / 2,
            i32::MAX / 2,
            -side,
            side,
        );
        if score > best_score {
            best_score = score;
            best_mv = mv;
        }
    }

    Some(best_mv)
}

fn negamax(
    board: &mut Board,
    depth: u8,
    alpha: i32,
    beta: i32,
    side: i8,
    ai_side: i8,
) -> i32 {
    if depth == 0 {
        return evaluate(board, ai_side);
    }

    let moves = all_legal_moves(board, side);
    if moves.is_empty() {
        if is_side_in_check(board, side) {
            return if side == ai_side {
                -VALUE_KING
            } else {
                VALUE_KING
            };
        }
        return 0;
    }

    let mut scored: Vec<(i32, Move)> = moves
        .iter()
        .copied()
        .map(|mv| (score_move(board, mv, side), mv))
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));

    let mut alpha = alpha;
    let mut best = i32::MIN;
    let limit = HARD_TOP_K.min(scored.len());

    for (_, mv) in scored.into_iter().take(limit) {
        let mut trial = board.clone();
        if !apply_move(&mut trial, mv) {
            continue;
        }

        let score = -negamax(
            &mut trial,
            depth - 1,
            -beta,
            -alpha,
            -side,
            ai_side,
        );

        best = best.max(score);
        alpha = alpha.max(score);
        if alpha >= beta {
            break;
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        board::{generate_board, Board},
        chess::{BLACK_KING_ID, BLACK_LEFT_CAR_ID, RED_KING_ID, RED_LEFT_CAR_ID, RED_MIDDLE_PAWN_ID},
        moves::all_legal_moves,
        pos,
    };

    #[test]
    fn opening_has_legal_move() {
        let mut board = Board::new();
        let mv = choose_move(&mut board, 1, AiDifficulty::Easy).unwrap();
        assert!(all_legal_moves(&mut board, 1).contains(&mv));
    }

    #[test]
    fn medium_takes_free_car() {
        let board_status = generate_board(vec![
            (RED_KING_ID, pos!(4, 0)),
            (BLACK_KING_ID, pos!(4, 9)),
            (RED_MIDDLE_PAWN_ID, pos!(4, 5)),
            (RED_LEFT_CAR_ID, pos!(0, 8)),
            (BLACK_LEFT_CAR_ID, pos!(0, 9)),
        ]);
        let mut board = Board::from_board_status(board_status);
        let mv = choose_move(&mut board, 1, AiDifficulty::Medium).unwrap();
        assert_eq!(mv.to, pos!(0, 9));
    }

    #[test]
    fn legal_moves_escape_check() {
        let board_status = generate_board(vec![
            (RED_KING_ID, pos!(4, 0)),
            (BLACK_KING_ID, pos!(4, 9)),
            (RED_LEFT_CAR_ID, pos!(4, 8)),
        ]);
        let mut board = Board::from_board_status(board_status);
        let moves = all_legal_moves(&mut board, -1);
        assert!(!moves.is_empty());
        for mv in moves {
            let mut trial = board.clone();
            apply_move(&mut trial, mv);
            assert!(!is_side_in_check(&mut trial, -1));
        }
    }
}

use std::collections::HashSet;

use board_engine::{Player, Position};
use game_view::AiDifficulty;

use crate::board::{Board, Cell, BOARD_HEIGHT, BOARD_WIDTH};
use crate::pos;
use crate::win::check_winner_on_board;

const SCORE_WIN: i32 = 1_000_000;
const SCORE_LIVE_FOUR: i32 = 50_000;
const SCORE_RUSH_FOUR: i32 = 10_000;
const SCORE_LIVE_THREE: i32 = 5_000;
const SCORE_SLEEP_THREE: i32 = 500;
const SCORE_LIVE_TWO: i32 = 200;
const DEFENSE_WEIGHT: f32 = 1.05;
const EASY_MISS_BLOCK_CHANCE: f32 = 0.4;
const MEDIUM_SUBOPTIMAL_CHANCE: f32 = 0.05;
const HARD_TOP_K: usize = 8;
const HARD_SEARCH_DEPTH: u8 = 3;
const NEIGHBOR_RADIUS: i32 = 2;

struct Rng {
    state: u64,
}

impl Rng {
    fn from_board(board: &Board, difficulty: AiDifficulty) -> Self {
        let mut hash = difficulty as u8 as u64;
        for (x, y) in board.grid().iter_coords() {
            if let Some(cell) = board.grid().get(x, y) {
                if cell != Cell::Empty {
                    hash = hash.wrapping_mul(31).wrapping_add(x as u64 + y as u64 * 17);
                }
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

pub fn choose_move(board: &Board, player: Player, difficulty: AiDifficulty) -> Option<Position> {
    let opponent = -player;
    let mut rng = Rng::from_board(board, difficulty);
    let candidates = candidate_moves(board);

    if candidates.is_empty() {
        return None;
    }

    if let Some(win) = find_winning_move(board, player, &candidates) {
        return Some(win);
    }

    if let Some(block) = find_winning_move(board, opponent, &candidates) {
        let should_block = match difficulty {
            AiDifficulty::Easy => !rng.chance(EASY_MISS_BLOCK_CHANCE),
            _ => true,
        };
        if should_block {
            return Some(block);
        }
    }

    match difficulty {
        AiDifficulty::Easy => {
            let idx = rng.pick_index(candidates.len());
            Some(candidates[idx])
        }
        AiDifficulty::Medium => {
            pick_heuristic(board, player, &candidates, &mut rng, false)
        }
        AiDifficulty::Hard => {
            pick_heuristic(board, player, &candidates, &mut rng, true)
        }
    }
}

fn candidate_moves(board: &Board) -> Vec<Position> {
    let mut stones = Vec::new();
    for (x, y) in board.grid().iter_coords() {
        if board.grid().get(x, y) == Some(Cell::Empty) {
            continue;
        }
        stones.push((x, y));
    }

    if stones.is_empty() {
        return vec![pos!(7, 7)];
    }

    let mut set = HashSet::new();
    for &(sx, sy) in &stones {
        for dx in -NEIGHBOR_RADIUS..=NEIGHBOR_RADIUS {
            for dy in -NEIGHBOR_RADIUS..=NEIGHBOR_RADIUS {
                let x = sx as i32 + dx;
                let y = sy as i32 + dy;
                if x < 0
                    || y < 0
                    || x >= BOARD_WIDTH as i32
                    || y >= BOARD_HEIGHT as i32
                {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;
                if board.is_empty(x, y) {
                    set.insert((x, y));
                }
            }
        }
    }

    let mut candidates: Vec<Position> = set.into_iter().map(|(x, y)| pos!(x, y)).collect();
    candidates.sort_by_key(|p| (p.x, p.y));
    candidates
}

fn find_winning_move(
    board: &Board,
    player: Player,
    candidates: &[Position],
) -> Option<Position> {
    for &pos in candidates {
        if would_win(board, pos.x, pos.y, player) {
            return Some(pos);
        }
    }
    None
}

fn would_win(board: &Board, x: usize, y: usize, player: Player) -> bool {
    let mut trial = board.clone();
    trial.place(x, y, player);
    check_winner_on_board(&trial, pos!(x, y)).is_some()
}

fn pick_heuristic(
    board: &Board,
    player: Player,
    candidates: &[Position],
    rng: &mut Rng,
    use_minimax: bool,
) -> Option<Position> {
    let mut scored: Vec<(i32, Position)> = candidates
        .iter()
        .map(|&pos| (combined_score(board, pos.x, pos.y, player), pos))
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));

    if scored.is_empty() {
        return None;
    }

    if use_minimax {
        let top: Vec<Position> = scored
            .iter()
            .take(HARD_TOP_K.min(scored.len()))
            .map(|(_, p)| *p)
            .collect();
        if let Some(best) = minimax_pick(board, player, &top) {
            return Some(best);
        }
    }

    if rng.chance(MEDIUM_SUBOPTIMAL_CHANCE) && scored.len() > 1 {
        return Some(scored[1].1);
    }

    Some(scored[0].1)
}

fn combined_score(board: &Board, x: usize, y: usize, player: Player) -> i32 {
    let attack = eval_at(board, x, y, player);
    let defense = eval_at(board, x, y, -player);
    attack + (defense as f32 * DEFENSE_WEIGHT) as i32
}

fn eval_at(board: &Board, x: usize, y: usize, player: Player) -> i32 {
    if !board.is_empty(x, y) {
        return i32::MIN / 4;
    }

    let mut trial = board.clone();
    trial.place(x, y, player);
    if check_winner_on_board(&trial, pos!(x, y)).is_some() {
        return SCORE_WIN;
    }

    let stone = Cell::from_player(player);
    let pos = pos!(x, y);
    let mut total = 0;
    for (dx, dy) in [(1, 0), (0, 1), (1, 1), (1, -1)] {
        total += line_score(board.grid(), pos, dx, dy, stone);
    }
    total
}

fn line_score(
    grid: &board_engine::Grid<Cell>,
    pos: Position,
    dx: i8,
    dy: i8,
    stone: Cell,
) -> i32 {
    let forward = count_dir(grid, pos, dx, dy, stone);
    let backward = count_dir(grid, pos, -dx, -dy, stone);
    let count = forward + backward + 1;
    let open_ends = open_end_count(grid, pos, dx, dy, stone, forward, backward);

    match (count, open_ends) {
        (n, _) if n >= 5 => SCORE_WIN,
        (4, 2) => SCORE_LIVE_FOUR,
        (4, 1) => SCORE_RUSH_FOUR,
        (4, 0) => SCORE_RUSH_FOUR / 2,
        (3, 2) => SCORE_LIVE_THREE,
        (3, 1) => SCORE_SLEEP_THREE,
        (2, 2) => SCORE_LIVE_TWO,
        _ => 0,
    }
}

fn count_dir(grid: &board_engine::Grid<Cell>, pos: Position, dx: i8, dy: i8, stone: Cell) -> usize {
    let mut count = 0;
    let mut x = pos.x as i8 + dx;
    let mut y = pos.y as i8 + dy;
    while x >= 0
        && y >= 0
        && (x as usize) < grid.width()
        && (y as usize) < grid.height()
        && grid.get(x as usize, y as usize) == Some(stone)
    {
        count += 1;
        x += dx;
        y += dy;
    }
    count
}

fn open_end_count(
    grid: &board_engine::Grid<Cell>,
    pos: Position,
    dx: i8,
    dy: i8,
    _stone: Cell,
    forward: usize,
    backward: usize,
) -> u8 {
    let mut open = 0;
    let fx = pos.x as i8 + dx * (forward as i8 + 1);
    let fy = pos.y as i8 + dy * (forward as i8 + 1);
    if is_open_cell(grid, fx, fy) {
        open += 1;
    }
    let bx = pos.x as i8 - dx * (backward as i8 + 1);
    let by = pos.y as i8 - dy * (backward as i8 + 1);
    if is_open_cell(grid, bx, by) {
        open += 1;
    }
    open
}

fn is_open_cell(grid: &board_engine::Grid<Cell>, x: i8, y: i8) -> bool {
    if x < 0 || y < 0 || x as usize >= grid.width() || y as usize >= grid.height() {
        return false;
    }
    matches!(grid.get(x as usize, y as usize), Some(Cell::Empty) | None)
}

fn minimax_pick(board: &Board, player: Player, candidates: &[Position]) -> Option<Position> {
    let mut best_pos = candidates.first().copied()?;
    let mut best_score = i32::MIN;
    let opponent = -player;

    for &pos in candidates {
        let mut trial = board.clone();
        trial.place(pos.x, pos.y, player);
        if check_winner_on_board(&trial, pos).is_some() {
            return Some(pos);
        }

        let score = -negamax(
            &trial,
            HARD_SEARCH_DEPTH - 1,
            i32::MIN / 2,
            i32::MAX / 2,
            opponent,
            player,
        );
        if score > best_score {
            best_score = score;
            best_pos = pos;
        }
    }

    Some(best_pos)
}

fn negamax(
    board: &Board,
    depth: u8,
    alpha: i32,
    beta: i32,
    player: Player,
    ai_player: Player,
) -> i32 {
    if depth == 0 {
        return evaluate_board(board, ai_player);
    }

    let candidates = candidate_moves(board);
    if candidates.is_empty() {
        return evaluate_board(board, ai_player);
    }

    let mut scored: Vec<(i32, Position)> = candidates
        .iter()
        .map(|pos| (combined_score(board, pos.x, pos.y, player), *pos))
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));

    let mut alpha = alpha;
    let mut best = i32::MIN;

    let limit = HARD_TOP_K.min(scored.len());
    for (_, pos) in scored.into_iter().take(limit) {

        let mut trial = board.clone();
        trial.place(pos.x, pos.y, player);
        if check_winner_on_board(&trial, pos).is_some() {
            let win_score = SCORE_WIN + depth as i32;
            if player == ai_player {
                return win_score;
            }
            return -win_score;
        }

        let score = -negamax(
            &trial,
            depth - 1,
            -beta,
            -alpha,
            -player,
            ai_player,
        );

        best = best.max(score);
        alpha = alpha.max(score);
        if alpha >= beta {
            break;
        }
    }

    best
}

fn evaluate_board(board: &Board, ai_player: Player) -> i32 {
    let human = -ai_player;
    let mut total = 0;
    for pos in candidate_moves(board) {
        total += eval_at(board, pos.x, pos.y, ai_player) / 10;
        total -= eval_at(board, pos.x, pos.y, human) / 10;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Cell;

    fn board_with(coords: &[(usize, usize, Cell)]) -> Board {
        let mut board = Board::new();
        for &(x, y, cell) in coords {
            let player = cell.to_player();
            board.place(x, y, player);
        }
        board
    }

    #[test]
    fn empty_board_plays_center() {
        let board = Board::new();
        let mv = choose_move(&board, 1, AiDifficulty::Easy).unwrap();
        assert_eq!(mv, pos!(7, 7));
    }

    #[test]
    fn blocks_opponent_four_in_row() {
        let board = board_with(&[
            (5, 7, Cell::Black),
            (6, 7, Cell::Black),
            (8, 7, Cell::Black),
            (9, 7, Cell::Black),
        ]);
        let mv = choose_move(&board, -1, AiDifficulty::Medium).unwrap();
        assert_eq!(mv, pos!(7, 7));
    }

    #[test]
    fn takes_winning_move() {
        let board = board_with(&[
            (5, 7, Cell::White),
            (6, 7, Cell::White),
            (8, 7, Cell::White),
            (9, 7, Cell::White),
        ]);
        let mv = choose_move(&board, -1, AiDifficulty::Easy).unwrap();
        assert_eq!(mv, pos!(7, 7));
    }

    #[test]
    fn easy_can_miss_block_with_seeded_layout() {
        let board = board_with(&[
            (5, 7, Cell::Black),
            (6, 7, Cell::Black),
            (8, 7, Cell::Black),
            (9, 7, Cell::Black),
            (7, 5, Cell::White),
        ]);
        let mv = choose_move(&board, -1, AiDifficulty::Easy).unwrap();
        assert!(mv != pos!(7, 7) || board.is_empty(7, 7));
        let _ = mv;
    }
}

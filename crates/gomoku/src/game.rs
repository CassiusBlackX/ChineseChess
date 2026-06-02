use game_view::{AiDifficulty, CoordDto, PieceDto, PlayMode, SessionDto, SnapshotDto};

use board_engine::{Player, Position};

use crate::{
    ai,
    board::{Board, BOARD_HEIGHT, BOARD_WIDTH, Cell},
    pos,
    win::check_winner_on_board,
};

#[derive(Clone)]
pub struct Game {
    board: Board,
    turn: Player,
    game_over: bool,
    winner: Player,
    message: String,
    last_move: Option<Position>,
    play_mode: PlayMode,
    ai_difficulty: AiDifficulty,
    human_side: Player,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn: 1,
            game_over: false,
            winner: 0,
            message: "黑方先手".to_string(),
            last_move: None,
            play_mode: PlayMode::LocalPvp,
            ai_difficulty: AiDifficulty::Medium,
            human_side: 1,
        }
    }

    pub fn reset(&mut self) {
        self.board = Board::new();
        self.turn = 1;
        self.game_over = false;
        self.winner = 0;
        self.message = "黑方先手".to_string();
        self.last_move = None;

        if self.needs_ai_move() {
            self.ai_move();
        }
    }

    pub fn set_play_mode(&mut self, play_mode: PlayMode) {
        self.play_mode = play_mode;
        self.reset();
    }

    pub fn set_ai_difficulty(&mut self, ai_difficulty: AiDifficulty) {
        self.ai_difficulty = ai_difficulty;
        self.reset();
    }

    pub fn set_human_side(&mut self, human_side: Player) {
        if human_side == 1 || human_side == -1 {
            self.human_side = human_side;
            self.reset();
        }
    }

    pub fn board_width(&self) -> usize {
        BOARD_WIDTH
    }

    pub fn board_height(&self) -> usize {
        BOARD_HEIGHT
    }

    pub fn current_turn(&self) -> Player {
        self.turn
    }

    pub fn play_mode(&self) -> PlayMode {
        self.play_mode
    }

    pub fn ai_difficulty(&self) -> AiDifficulty {
        self.ai_difficulty
    }

    pub fn human_side(&self) -> Player {
        self.human_side
    }

    fn human_input_enabled(&self) -> bool {
        !self.game_over
            && (self.play_mode == PlayMode::LocalPvp || self.turn == self.human_side)
    }

    fn needs_ai_move(&self) -> bool {
        self.play_mode == PlayMode::HumanVsAi
            && !self.game_over
            && self.turn != self.human_side
    }

    pub fn snapshot(&self) -> SnapshotDto {
        let human_input_enabled = self.human_input_enabled();
        let legal_moves = if self.game_over || !human_input_enabled {
            Vec::new()
        } else {
            self.board
                .grid()
                .iter_coords()
                .filter(|&(x, y)| self.board.is_empty(x, y))
                .map(|(x, y)| CoordDto { x, y })
                .collect()
        };

        SnapshotDto {
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
            turn: self.turn,
            selected: None,
            legal_moves,
            pieces: self.collect_pieces(),
            in_check_side: None,
            game_over: self.game_over,
            winner: self.winner,
            message: self.message.clone(),
            last_move: self.last_move.map(|p| CoordDto { x: p.x, y: p.y }),
            session: Some(SessionDto {
                play_mode: self.play_mode,
                ai_difficulty: self.ai_difficulty,
                human_side: self.human_side,
                human_input_enabled,
            }),
        }
    }

    pub fn click(&mut self, x: usize, y: usize) -> SnapshotDto {
        if x >= BOARD_WIDTH || y >= BOARD_HEIGHT {
            self.message = "坐标越界".to_string();
            return self.snapshot();
        }

        if self.game_over {
            self.message = "对局已结束，请重开一局".to_string();
            return self.snapshot();
        }

        if self.play_mode == PlayMode::HumanVsAi && self.turn != self.human_side {
            self.message = "轮到 AI 落子".to_string();
            return self.snapshot();
        }

        if !self.board.is_empty(x, y) {
            self.message = "该位置已有棋子".to_string();
            return self.snapshot();
        }

        self.place_at(x, y, self.turn);
        self.snapshot()
    }

    pub fn human_click(&mut self, x: usize, y: usize) -> SnapshotDto {
        let before_over = self.game_over;
        let snap = self.click(x, y);
        if !before_over && !self.game_over && self.needs_ai_move() {
            self.ai_move();
            return self.snapshot();
        }
        snap
    }

    pub fn ai_move(&mut self) {
        if !self.needs_ai_move() {
            return;
        }

        let Some(pos) = ai::choose_move(&self.board, self.turn, self.ai_difficulty) else {
            self.message = "AI 无法落子".to_string();
            return;
        };

        self.place_at(pos.x, pos.y, self.turn);
    }

    fn place_at(&mut self, x: usize, y: usize, side: Player) {
        self.board.place(x, y, side);
        let placed = pos!(x, y);
        self.last_move = Some(placed);

        if let Some(winner) = check_winner_on_board(&self.board, placed) {
            self.game_over = true;
            self.winner = winner;
            self.message = format!("{}方五连，获胜！", side_name(winner));
        } else if self.board.is_full() {
            self.game_over = true;
            self.winner = 0;
            self.message = "棋盘已满，和棋".to_string();
        } else {
            self.turn = -self.turn;
            self.message = format!("{}方落子", side_name(-self.turn));
        }
    }

    fn collect_pieces(&self) -> Vec<PieceDto> {
        let mut pieces = Vec::new();
        for (x, y) in self.board.grid().iter_coords() {
            let cell = self.board.grid().get(x, y).unwrap_or(Cell::Empty);
            if cell == Cell::Empty {
                continue;
            }
            let side = cell.to_player();
            let symbol = if side > 0 { "●" } else { "○" }.to_string();
            pieces.push(PieceDto {
                id: side,
                x,
                y,
                side,
                symbol,
            });
        }
        pieces
    }
}

fn side_name(side: Player) -> &'static str {
    if side > 0 { "黑" } else { "白" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn places_stone_and_switches_turn() {
        let mut game = Game::new();
        let snap = game.click(7, 7);
        assert_eq!(snap.turn, -1);
        assert_eq!(snap.pieces.len(), 1);
    }

    #[test]
    fn rejects_occupied_cell() {
        let mut game = Game::new();
        game.click(7, 7);
        let snap = game.click(7, 7);
        assert_eq!(snap.message, "该位置已有棋子");
        assert_eq!(snap.pieces.len(), 1);
    }

    #[test]
    fn game_ends_on_five_in_row() {
        let mut game = Game::new();
        game.click(7, 7);
        game.click(8, 7);
        game.click(7, 8);
        game.click(8, 8);
        game.click(7, 9);
        game.click(9, 7);
        game.click(7, 10);
        game.click(9, 8);
        let snap = game.click(7, 11);
        assert!(snap.game_over);
        assert_eq!(snap.winner, 1);
    }

    #[test]
    fn pve_human_black_ai_follows() {
        let mut game = Game::new();
        game.set_play_mode(PlayMode::HumanVsAi);
        game.set_human_side(1);
        let snap = game.human_click(7, 7);
        assert_eq!(snap.pieces.len(), 2);
        assert_eq!(snap.turn, 1);
    }

    #[test]
    fn pve_human_white_ai_starts() {
        let mut game = Game::new();
        game.set_play_mode(PlayMode::HumanVsAi);
        game.set_human_side(-1);
        let snap = game.snapshot();
        assert_eq!(snap.pieces.len(), 1);
        assert_eq!(snap.turn, -1);
        assert!(snap.session.unwrap().human_input_enabled);
    }

    #[test]
    fn rejects_click_on_ai_turn() {
        let mut game = Game::new();
        game.set_play_mode(PlayMode::HumanVsAi);
        game.click(7, 7);
        let snap = game.click(8, 7);
        assert_eq!(snap.message, "轮到 AI 落子");
    }
}

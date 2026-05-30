use game_view::{CoordDto, PieceDto, SnapshotDto};

use board_engine::{Player, Position};

use crate::{
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
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
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

    pub fn snapshot(&self) -> SnapshotDto {
        let legal_moves = if self.game_over {
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

        if !self.board.is_empty(x, y) {
            self.message = "该位置已有棋子".to_string();
            return self.snapshot();
        }

        self.board.place(x, y, self.turn);
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

        self.snapshot()
    }
}

fn side_name(side: Player) -> &'static str {
    if side > 0 { "黑" } else { "白" }
}

impl Game {
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
}

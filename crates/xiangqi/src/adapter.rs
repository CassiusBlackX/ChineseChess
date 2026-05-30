use game_view::{GameViewAdapter, ViewInput, ViewOutput};

use crate::game::Game;

pub struct XiangqiAdapter {
    game: Game,
}

impl XiangqiAdapter {
    pub fn new() -> Self {
        Self { game: Game::new() }
    }
}

impl Default for XiangqiAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl GameViewAdapter for XiangqiAdapter {
    fn handle(&mut self, input: ViewInput) -> ViewOutput {
        match input {
            ViewInput::Snapshot => ViewOutput::Snapshot(self.game.snapshot()),
            ViewInput::Reset => {
                self.game.reset();
                ViewOutput::Snapshot(self.game.snapshot())
            }
            ViewInput::Click { x, y } => ViewOutput::Snapshot(self.game.click(x, y)),
            ViewInput::TryMove {
                from_x,
                from_y,
                to_x,
                to_y,
            } => ViewOutput::Snapshot(self.game.try_move(from_x, from_y, to_x, to_y)),
            ViewInput::LegalMoves { x, y } => match self.game.legal_moves(x, y) {
                Ok(moves) => ViewOutput::Moves(moves),
                Err(err) => ViewOutput::Error(err),
            },
        }
    }

    fn board_width(&self) -> usize {
        self.game.board_width()
    }

    fn board_height(&self) -> usize {
        self.game.board_height()
    }

    fn current_turn(&self) -> i8 {
        self.game.current_turn()
    }

    fn game_title(&self) -> &str {
        "中国象棋"
    }
}

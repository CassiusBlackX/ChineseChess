use game_view::{GameViewAdapter, ViewInput, ViewOutput};

use crate::game::Game;

pub struct GomokuAdapter {
    game: Game,
}

impl GomokuAdapter {
    pub fn new() -> Self {
        Self { game: Game::new() }
    }
}

impl Default for GomokuAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl GameViewAdapter for GomokuAdapter {
    fn handle(&mut self, input: ViewInput) -> ViewOutput {
        match input {
            ViewInput::Snapshot => ViewOutput::Snapshot(self.game.snapshot()),
            ViewInput::Reset => {
                self.game.reset();
                ViewOutput::Snapshot(self.game.snapshot())
            }
            ViewInput::Click { x, y } => ViewOutput::Snapshot(self.game.click(x, y)),
            ViewInput::TryMove { .. } | ViewInput::LegalMoves { .. } => {
                ViewOutput::Snapshot(self.game.snapshot())
            }
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
        "五子棋"
    }
}

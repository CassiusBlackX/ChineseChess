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
            ViewInput::Click { x, y } => ViewOutput::Snapshot(self.game.human_click(x, y)),
            ViewInput::SetPlayMode(mode) => {
                self.game.set_play_mode(mode);
                ViewOutput::Snapshot(self.game.snapshot())
            }
            ViewInput::SetAiDifficulty(difficulty) => {
                self.game.set_ai_difficulty(difficulty);
                ViewOutput::Snapshot(self.game.snapshot())
            }
            ViewInput::SetHumanSide(side) => {
                if side == 1 || side == -1 {
                    self.game.set_human_side(side);
                    ViewOutput::Snapshot(self.game.snapshot())
                } else {
                    ViewOutput::Error("执棋方只能是黑(1)或白(-1)".to_string())
                }
            }
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

    fn supports_session_config(&self) -> bool {
        true
    }
}

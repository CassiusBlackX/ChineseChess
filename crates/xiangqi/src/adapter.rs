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
            ViewInput::Click { x, y } => ViewOutput::Snapshot(self.game.human_click(x, y)),
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
                    ViewOutput::Error("执棋方只能是红(1)或黑(-1)".to_string())
                }
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
        "中国象棋"
    }

    fn supports_session_config(&self) -> bool {
        true
    }
}

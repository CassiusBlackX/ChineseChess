use crate::game::{Game, MoveDto, SnapshotDto};

pub enum ViewInput {
    Snapshot,
    Reset,
    Click { x: usize, y: usize },
    TryMove {
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
    },
    LegalMoves { x: usize, y: usize },
}

pub enum ViewOutput {
    Snapshot(SnapshotDto),
    Moves(Vec<MoveDto>),
    Error(String),
}

pub trait GameViewAdapter {
    fn handle(&mut self, input: ViewInput) -> ViewOutput;
    fn board_width(&self) -> usize;
    fn board_height(&self) -> usize;
    fn current_turn(&self) -> i8;
}

pub struct SharedGameAdapter {
    game: Game,
}

impl SharedGameAdapter {
    pub fn new() -> Self {
        Self { game: Game::new() }
    }
}

impl Default for SharedGameAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl GameViewAdapter for SharedGameAdapter {
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
}

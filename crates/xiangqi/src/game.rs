use game_view::{
    AiDifficulty, CoordDto, PieceDto, PlayMode, SessionDto, SnapshotDto,
};

use board_engine::{Player, Position, Vec2d};

use crate::{
    ai,
    board::{BOARD_HEIGHT, BOARD_WIDTH, Board},
    moves::{self, is_checkmate_on_board, Move},
    pos,
    rules::{self, is_side_in_check},
};

#[derive(Clone)]
pub struct Game {
    board: Board,
    selected: Option<Position>,
    turn: i8,
    in_check_side: i8,
    game_over: bool,
    winner: i8,
    message: String,
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
            selected: None,
            turn: 1,
            in_check_side: 0,
            game_over: false,
            winner: 0,
            message: "红方先手".to_string(),
            play_mode: PlayMode::LocalPvp,
            ai_difficulty: AiDifficulty::Medium,
            human_side: 1,
        }
    }

    pub fn reset(&mut self) {
        self.board = Board::new();
        self.selected = None;
        self.turn = 1;
        self.in_check_side = 0;
        self.game_over = false;
        self.winner = 0;
        self.message = "红方先手".to_string();

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

    pub fn current_turn(&self) -> i8 {
        self.turn
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

    pub fn snapshot(&mut self) -> SnapshotDto {
        let human_input_enabled = self.human_input_enabled();
        let legal_moves = if self.game_over || !human_input_enabled {
            Vec::new()
        } else {
            match self.selected {
                Some(selected) => {
                    let id = self.board.id_at(selected);
                    if id != 0 && self.same_side_with_turn(id) {
                        self.collect_legal_moves_for(id)
                    } else {
                        Vec::new()
                    }
                }
                None => Vec::new(),
            }
        };

        SnapshotDto {
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
            turn: self.turn,
            selected: self.selected.map(|p| CoordDto { x: p.x, y: p.y }),
            legal_moves,
            pieces: self.collect_pieces(),
            in_check_side: if self.in_check_side == 0 {
                None
            } else {
                Some(self.in_check_side)
            },
            game_over: self.game_over,
            winner: self.winner,
            message: self.message.clone(),
            last_move: None,
            session: Some(SessionDto {
                play_mode: self.play_mode,
                ai_difficulty: self.ai_difficulty,
                human_side: self.human_side,
                human_input_enabled,
            }),
        }
    }

    pub fn legal_moves(&mut self, x: usize, y: usize) -> Result<Vec<CoordDto>, String> {
        if x >= BOARD_WIDTH || y >= BOARD_HEIGHT {
            return Err("坐标越界".to_string());
        }

        let pos = pos!(x, y);
        let id = self.board.id_at(pos);
        if id == 0 || self.game_over {
            return Ok(Vec::new());
        }

        Ok(self.collect_legal_moves_for(id))
    }

    pub fn click(&mut self, x: usize, y: usize) -> SnapshotDto {
        if x >= BOARD_WIDTH || y >= BOARD_HEIGHT {
            self.message = "坐标越界".to_string();
            return self.snapshot();
        }

        if self.game_over {
            self.selected = None;
            self.message = "对局已结束，请重开一局".to_string();
            return self.snapshot();
        }

        if self.play_mode == PlayMode::HumanVsAi && self.turn != self.human_side {
            self.message = "轮到 AI 落子".to_string();
            return self.snapshot();
        }

        let pos = pos!(x, y);
        let id = self.board.id_at(pos);

        if let Some(selected) = self.selected {
            let selected_id = self.board.id_at(selected);
            if selected_id == 0 {
                self.selected = None;
                self.message = "已取消选择".to_string();
                return self.snapshot();
            }

            if id != 0 && self.same_side_with_turn(id) {
                self.selected = Some(pos);
                self.message = "已切换选择".to_string();
                return self.snapshot();
            }

            if !rules::is_move_safe(&self.board, selected_id, selected, pos) {
                self.message = "该走法会导致己方被将军".to_string();
                return self.snapshot();
            }

            let direction = Vec2d {
                x: x as i8 - selected.x as i8,
                y: y as i8 - selected.y as i8,
            };

            match self.board.walk(selected_id, direction) {
                Ok(()) => self.finish_turn_after_successful_move(),
                Err(err) => {
                    self.message = match err {
                        crate::board::WalkErr::OutOfBound => "目标越界".to_string(),
                        crate::board::WalkErr::Unreachable => "非法走法".to_string(),
                        crate::board::WalkErr::Hindered => "己方棋子阻挡".to_string(),
                    };
                }
            }

            return self.snapshot();
        }

        if id == 0 {
            self.message = "请选择一个棋子".to_string();
            return self.snapshot();
        }

        if self.play_mode == PlayMode::HumanVsAi && id.signum() != self.human_side {
            self.message = "请选择己方棋子".to_string();
            return self.snapshot();
        }

        if !self.same_side_with_turn(id) {
            self.message = "当前不是该方回合".to_string();
            return self.snapshot();
        }

        self.selected = Some(pos);
        self.message = "已选择棋子".to_string();
        self.snapshot()
    }

    pub fn human_click(&mut self, x: usize, y: usize) -> SnapshotDto {
        let turn_before = self.turn;
        let snap = self.click(x, y);
        if !self.game_over
            && self.play_mode == PlayMode::HumanVsAi
            && turn_before == self.human_side
            && self.turn != turn_before
        {
            self.ai_move();
            return self.snapshot();
        }
        snap
    }

    pub fn ai_move(&mut self) {
        if !self.needs_ai_move() {
            return;
        }

        let Some(mv) = ai::choose_move(&mut self.board, self.turn, self.ai_difficulty) else {
            self.message = "AI 无法落子".to_string();
            return;
        };

        self.apply_move(mv);
    }

    fn apply_move(&mut self, mv: Move) {
        if !moves::apply_move(&mut self.board, mv) {
            self.message = "AI 走法无效".to_string();
            return;
        }
        self.finish_turn_after_successful_move();
    }

    pub fn try_move(
        &mut self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
    ) -> SnapshotDto {
        if self.game_over {
            self.selected = None;
            self.message = "对局已结束，请重开一局".to_string();
            return self.snapshot();
        }

        if from_x >= BOARD_WIDTH
            || from_y >= BOARD_HEIGHT
            || to_x >= BOARD_WIDTH
            || to_y >= BOARD_HEIGHT
        {
            self.message = "坐标越界".to_string();
            return self.snapshot();
        }

        let from = pos!(from_x, from_y);
        let from_id = self.board.id_at(from);
        if from_id == 0 {
            self.message = "起点无棋子".to_string();
            return self.snapshot();
        }

        if !self.same_side_with_turn(from_id) {
            self.message = "当前不是该方回合".to_string();
            return self.snapshot();
        }

        let to = pos!(to_x, to_y);
        if !rules::is_move_safe(&self.board, from_id, from, to) {
            self.message = "该走法会导致己方被将军".to_string();
            return self.snapshot();
        }

        let direction = Vec2d {
            x: to_x as i8 - from_x as i8,
            y: to_y as i8 - from_y as i8,
        };

        match self.board.walk(from_id, direction) {
            Ok(()) => self.finish_turn_after_successful_move(),
            Err(err) => {
                self.message = match err {
                    crate::board::WalkErr::OutOfBound => "目标越界".to_string(),
                    crate::board::WalkErr::Unreachable => "非法走法".to_string(),
                    crate::board::WalkErr::Hindered => "己方棋子阻挡".to_string(),
                };
            }
        }

        self.snapshot()
    }

    fn same_side_with_turn(&self, id: i8) -> bool {
        id.signum() == self.turn
    }

    fn collect_legal_moves_for(&mut self, id: i8) -> Vec<CoordDto> {
        rules::pseudo_moves_for_piece(&mut self.board, id)
            .into_iter()
            .filter(|to| {
                let from = self
                    .board
                    .get_piece(id)
                    .map(|piece| piece.get_pos())
                    .unwrap_or(Position { x: 0, y: 0 });
                rules::is_move_safe(&self.board, id, from, *to)
            })
            .map(|p| CoordDto { x: p.x, y: p.y })
            .collect()
    }

    fn is_checkmate_for_side(&self, side: i8) -> bool {
        is_checkmate_on_board(&self.board, side)
    }

    fn side_name(side: i8) -> &'static str {
        if side > 0 {
            "红"
        } else {
            "黑"
        }
    }

    fn finish_turn_after_successful_move(&mut self) {
        self.selected = None;
        self.turn = -self.turn;

        let mut current = self.board.clone();
        if is_side_in_check(&mut current, self.turn) {
            self.in_check_side = self.turn;
            if self.is_checkmate_for_side(self.turn) {
                self.game_over = true;
                self.winner = -self.turn;
                self.selected = None;
                self.message = format!("将死，{}方胜", Self::side_name(self.winner));
            } else {
                self.message = format!("将军：{}方", Self::side_name(self.turn));
            }
        } else {
            self.in_check_side = 0;
            self.message = "落子成功".to_string();
        }
    }

    fn collect_pieces(&self) -> Vec<PieceDto> {
        let mut pieces = Vec::new();
        for (x, y) in self.board.board_status().iter_coords() {
            let id = self.board.board_status().get(x, y).unwrap_or(0);
            if id == 0 {
                continue;
            }

            let symbol = self
                .board
                .piece_name(id)
                .map(|ch| ch.to_string())
                .unwrap_or_else(|| "?".to_string());

            pieces.push(PieceDto {
                id,
                x,
                y,
                side: id.signum(),
                symbol,
            });
        }
        pieces
    }
}

#[cfg(test)]
impl Game {
    fn from_board_for_test(board: Board, turn: i8) -> Self {
        Self {
            board,
            selected: None,
            turn,
            in_check_side: 0,
            game_over: false,
            winner: 0,
            message: String::new(),
            play_mode: PlayMode::LocalPvp,
            ai_difficulty: AiDifficulty::Medium,
            human_side: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        board::generate_board,
        chess::{
            BLACK_KING_ID, BLACK_LEFT_ELEPHANT_ID, BLACK_RIGHT_ELEPHANT_ID,
            RED_KING_ID, RED_LEFT_CAR_ID, RED_RIGHT_CAR_ID,
        },
        pos,
    };

    #[test]
    fn check_detection_works() {
        let board_status = generate_board(vec![
            (RED_KING_ID, pos!(4, 0)),
            (BLACK_KING_ID, pos!(4, 9)),
            (RED_LEFT_CAR_ID, pos!(4, 8)),
        ]);
        let game = Game::from_board_for_test(Board::from_board_status(board_status), -1);
        let mut probe = game.board.clone();

        assert!(is_side_in_check(&mut probe, -1));
        assert!(!game.is_checkmate_for_side(-1));
    }

    #[test]
    fn checkmate_detection_works() {
        let board_status = generate_board(vec![
            (RED_KING_ID, pos!(4, 0)),
            (BLACK_KING_ID, pos!(4, 9)),
            (RED_LEFT_CAR_ID, pos!(4, 8)),
            (RED_RIGHT_CAR_ID, pos!(3, 8)),
            (BLACK_LEFT_ELEPHANT_ID, pos!(3, 9)),
            (BLACK_RIGHT_ELEPHANT_ID, pos!(5, 9)),
        ]);
        let game = Game::from_board_for_test(Board::from_board_status(board_status), -1);

        assert!(game.is_checkmate_for_side(-1));
    }

    #[test]
    fn game_ends_on_checkmate_and_locks() {
        let board_status = generate_board(vec![
            (RED_KING_ID, pos!(4, 0)),
            (BLACK_KING_ID, pos!(4, 9)),
            (RED_LEFT_CAR_ID, pos!(4, 7)),
            (RED_RIGHT_CAR_ID, pos!(3, 8)),
            (BLACK_LEFT_ELEPHANT_ID, pos!(3, 9)),
            (BLACK_RIGHT_ELEPHANT_ID, pos!(5, 9)),
        ]);
        let mut game = Game::from_board_for_test(Board::from_board_status(board_status), 1);

        let snapshot = game.try_move(4, 7, 4, 8);
        assert!(snapshot.game_over);
        assert_eq!(snapshot.winner, 1);

        let blocked = game.click(4, 9);
        assert!(blocked.game_over);
        assert_eq!(blocked.message, "对局已结束，请重开一局");
    }

    #[test]
    fn pve_human_red_ai_follows() {
        let mut game = Game::new();
        game.set_play_mode(PlayMode::HumanVsAi);
        game.click(0, 0);
        let snap = game.human_click(0, 1);
        assert!(snap.turn == 1);
        assert!(snap.session.unwrap().human_input_enabled);
    }

    #[test]
    fn pve_human_black_ai_starts() {
        let mut game = Game::new();
        game.set_play_mode(PlayMode::HumanVsAi);
        game.set_human_side(-1);
        let snap = game.snapshot();
        assert_eq!(snap.turn, -1);
        assert!(snap.session.unwrap().human_input_enabled);
    }
}

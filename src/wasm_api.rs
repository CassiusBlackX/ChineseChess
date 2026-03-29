use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{
    board::{BOARD_HEIGHT, BOARD_WIDTH, Board, WalkErr},
    chess::{BLACK_KING_ID, RED_KING_ID},
    pos,
    position::Position,
    vec2d::Vec2d,
};

#[derive(Serialize)]
struct MoveDto {
    x: usize,
    y: usize,
}

#[derive(Serialize)]
struct PieceDto {
    id: i8,
    x: usize,
    y: usize,
    side: i8,
    symbol: String,
}

#[derive(Serialize)]
struct SnapshotDto {
    turn: i8,
    selected: Option<MoveDto>,
    legal_moves: Vec<MoveDto>,
    pieces: Vec<PieceDto>,
    in_check_side: i8,
    game_over: bool,
    winner: i8,
    message: String,
}

#[wasm_bindgen]
pub struct WasmGame {
    board: Board,
    selected: Option<Position>,
    turn: i8,
    in_check_side: i8,
    game_over: bool,
    winner: i8,
    message: String,
}

#[wasm_bindgen]
impl WasmGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            selected: None,
            turn: 1,
            in_check_side: 0,
            game_over: false,
            winner: 0,
            message: "红方先手".to_string(),
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

    pub fn snapshot(&mut self) -> Result<JsValue, JsValue> {
        self.serialize_snapshot()
    }

    pub fn legal_moves(&mut self, x: usize, y: usize) -> Result<JsValue, JsValue> {
        if x >= BOARD_WIDTH || y >= BOARD_HEIGHT {
            return Err(JsValue::from_str("坐标越界"));
        }
        let pos = pos!(x, y);
        let id = self.board.id_at(pos);
        if id == 0 || self.game_over {
            return Ok(Self::to_js_value(Vec::<MoveDto>::new())?);
        }
        let moves = self.collect_legal_moves_for(id);
        Self::to_js_value(moves)
    }

    pub fn click(&mut self, x: usize, y: usize) -> Result<JsValue, JsValue> {
        if x >= BOARD_WIDTH || y >= BOARD_HEIGHT {
            self.message = "坐标越界".to_string();
            return self.serialize_snapshot();
        }

        if self.game_over {
            self.selected = None;
            self.message = "对局已结束，请重开一局".to_string();
            return self.serialize_snapshot();
        }

        let pos = pos!(x, y);
        let id = self.board.id_at(pos);

        if let Some(selected) = self.selected {
            let selected_id = self.board.id_at(selected);
            if selected_id == 0 {
                self.selected = None;
                self.message = "已取消选择".to_string();
                return self.serialize_snapshot();
            }

            if id != 0 && self.same_side_with_turn(id) {
                self.selected = Some(pos);
                self.message = "已切换选择".to_string();
                return self.serialize_snapshot();
            }

            let direction = Vec2d {
                x: x as i8 - selected.x as i8,
                y: y as i8 - selected.y as i8,
            };

            if !self.is_move_safe(selected_id, selected, pos) {
                self.message = "该走法会导致己方被将军".to_string();
                return self.serialize_snapshot();
            }

            match self.board.walk(selected_id, direction) {
                Ok(()) => {
                    self.finish_turn_after_successful_move();
                }
                Err(WalkErr::OutOfBound) => {
                    self.message = "目标越界".to_string();
                }
                Err(WalkErr::Unreachable) => {
                    self.message = "非法走法".to_string();
                }
                Err(WalkErr::Hindered) => {
                    self.message = "己方棋子阻挡".to_string();
                }
            }
            return self.serialize_snapshot();
        }

        if id == 0 {
            self.message = "请选择一个棋子".to_string();
            return self.serialize_snapshot();
        }

        if !self.same_side_with_turn(id) {
            self.message = "当前不是该方回合".to_string();
            return self.serialize_snapshot();
        }

        self.selected = Some(pos);
        self.message = "已选择棋子".to_string();
        self.serialize_snapshot()
    }

    pub fn try_move(
        &mut self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
    ) -> Result<JsValue, JsValue> {
        if self.game_over {
            self.selected = None;
            self.message = "对局已结束，请重开一局".to_string();
            return self.serialize_snapshot();
        }

        if from_x >= BOARD_WIDTH || from_y >= BOARD_HEIGHT || to_x >= BOARD_WIDTH || to_y >= BOARD_HEIGHT {
            return Err(JsValue::from_str("坐标越界"));
        }

        let from = pos!(from_x, from_y);
        let from_id = self.board.id_at(from);
        if from_id == 0 {
            self.message = "起点无棋子".to_string();
            return self.serialize_snapshot();
        }
        if !self.same_side_with_turn(from_id) {
            self.message = "当前不是该方回合".to_string();
            return self.serialize_snapshot();
        }

        let direction = Vec2d {
            x: to_x as i8 - from_x as i8,
            y: to_y as i8 - from_y as i8,
        };

        if !self.is_move_safe(from_id, from, pos!(to_x, to_y)) {
            self.message = "该走法会导致己方被将军".to_string();
            return self.serialize_snapshot();
        }

        match self.board.walk(from_id, direction) {
            Ok(()) => {
                self.finish_turn_after_successful_move();
            }
            Err(WalkErr::OutOfBound) => self.message = "目标越界".to_string(),
            Err(WalkErr::Unreachable) => self.message = "非法走法".to_string(),
            Err(WalkErr::Hindered) => self.message = "己方棋子阻挡".to_string(),
        }
        self.serialize_snapshot()
    }
}

impl WasmGame {
    fn same_side_with_turn(&self, id: i8) -> bool {
        id.signum() == self.turn
    }

    fn collect_legal_moves_for(&mut self, id: i8) -> Vec<MoveDto> {
        Self::collect_legal_moves_for_board(&mut self.board, id)
    }

    fn collect_legal_moves_for_board(board: &mut Board, id: i8) -> Vec<MoveDto> {
        board
            .walk_options(id)
            .iter()
            .filter_map(|opt| opt.as_ref())
            .map(|p| MoveDto { x: p.x, y: p.y })
            .collect()
    }

    fn is_move_safe(&self, id: i8, from: Position, to: Position) -> bool {
        let mut simulated = self.board.clone();
        let direction = Vec2d {
            x: to.x as i8 - from.x as i8,
            y: to.y as i8 - from.y as i8,
        };
        if simulated.walk(id, direction).is_err() {
            return false;
        }
        !Self::is_side_in_check_on_board(&mut simulated, id.signum())
    }

    fn find_king_pos_on_board(board: &Board, side: i8) -> Option<Position> {
        let king_id = if side > 0 { RED_KING_ID } else { BLACK_KING_ID };
        let king = board.get_piece(king_id)?;
        if !king.is_alive() {
            return None;
        }
        Some(king.get_pos())
    }

    fn kings_face_each_other_on_board(board: &Board) -> bool {
        let Some(red_king_pos) = Self::find_king_pos_on_board(board, 1) else {
            return false;
        };
        let Some(black_king_pos) = Self::find_king_pos_on_board(board, -1) else {
            return false;
        };

        if red_king_pos.x != black_king_pos.x {
            return false;
        }

        let x = red_king_pos.x;
        let min_y = red_king_pos.y.min(black_king_pos.y);
        let max_y = red_king_pos.y.max(black_king_pos.y);
        for y in (min_y + 1)..max_y {
            if board.board_status()[x][y] != 0 {
                return false;
            }
        }
        true
    }

    fn is_side_in_check_on_board(board: &mut Board, side: i8) -> bool {
        if Self::kings_face_each_other_on_board(board) {
            return true;
        }

        let Some(king_pos) = Self::find_king_pos_on_board(board, side) else {
            return true;
        };

        let mut enemy_ids = Vec::new();
        let board_status = *board.board_status();
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                let id = board_status[x][y];
                if id != 0 && id.signum() == -side {
                    enemy_ids.push(id);
                }
            }
        }

        for enemy_id in enemy_ids {
            let moves = Self::collect_legal_moves_for_board(board, enemy_id);
            if moves.iter().any(|m| m.x == king_pos.x && m.y == king_pos.y) {
                return true;
            }
        }
        false
    }

    fn is_checkmate_for_side(&self, side: i8) -> bool {
        let mut probe = self.board.clone();
        if !Self::is_side_in_check_on_board(&mut probe, side) {
            return false;
        }

        let board_status = *self.board.board_status();
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                let id = board_status[x][y];
                if id == 0 || id.signum() != side {
                    continue;
                }

                let from = Position { x, y };
                let mut simulation_for_moves = self.board.clone();
                let moves = Self::collect_legal_moves_for_board(&mut simulation_for_moves, id);
                for mv in moves {
                    let mut simulation = self.board.clone();
                    let direction = Vec2d {
                        x: mv.x as i8 - from.x as i8,
                        y: mv.y as i8 - from.y as i8,
                    };

                    if simulation.walk(id, direction).is_ok()
                        && !Self::is_side_in_check_on_board(&mut simulation, side)
                    {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn side_name(side: i8) -> &'static str {
        if side > 0 { "红" } else { "黑" }
    }

    fn finish_turn_after_successful_move(&mut self) {
        self.selected = None;
        self.turn = -self.turn;

        let mut current = self.board.clone();
        if Self::is_side_in_check_on_board(&mut current, self.turn) {
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
        let board_status = self.board.board_status();
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                let id = board_status[x][y];
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
        }
        pieces
    }

    fn serialize_snapshot(&mut self) -> Result<JsValue, JsValue> {
        let legal_moves = match self.selected {
            Some(selected) => {
                let id = self.board.id_at(selected);
                if id != 0 && self.same_side_with_turn(id) {
                    self.collect_legal_moves_for(id)
                } else {
                    Vec::new()
                }
            }
            None => Vec::new(),
        };

        let snapshot = SnapshotDto {
            turn: self.turn,
            selected: self.selected.map(|p| MoveDto { x: p.x, y: p.y }),
            legal_moves,
            pieces: self.collect_pieces(),
            in_check_side: self.in_check_side,
            game_over: self.game_over,
            winner: self.winner,
            message: self.message.clone(),
        };

        Self::to_js_value(snapshot)
    }

    fn to_js_value<T: Serialize>(value: T) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&value)
            .map_err(|err| JsValue::from_str(&format!("序列化失败: {err}")))
    }
}

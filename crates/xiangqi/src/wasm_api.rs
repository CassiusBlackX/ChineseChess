use wasm_bindgen::prelude::*;

use game_view::{AiDifficulty, GameViewAdapter, PlayMode, ViewInput, ViewOutput};

use crate::adapter::XiangqiAdapter;

#[wasm_bindgen]
pub struct WasmGame {
    adapter: XiangqiAdapter,
}

#[wasm_bindgen]
impl WasmGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            adapter: XiangqiAdapter::new(),
        }
    }

    pub fn reset(&mut self) {
        let _ = self.adapter.handle(ViewInput::Reset);
    }

    pub fn board_width(&self) -> usize {
        self.adapter.board_width()
    }

    pub fn board_height(&self) -> usize {
        self.adapter.board_height()
    }

    pub fn current_turn(&self) -> i8 {
        self.adapter.current_turn()
    }

    pub fn snapshot(&mut self) -> Result<JsValue, JsValue> {
        match self.adapter.handle(ViewInput::Snapshot) {
            ViewOutput::Snapshot(snapshot) => Self::to_js_value(snapshot),
            ViewOutput::Moves(_) => Err(JsValue::from_str("内部状态错误")),
            ViewOutput::Error(err) => Err(JsValue::from_str(&err)),
        }
    }

    pub fn legal_moves(&mut self, x: usize, y: usize) -> Result<JsValue, JsValue> {
        match self.adapter.handle(ViewInput::LegalMoves { x, y }) {
            ViewOutput::Moves(moves) => Self::to_js_value(moves),
            ViewOutput::Snapshot(_) => Err(JsValue::from_str("内部状态错误")),
            ViewOutput::Error(err) => Err(JsValue::from_str(&err)),
        }
    }

    pub fn click(&mut self, x: usize, y: usize) -> Result<JsValue, JsValue> {
        match self.adapter.handle(ViewInput::Click { x, y }) {
            ViewOutput::Snapshot(snapshot) => Self::to_js_value(snapshot),
            ViewOutput::Moves(_) => Err(JsValue::from_str("内部状态错误")),
            ViewOutput::Error(err) => Err(JsValue::from_str(&err)),
        }
    }

    pub fn try_move(
        &mut self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
    ) -> Result<JsValue, JsValue> {
        match self.adapter.handle(ViewInput::TryMove {
            from_x,
            from_y,
            to_x,
            to_y,
        }) {
            ViewOutput::Snapshot(snapshot) => Self::to_js_value(snapshot),
            ViewOutput::Moves(_) => Err(JsValue::from_str("内部状态错误")),
            ViewOutput::Error(err) => Err(JsValue::from_str(&err)),
        }
    }

    pub fn set_play_mode(&mut self, mode: &str) -> Result<JsValue, JsValue> {
        let play_mode = match mode {
            "pvp" | "local_pvp" => PlayMode::LocalPvp,
            "pve" | "human_vs_ai" => PlayMode::HumanVsAi,
            _ => return Err(JsValue::from_str("无效模式，请使用 pvp 或 pve")),
        };
        match self.adapter.handle(ViewInput::SetPlayMode(play_mode)) {
            ViewOutput::Snapshot(snapshot) => Self::to_js_value(snapshot),
            ViewOutput::Error(err) => Err(JsValue::from_str(&err)),
            ViewOutput::Moves(_) => Err(JsValue::from_str("内部状态错误")),
        }
    }

    pub fn set_ai_difficulty(&mut self, level: &str) -> Result<JsValue, JsValue> {
        let difficulty = match level {
            "easy" => AiDifficulty::Easy,
            "medium" => AiDifficulty::Medium,
            "hard" => AiDifficulty::Hard,
            _ => return Err(JsValue::from_str("无效难度，请使用 easy、medium 或 hard")),
        };
        match self.adapter.handle(ViewInput::SetAiDifficulty(difficulty)) {
            ViewOutput::Snapshot(snapshot) => Self::to_js_value(snapshot),
            ViewOutput::Error(err) => Err(JsValue::from_str(&err)),
            ViewOutput::Moves(_) => Err(JsValue::from_str("内部状态错误")),
        }
    }

    pub fn set_human_side(&mut self, side: &str) -> Result<JsValue, JsValue> {
        let human_side = match side {
            "red" => 1,
            "black" => -1,
            _ => return Err(JsValue::from_str("无效执棋，请使用 red 或 black")),
        };
        match self.adapter.handle(ViewInput::SetHumanSide(human_side)) {
            ViewOutput::Snapshot(snapshot) => Self::to_js_value(snapshot),
            ViewOutput::Error(err) => Err(JsValue::from_str(&err)),
            ViewOutput::Moves(_) => Err(JsValue::from_str("内部状态错误")),
        }
    }
}

impl WasmGame {
    fn to_js_value<T: serde::Serialize>(value: T) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&value)
            .map_err(|err| JsValue::from_str(&format!("序列化失败: {err}")))
    }
}

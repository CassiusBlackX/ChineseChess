use wasm_bindgen::prelude::*;

use crate::view_adapter::{GameViewAdapter, SharedGameAdapter, ViewInput, ViewOutput};

#[wasm_bindgen]
pub struct WasmGame {
    adapter: SharedGameAdapter,
}

#[wasm_bindgen]
impl WasmGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            adapter: SharedGameAdapter::new(),
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
}

impl WasmGame {
    fn to_js_value<T: serde::Serialize>(value: T) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&value)
            .map_err(|err| JsValue::from_str(&format!("序列化失败: {err}")))
    }
}

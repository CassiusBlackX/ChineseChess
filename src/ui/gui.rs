use std::collections::HashMap;

use chinese_chess::{
    game::SnapshotDto,
    view_adapter::{GameViewAdapter, SharedGameAdapter, ViewInput, ViewOutput},
};
use eframe::egui::{self, Color32, FontData, FontDefinitions, FontFamily, RichText};

struct DesktopChessApp {
    adapter: SharedGameAdapter,
    snapshot: SnapshotDto,
}

impl DesktopChessApp {
    fn new() -> Self {
        let mut adapter = SharedGameAdapter::new();
        let snapshot = match adapter.handle(ViewInput::Snapshot) {
            ViewOutput::Snapshot(s) => s,
            ViewOutput::Moves(_) | ViewOutput::Error(_) => {
                panic!("adapter should return snapshot for ViewInput::Snapshot")
            }
        };
        Self { adapter, snapshot }
    }

    fn apply_cjk_font(ctx: &egui::Context) {
        let font_candidates = [
            "C:/Windows/Fonts/simhei.ttf",
            "C:/Windows/Fonts/simkai.ttf",
            "C:/Windows/Fonts/simsun.ttc",
            "C:/Windows/Fonts/msyh.ttc",
            "/System/Library/Fonts/PingFang.ttc",
            "/System/Library/Fonts/STHeiti Light.ttc",
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/truetype/arphic/ukai.ttc",
        ];

        for path in font_candidates {
            if let Ok(bytes) = std::fs::read(path) {
                let mut fonts = FontDefinitions::default();
                fonts
                    .font_data
                    .insert("cjk_fallback".to_string(), FontData::from_owned(bytes).into());
                fonts
                    .families
                    .entry(FontFamily::Proportional)
                    .or_default()
                    .insert(0, "cjk_fallback".to_string());
                fonts
                    .families
                    .entry(FontFamily::Monospace)
                    .or_default()
                    .push("cjk_fallback".to_string());
                ctx.set_fonts(fonts);
                return;
            }
        }
    }
}

impl eframe::App for DesktopChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("status_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("中国象棋 · 原生桌面版 (Rust)");
                if ui.button("重开一局").clicked()
                    && let ViewOutput::Snapshot(snapshot) = self.adapter.handle(ViewInput::Reset)
                {
                    self.snapshot = snapshot;
                }
            });

            let turn_text = if self.snapshot.turn > 0 { "红方" } else { "黑方" };
            let mut status = format!("{} | 当前回合: {}", self.snapshot.message, turn_text);
            if self.snapshot.game_over {
                let winner = if self.snapshot.winner > 0 { "红方" } else { "黑方" };
                status.push_str(&format!(" | 对局结束: {}胜", winner));
            } else if self.snapshot.in_check_side != 0 {
                let checked = if self.snapshot.in_check_side > 0 { "红方" } else { "黑方" };
                status.push_str(&format!(" | 被将军: {}", checked));
            }
            ui.label(status);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut piece_map: HashMap<(usize, usize), (String, i8)> = HashMap::new();
            for piece in &self.snapshot.pieces {
                piece_map.insert((piece.x, piece.y), (piece.symbol.clone(), piece.side));
            }

            let mut legal_moves = HashMap::new();
            for mv in &self.snapshot.legal_moves {
                legal_moves.insert((mv.x, mv.y), true);
            }

            ui.vertical_centered(|ui| {
                egui::Grid::new("board_grid")
                    .spacing([0.0, 0.0])
                    .show(ui, |ui| {
                        for y in (0..self.adapter.board_height()).rev() {
                            for x in 0..self.adapter.board_width() {
                                let is_selected = self
                                    .snapshot
                                    .selected
                                    .as_ref()
                                    .map(|s| s.x == x && s.y == y)
                                    .unwrap_or(false);
                                let is_hint = legal_moves.contains_key(&(x, y));

                                let (symbol, side) = piece_map
                                    .get(&(x, y))
                                    .cloned()
                                    .unwrap_or_else(|| (" ".to_string(), 0));

                                let mut text = RichText::new(symbol).size(28.0);
                                if side > 0 {
                                    text = text.color(Color32::from_rgb(183, 34, 34));
                                } else if side < 0 {
                                    text = text.color(Color32::from_rgb(47, 42, 38));
                                }

                                let mut button = egui::Button::new(text)
                                    .min_size(egui::vec2(52.0, 52.0))
                                    .fill(Color32::from_rgb(242, 221, 185));

                                if is_selected {
                                    button = button.stroke(egui::Stroke::new(
                                        2.0,
                                        Color32::from_rgb(192, 125, 58),
                                    ));
                                } else if is_hint {
                                    button = button.stroke(egui::Stroke::new(
                                        2.0,
                                        Color32::from_rgb(77, 159, 87),
                                    ));
                                } else {
                                    button = button.stroke(egui::Stroke::new(
                                        1.0,
                                        Color32::from_rgb(168, 105, 82),
                                    ));
                                }

                                let response = ui.add_enabled(!self.snapshot.game_over, button);
                                if response.clicked()
                                    && let ViewOutput::Snapshot(snapshot) =
                                        self.adapter.handle(ViewInput::Click { x, y })
                                {
                                    self.snapshot = snapshot;
                                }
                            }
                            ui.end_row();
                        }
                    });
            });
        });
    }
}

pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([620.0, 760.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Chinese Chess Desktop",
        options,
        Box::new(|cc| {
            DesktopChessApp::apply_cjk_font(&cc.egui_ctx);
            Ok(Box::new(DesktopChessApp::new()))
        }),
    )
}

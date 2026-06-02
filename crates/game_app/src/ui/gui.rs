use std::collections::HashMap;

use eframe::egui::{self, Color32, FontData, FontDefinitions, FontFamily, RichText};
use game_view::{
    AiDifficulty, GameViewAdapter, PlayMode, SnapshotDto, ViewInput, ViewOutput,
};

use crate::ui::common::{format_status, human_input_enabled, piece_color_rgb};

struct DesktopGameApp {
    adapter: Box<dyn GameViewAdapter>,
    snapshot: SnapshotDto,
    game_title: String,
    supports_session: bool,
}

impl DesktopGameApp {
    fn new(mut adapter: Box<dyn GameViewAdapter>) -> Self {
        let game_title = adapter.game_title().to_string();
        let supports_session = adapter.supports_session_config();
        let snapshot = match adapter.handle(ViewInput::Snapshot) {
            ViewOutput::Snapshot(s) => s,
            ViewOutput::Moves(_) | ViewOutput::Error(_) => {
                panic!("adapter should return snapshot for ViewInput::Snapshot")
            }
        };
        Self {
            adapter,
            snapshot,
            game_title,
            supports_session,
        }
    }

    fn apply_session_input(&mut self, input: ViewInput) {
        if let ViewOutput::Snapshot(snapshot) = self.adapter.handle(input) {
            self.snapshot = snapshot;
        }
    }

    fn draw_session_controls(&mut self, ui: &mut egui::Ui) {
        if !self.supports_session {
            return;
        }

        let session = self.snapshot.session.clone();
        let Some(session) = session else {
            return;
        };

        let mut pending: Option<ViewInput> = None;

        ui.separator();
        ui.label("模式:");
        let mut play_mode = session.play_mode;
        egui::ComboBox::from_id_salt("play_mode")
            .selected_text(match play_mode {
                PlayMode::LocalPvp => "人人对战",
                PlayMode::HumanVsAi => "人机对战",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut play_mode, PlayMode::LocalPvp, "人人对战");
                ui.selectable_value(&mut play_mode, PlayMode::HumanVsAi, "人机对战");
            });
        if play_mode != session.play_mode {
            pending = Some(ViewInput::SetPlayMode(play_mode));
        }

        ui.label("难度:");
        let mut difficulty = session.ai_difficulty;
        let difficulty_enabled = session.play_mode == PlayMode::HumanVsAi;
        ui.add_enabled_ui(difficulty_enabled, |ui| {
            egui::ComboBox::from_id_salt("ai_difficulty")
                .selected_text(match difficulty {
                    AiDifficulty::Easy => "简单",
                    AiDifficulty::Medium => "中等",
                    AiDifficulty::Hard => "困难",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut difficulty, AiDifficulty::Easy, "简单");
                    ui.selectable_value(&mut difficulty, AiDifficulty::Medium, "中等");
                    ui.selectable_value(&mut difficulty, AiDifficulty::Hard, "困难");
                });
        });
        if difficulty_enabled && difficulty != session.ai_difficulty {
            pending = Some(ViewInput::SetAiDifficulty(difficulty));
        }

        ui.label("执棋:");
        let mut human_side = session.human_side;
        egui::ComboBox::from_id_salt("human_side")
            .selected_text(if human_side > 0 { "执黑" } else { "执白" })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut human_side, 1, "执黑");
                ui.selectable_value(&mut human_side, -1, "执白");
            });
        if human_side != session.human_side {
            pending = Some(ViewInput::SetHumanSide(human_side));
        }

        if let Some(input) = pending {
            self.apply_session_input(input);
        }
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

impl eframe::App for DesktopGameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let title = self.game_title.clone();
        egui::TopBottomPanel::top("status_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(format!("{} · 原生桌面版 (Rust)", title));
                if ui.button("重开一局").clicked()
                    && let ViewOutput::Snapshot(snapshot) = self.adapter.handle(ViewInput::Reset)
                {
                    self.snapshot = snapshot;
                }
                self.draw_session_controls(ui);
            });
            ui.label(format_status(&self.snapshot, &title));
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

            let last_move = self
                .snapshot
                .last_move
                .as_ref()
                .map(|m| (m.x, m.y));

            let board_w = self.adapter.board_width();
            let board_h = self.adapter.board_height();
            let cell_size = if board_w > 10 { 36.0 } else { 52.0 };
            let board_enabled = human_input_enabled(&self.snapshot) && !self.snapshot.game_over;

            ui.vertical_centered(|ui| {
                egui::Grid::new("board_grid")
                    .spacing([0.0, 0.0])
                    .show(ui, |ui| {
                        for y in (0..board_h).rev() {
                            for x in 0..board_w {
                                let is_selected = self
                                    .snapshot
                                    .selected
                                    .as_ref()
                                    .map(|s| s.x == x && s.y == y)
                                    .unwrap_or(false);
                                let is_hint = legal_moves.contains_key(&(x, y));
                                let is_last = last_move == Some((x, y));

                                let (symbol, side) = piece_map
                                    .get(&(x, y))
                                    .cloned()
                                    .unwrap_or_else(|| (" ".to_string(), 0));

                                let mut text = RichText::new(symbol).size(if board_w > 10 { 22.0 } else { 28.0 });
                                if side != 0 {
                                    let (r, g, b) = piece_color_rgb(side);
                                    text = text.color(Color32::from_rgb(r, g, b));
                                }

                                let mut button = egui::Button::new(text)
                                    .min_size(egui::vec2(cell_size, cell_size))
                                    .fill(Color32::from_rgb(242, 221, 185));

                                if is_selected {
                                    button = button.stroke(egui::Stroke::new(
                                        2.0,
                                        Color32::from_rgb(192, 125, 58),
                                    ));
                                } else if is_last {
                                    button = button.stroke(egui::Stroke::new(
                                        2.0,
                                        Color32::from_rgb(70, 130, 200),
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

                                let response = ui.add_enabled(board_enabled, button);
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

pub fn run_gui(adapter: Box<dyn GameViewAdapter>) -> Result<(), eframe::Error> {
    let title = format!("{} Desktop", adapter.game_title());
    let window_w = if adapter.board_width() > 10 { 680.0 } else { 620.0 };
    let window_h = if adapter.board_height() > 10 { 820.0 } else { 760.0 };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([window_w, window_h]),
        ..Default::default()
    };

    eframe::run_native(
        &title,
        options,
        Box::new(move |cc| {
            DesktopGameApp::apply_cjk_font(&cc.egui_ctx);
            Ok(Box::new(DesktopGameApp::new(adapter)))
        }),
    )
}

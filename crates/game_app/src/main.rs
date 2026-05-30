mod launcher;
mod ui;

use launcher::{create_adapter, parse_args, prompt_game_and_mode, RunMode};

fn main() {
    let (game, mode) = parse_args().unwrap_or_else(prompt_game_and_mode);
    let adapter = create_adapter(game);

    match mode {
        RunMode::Gui => {
            if let Err(err) = ui::gui::run_gui(adapter) {
                eprintln!("GUI 启动失败: {}", err);
            }
        }
        RunMode::Tui => {
            if let Err(err) = ui::tui::run_tui(adapter) {
                eprintln!("TUI 启动失败: {}", err);
            }
        }
    }
}

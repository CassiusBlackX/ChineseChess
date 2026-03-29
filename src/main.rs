mod ui;

use std::io::{self, Write};

enum RunMode {
    Gui,
    Tui,
}

fn parse_mode_arg() -> Option<RunMode> {
    let arg = std::env::args().nth(1)?;
    match arg.as_str() {
        "gui" | "--gui" => Some(RunMode::Gui),
        "tui" | "--tui" => Some(RunMode::Tui),
        _ => None,
    }
}

fn prompt_mode() -> io::Result<RunMode> {
    println!("请选择运行模式:");
    println!("  1. GUI (原生窗口)");
    println!("  2. TUI (终端界面)");
    print!("输入 1 或 2: ");
    io::stdout().flush()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    if line.trim() == "2" {
        Ok(RunMode::Tui)
    } else {
        Ok(RunMode::Gui)
    }
}

fn main() {
    let mode = parse_mode_arg().unwrap_or_else(|| prompt_mode().unwrap_or(RunMode::Gui));
    match mode {
        RunMode::Gui => {
            if let Err(err) = ui::gui::run_gui() {
                eprintln!("GUI 启动失败: {}", err);
            }
        }
        RunMode::Tui => {
            if let Err(err) = ui::tui::run_tui() {
                eprintln!("TUI 启动失败: {}", err);
            }
        }
    }
}

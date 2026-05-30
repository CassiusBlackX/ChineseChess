use game_view::GameViewAdapter;
use gomoku::GomokuAdapter;
use xiangqi::XiangqiAdapter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameKind {
    Xiangqi,
    Gomoku,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunMode {
    Gui,
    Tui,
}

pub fn parse_args() -> Option<(GameKind, RunMode)> {
    let mut args = std::env::args().skip(1);
    let game = match args.next()?.as_str() {
        "xiangqi" | "xq" | "chess" => GameKind::Xiangqi,
        "gomoku" | "gmk" | "wuziqi" => GameKind::Gomoku,
        _ => return None,
    };
    let mode = match args.next()?.as_str() {
        "gui" | "--gui" => RunMode::Gui,
        "tui" | "--tui" => RunMode::Tui,
        _ => return None,
    };
    Some((game, mode))
}

pub fn create_adapter(game: GameKind) -> Box<dyn GameViewAdapter> {
    match game {
        GameKind::Xiangqi => Box::new(XiangqiAdapter::new()),
        GameKind::Gomoku => Box::new(GomokuAdapter::new()),
    }
}

pub fn prompt_game_and_mode() -> (GameKind, RunMode) {
    println!("请选择游戏:");
    println!("  1. 中国象棋 (xiangqi)");
    println!("  2. 五子棋 (gomoku)");
    print!("输入 1 或 2: ");
    let _ = std::io::Write::flush(&mut std::io::stdout());
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    let game = if line.trim() == "2" {
        GameKind::Gomoku
    } else {
        GameKind::Xiangqi
    };

    println!("请选择运行模式:");
    println!("  1. GUI (原生窗口)");
    println!("  2. TUI (终端界面)");
    print!("输入 1 或 2: ");
    let _ = std::io::Write::flush(&mut std::io::stdout());
    line.clear();
    std::io::stdin().read_line(&mut line).ok();
    let mode = if line.trim() == "2" {
        RunMode::Tui
    } else {
        RunMode::Gui
    };

    (game, mode)
}

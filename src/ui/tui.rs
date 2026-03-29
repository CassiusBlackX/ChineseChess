use std::{
    collections::{HashMap, HashSet},
    io,
    time::Duration,
};

use chinese_chess::{
    game::SnapshotDto,
    view_adapter::{GameViewAdapter, SharedGameAdapter, ViewInput, ViewOutput},
};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseButton,
        MouseEventKind,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

const BOARD_W: usize = 9;
const BOARD_H: usize = 10;
const CELL_W: u16 = 4;

struct TuiApp {
    adapter: SharedGameAdapter,
    snapshot: SnapshotDto,
    cursor_x: usize,
    cursor_y: usize,
    board_inner: Option<Rect>,
    should_quit: bool,
}

impl TuiApp {
    fn new() -> Self {
        let mut adapter = SharedGameAdapter::new();
        let snapshot = match adapter.handle(ViewInput::Snapshot) {
            ViewOutput::Snapshot(s) => s,
            ViewOutput::Moves(_) | ViewOutput::Error(_) => panic!("snapshot fetch failed"),
        };

        Self {
            adapter,
            snapshot,
            cursor_x: 4,
            cursor_y: 4,
            board_inner: None,
            should_quit: false,
        }
    }

    fn click_at(&mut self, x: usize, y: usize) {
        if let ViewOutput::Snapshot(s) = self.adapter.handle(ViewInput::Click { x, y }) {
            self.snapshot = s;
        }
    }

    fn reset(&mut self) {
        if let ViewOutput::Snapshot(s) = self.adapter.handle(ViewInput::Reset) {
            self.snapshot = s;
        }
    }

    fn handle_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('r') => self.reset(),
            KeyCode::Left => {
                self.cursor_x = self.cursor_x.saturating_sub(1);
            }
            KeyCode::Right => {
                self.cursor_x = (self.cursor_x + 1).min(BOARD_W - 1);
            }
            KeyCode::Up => {
                self.cursor_y = (self.cursor_y + 1).min(BOARD_H - 1);
            }
            KeyCode::Down => {
                self.cursor_y = self.cursor_y.saturating_sub(1);
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                if !self.snapshot.game_over {
                    self.click_at(self.cursor_x, self.cursor_y);
                }
            }
            _ => {}
        }
    }

    fn terminal_to_board(&self, col: u16, row: u16) -> Option<(usize, usize)> {
        let inner = self.board_inner?;
        if col < inner.x || row < inner.y {
            return None;
        }

        let rel_x = col - inner.x;
        let rel_y = row - inner.y;
        if rel_x >= CELL_W * BOARD_W as u16 || rel_y >= BOARD_H as u16 {
            return None;
        }

        let x = (rel_x / CELL_W) as usize;
        let board_row = rel_y as usize;
        let y = BOARD_H - 1 - board_row;
        Some((x, y))
    }

    fn handle_mouse(&mut self, kind: MouseEventKind, col: u16, row: u16) {
        if !matches!(kind, MouseEventKind::Down(MouseButton::Left)) {
            return;
        }
        if let Some((x, y)) = self.terminal_to_board(col, row) {
            self.cursor_x = x;
            self.cursor_y = y;
            if !self.snapshot.game_over {
                self.click_at(x, y);
            }
        }
    }

    fn draw(&mut self, f: &mut Frame<'_>) {
        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(12),
        ])
        .split(f.area());

        let turn_text = if self.snapshot.turn > 0 { "红方" } else { "黑方" };
        let mut status = format!("{} | 当前回合: {}", self.snapshot.message, turn_text);
        if self.snapshot.game_over {
            let winner = if self.snapshot.winner > 0 { "红方" } else { "黑方" };
            status.push_str(&format!(" | 对局结束: {}胜", winner));
        } else if self.snapshot.in_check_side != 0 {
            let checked = if self.snapshot.in_check_side > 0 { "红方" } else { "黑方" };
            status.push_str(&format!(" | 被将军: {}", checked));
        }

        let status_widget = Paragraph::new(status).block(
            Block::default()
                .borders(Borders::ALL)
                .title("中国象棋 TUI (ratatui)"),
        );
        f.render_widget(status_widget, layout[0]);

        let help = Paragraph::new(
            "方向键移动光标 | Enter/空格点击 | 鼠标左键点击 | r 重开 | q/Esc 退出",
        )
        .block(Block::default().borders(Borders::ALL).title("操作"));
        f.render_widget(help, layout[1]);

        let board_block = Block::default().borders(Borders::ALL).title("棋盘");
        let board_area = layout[2];
        let inner = board_block.inner(board_area);
        self.board_inner = Some(inner);

        let mut piece_map = HashMap::new();
        for piece in &self.snapshot.pieces {
            piece_map.insert((piece.x, piece.y), (piece.symbol.clone(), piece.side));
        }
        let mut legal_moves = HashSet::new();
        for mv in &self.snapshot.legal_moves {
            legal_moves.insert((mv.x, mv.y));
        }

        let mut rows = Vec::with_capacity(BOARD_H);
        for row_index in 0..BOARD_H {
            let y = BOARD_H - 1 - row_index;
            let mut cells = Vec::with_capacity(BOARD_W);
            for x in 0..BOARD_W {
                let is_cursor = x == self.cursor_x && y == self.cursor_y;
                let is_selected = self
                    .snapshot
                    .selected
                    .as_ref()
                    .map(|s| s.x == x && s.y == y)
                    .unwrap_or(false);
                let is_hint = legal_moves.contains(&(x, y));

                let mut style = Style::default().bg(Color::Rgb(242, 221, 185));
                if is_hint {
                    style = style.bg(Color::Rgb(58, 89, 58));
                }
                if is_selected {
                    style = style.bg(Color::Rgb(107, 79, 32));
                }
                if is_cursor {
                    style = style.bg(Color::Rgb(45, 75, 120));
                }

                let text = if let Some((symbol, side)) = piece_map.get(&(x, y)) {
                    if *side > 0 {
                        style = style.fg(Color::Rgb(220, 60, 60));
                    } else {
                        style = style.fg(Color::Rgb(36, 36, 36));
                    }
                    format!(" {} ", symbol)
                } else {
                    style = style.fg(Color::Rgb(120, 95, 70));
                    " ·  ".to_string()
                };

                cells.push(Cell::from(text).style(style));
            }
            rows.push(Row::new(cells));
        }

        let widths = [Constraint::Length(CELL_W); BOARD_W];
        let table = Table::new(rows, widths).block(board_block);
        f.render_widget(table, board_area);
    }
}

fn run_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = TuiApp::new();
    while !app.should_quit {
        terminal.draw(|f| app.draw(f))?;
        if event::poll(Duration::from_millis(150))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => app.handle_key(key.code),
                Event::Mouse(mouse) => app.handle_mouse(mouse.kind, mouse.column, mouse.row),
                _ => {}
            }
        }
    }
    Ok(())
}

pub fn run_tui() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_loop(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    result
}

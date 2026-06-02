use std::{
    collections::{HashMap, HashSet},
    io,
    time::Duration,
};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseButton,
        MouseEventKind,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use game_view::{
    AiDifficulty, GameViewAdapter, PlayMode, SnapshotDto, ViewInput, ViewOutput,
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use crate::ui::common::{format_session, format_status, human_input_enabled};

const CELL_W: u16 = 4;

struct TuiApp {
    adapter: Box<dyn GameViewAdapter>,
    snapshot: SnapshotDto,
    game_title: String,
    supports_session: bool,
    cursor_x: usize,
    cursor_y: usize,
    board_inner: Option<Rect>,
    should_quit: bool,
}

impl TuiApp {
    fn new(mut adapter: Box<dyn GameViewAdapter>) -> Self {
        let game_title = adapter.game_title().to_string();
        let supports_session = adapter.supports_session_config();
        let snapshot = match adapter.handle(ViewInput::Snapshot) {
            ViewOutput::Snapshot(s) => s,
            ViewOutput::Moves(_) | ViewOutput::Error(_) => panic!("snapshot fetch failed"),
        };
        let cursor_x = adapter.board_width() / 2;
        let cursor_y = adapter.board_height() / 2;

        Self {
            adapter,
            snapshot,
            game_title,
            supports_session,
            cursor_x,
            cursor_y,
            board_inner: None,
            should_quit: false,
        }
    }

    fn board_w(&self) -> usize {
        self.adapter.board_width()
    }

    fn board_h(&self) -> usize {
        self.adapter.board_height()
    }

    fn apply_session_input(&mut self, input: ViewInput) {
        if let ViewOutput::Snapshot(s) = self.adapter.handle(input) {
            self.snapshot = s;
        }
    }

    fn can_place(&self) -> bool {
        human_input_enabled(&self.snapshot) && !self.snapshot.game_over
    }

    fn click_at(&mut self, x: usize, y: usize) {
        if !self.can_place() {
            return;
        }
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
        let board_w = self.board_w();
        let board_h = self.board_h();
        match code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('r') => self.reset(),
            KeyCode::Char('m') if self.supports_session => self.cycle_play_mode(),
            KeyCode::Char('1') if self.supports_session => {
                self.apply_session_input(ViewInput::SetAiDifficulty(AiDifficulty::Easy));
            }
            KeyCode::Char('2') if self.supports_session => {
                self.apply_session_input(ViewInput::SetAiDifficulty(AiDifficulty::Medium));
            }
            KeyCode::Char('3') if self.supports_session => {
                self.apply_session_input(ViewInput::SetAiDifficulty(AiDifficulty::Hard));
            }
            KeyCode::Char('b') if self.supports_session => {
                self.apply_session_input(ViewInput::SetHumanSide(1));
            }
            KeyCode::Char('w') if self.supports_session => {
                self.apply_session_input(ViewInput::SetHumanSide(-1));
            }
            KeyCode::Left => {
                self.cursor_x = self.cursor_x.saturating_sub(1);
            }
            KeyCode::Right => {
                self.cursor_x = (self.cursor_x + 1).min(board_w - 1);
            }
            KeyCode::Up => {
                self.cursor_y = (self.cursor_y + 1).min(board_h - 1);
            }
            KeyCode::Down => {
                self.cursor_y = self.cursor_y.saturating_sub(1);
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                if self.can_place() {
                    self.click_at(self.cursor_x, self.cursor_y);
                }
            }
            _ => {}
        }
    }

    fn cycle_play_mode(&mut self) {
        let next = match self.snapshot.session.as_ref().map(|s| s.play_mode) {
            Some(PlayMode::LocalPvp) => PlayMode::HumanVsAi,
            _ => PlayMode::LocalPvp,
        };
        self.apply_session_input(ViewInput::SetPlayMode(next));
    }

    fn terminal_to_board(&self, col: u16, row: u16) -> Option<(usize, usize)> {
        let inner = self.board_inner?;
        let board_w = self.board_w();
        let board_h = self.board_h();
        if col < inner.x || row < inner.y {
            return None;
        }

        let rel_x = col - inner.x;
        let rel_y = row - inner.y;
        if rel_x >= CELL_W * board_w as u16 || rel_y >= board_h as u16 {
            return None;
        }

        let x = (rel_x / CELL_W) as usize;
        let board_row = rel_y as usize;
        let y = board_h - 1 - board_row;
        Some((x, y))
    }

    fn handle_mouse(&mut self, kind: MouseEventKind, col: u16, row: u16) {
        if !matches!(kind, MouseEventKind::Down(MouseButton::Left)) {
            return;
        }
        if let Some((x, y)) = self.terminal_to_board(col, row) {
            self.cursor_x = x;
            self.cursor_y = y;
            if self.can_place() {
                self.click_at(x, y);
            }
        }
    }

    fn draw(&mut self, f: &mut Frame<'_>) {
        let title = self.game_title.clone();
        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(if self.supports_session { 3 } else { 2 }),
            Constraint::Min(12),
        ])
        .split(f.area());

        let status_widget = Paragraph::new(format_status(&self.snapshot, &title)).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("{} TUI (ratatui)", title)),
        );
        f.render_widget(status_widget, layout[0]);

        let help_text = if self.supports_session {
            let session_hint = self
                .snapshot
                .session
                .as_ref()
                .map(format_session)
                .unwrap_or_default();
            format!(
                "{session_hint}\n方向键移动 | Enter/空格/鼠标落子 | m 切换模式 | 1/2/3 难度 | b/w 执棋 | r 重开 | q 退出"
            )
        } else {
            "方向键移动光标 | Enter/空格点击 | 鼠标左键点击 | r 重开 | q/Esc 退出".to_string()
        };
        let help = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title("操作"));
        f.render_widget(help, layout[1]);

        let board_block = Block::default().borders(Borders::ALL).title("棋盘");
        let board_area = layout[2];
        let inner = board_block.inner(board_area);
        self.board_inner = Some(inner);

        let board_w = self.board_w();
        let board_h = self.board_h();

        let mut piece_map = HashMap::new();
        for piece in &self.snapshot.pieces {
            piece_map.insert((piece.x, piece.y), (piece.symbol.clone(), piece.side));
        }
        let mut legal_moves = HashSet::new();
        for mv in &self.snapshot.legal_moves {
            legal_moves.insert((mv.x, mv.y));
        }
        let last_move = self
            .snapshot
            .last_move
            .as_ref()
            .map(|m| (m.x, m.y));

        let mut rows = Vec::with_capacity(board_h);
        for row_index in 0..board_h {
            let y = board_h - 1 - row_index;
            let mut cells = Vec::with_capacity(board_w);
            for x in 0..board_w {
                let is_cursor = x == self.cursor_x && y == self.cursor_y;
                let is_selected = self
                    .snapshot
                    .selected
                    .as_ref()
                    .map(|s| s.x == x && s.y == y)
                    .unwrap_or(false);
                let is_hint = legal_moves.contains(&(x, y));
                let is_last = last_move == Some((x, y));

                let mut style = Style::default().bg(Color::Rgb(242, 221, 185));
                if is_hint {
                    style = style.bg(Color::Rgb(58, 89, 58));
                }
                if is_selected {
                    style = style.bg(Color::Rgb(107, 79, 32));
                }
                if is_last {
                    style = style.bg(Color::Rgb(45, 75, 120));
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

        let widths = vec![Constraint::Length(CELL_W); board_w];
        let table = Table::new(rows, widths).block(board_block);
        f.render_widget(table, board_area);
    }
}

pub fn run_tui(adapter: Box<dyn GameViewAdapter>) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = TuiApp::new(adapter);
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

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

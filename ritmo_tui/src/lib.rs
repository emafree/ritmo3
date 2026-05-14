pub mod app;
pub mod screens;
pub mod widgets;

use std::{
    io::stdout,
    time::Duration,
};

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use ritmo_errors::RitmoResult;

pub use app::{AppAction, AppState, MainWindow, ScreenLevel};

const EVENT_POLL_TIMEOUT: Duration = Duration::from_millis(250);

struct TerminalSession;

impl TerminalSession {
    fn start() -> RitmoResult<Self> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
    }
}

pub fn run() -> RitmoResult<()> {
    let _session = TerminalSession::start()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut app_state = AppState::default();

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let status = format!(
                "Window: {:?} | Level: {:?} | q per uscire",
                app_state.main_window, app_state.level
            );
            frame.render_widget(
                Paragraph::new(status)
                    .block(Block::default().borders(Borders::ALL).title("Ritmo")),
                area,
            );
        })?;

        if event::poll(EVENT_POLL_TIMEOUT)? {
            if let Event::Key(key) = event::read()? {
                if matches!(app_state.handle_key(key), AppAction::Quit) {
                    break;
                }
            }
        }
    }

    terminal.show_cursor()?;
    Ok(())
}

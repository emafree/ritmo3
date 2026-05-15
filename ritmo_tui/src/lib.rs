pub mod app;
pub mod screens;
pub mod widgets;

use std::{io::stdout, time::Duration};

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use ritmo_errors::RitmoResult;
use sqlx::SqlitePool;

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

pub async fn run(pool: SqlitePool) -> RitmoResult<()> {
    let _session = TerminalSession::start()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut app_state = AppState::new(pool).await?;

    loop {
        terminal.draw(|frame| app_state.render(frame))?;

        if event::poll(EVENT_POLL_TIMEOUT)? {
            if let Event::Key(key) = event::read()? {
                let action = app_state.handle_key(key);
                match action {
                    AppAction::NewRecord => {
                        app_state.open_create_screen();
                    }
                    AppAction::DeleteRecord => {
                        // TODO: open confirmation popup
                    }
                    AppAction::EnterLevel => {
                        // TODO: load and show detail
                    }
                    AppAction::Search => {
                        // TODO: activate search bar
                    }
                    _ => {}
                }
                if app_state.should_quit() {
                    break;
                }
            }
        }
    }

    terminal.show_cursor()?;
    Ok(())
}

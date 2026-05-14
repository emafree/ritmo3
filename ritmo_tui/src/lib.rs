pub mod app;
pub mod screens;
pub mod widgets;

use ritmo_errors::RitmoResult;

pub use app::{AppAction, AppState, MainWindow, ScreenLevel};

pub fn run() -> RitmoResult<()> {
    let _ = AppState::default();
    Ok(())
}

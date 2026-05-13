use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainWindow {
    Filters,
    Books,
    Contents,
}

impl MainWindow {
    fn previous(self) -> Self {
        match self {
            Self::Filters => Self::Contents,
            Self::Books => Self::Filters,
            Self::Contents => Self::Books,
        }
    }

    fn next(self) -> Self {
        match self {
            Self::Filters => Self::Books,
            Self::Books => Self::Contents,
            Self::Contents => Self::Filters,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenLevel {
    List,
    Detail,
    Editing,
    Popup,
}

impl ScreenLevel {
    fn descend(self) -> Self {
        match self {
            Self::List => Self::Detail,
            Self::Detail => Self::Editing,
            Self::Editing => Self::Popup,
            Self::Popup => Self::Popup,
        }
    }

    fn ascend(self) -> Self {
        match self {
            Self::List => Self::List,
            Self::Detail => Self::List,
            Self::Editing => Self::Detail,
            Self::Popup => Self::Editing,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppAction {
    None,
    Quit,
    SwitchWindow(MainWindow),
    ScrollUp,
    ScrollDown,
    EnterLevel,
    ExitLevel,
    Search,
    NewRecord,
    EditRecord,
    DeleteRecord,
    ToggleFilterSet,
    ConfirmPopup,
    CancelPopup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppState {
    pub main_window: MainWindow,
    pub level: ScreenLevel,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            main_window: MainWindow::Filters,
            level: ScreenLevel::List,
        }
    }
}

impl AppState {
    pub fn handle_key(&mut self, key: KeyEvent) -> AppAction {
        if self.level == ScreenLevel::Popup {
            return match key.code {
                KeyCode::Enter => AppAction::ConfirmPopup,
                KeyCode::Esc => {
                    self.level = self.level.ascend();
                    AppAction::CancelPopup
                }
                _ => AppAction::None,
            };
        }

        match key.code {
            KeyCode::Char('q') if self.level == ScreenLevel::List => AppAction::Quit,
            KeyCode::Left if self.level == ScreenLevel::List => {
                self.main_window = self.main_window.previous();
                AppAction::SwitchWindow(self.main_window)
            }
            KeyCode::Right if self.level == ScreenLevel::List => {
                self.main_window = self.main_window.next();
                AppAction::SwitchWindow(self.main_window)
            }
            KeyCode::Char('f') if self.level == ScreenLevel::List => {
                self.main_window = MainWindow::Filters;
                AppAction::SwitchWindow(self.main_window)
            }
            KeyCode::Char('b') if self.level == ScreenLevel::List => {
                self.main_window = MainWindow::Books;
                AppAction::SwitchWindow(self.main_window)
            }
            KeyCode::Char('c') if self.level == ScreenLevel::List => {
                self.main_window = MainWindow::Contents;
                AppAction::SwitchWindow(self.main_window)
            }
            KeyCode::Up | KeyCode::Char('k') => AppAction::ScrollUp,
            KeyCode::Down | KeyCode::Char('j') => AppAction::ScrollDown,
            KeyCode::Enter => {
                self.level = self.level.descend();
                AppAction::EnterLevel
            }
            KeyCode::Esc => {
                self.level = self.level.ascend();
                AppAction::ExitLevel
            }
            KeyCode::Char('/') => AppAction::Search,
            KeyCode::Char('n') | KeyCode::Char('+') => AppAction::NewRecord,
            KeyCode::Char('e') => AppAction::EditRecord,
            KeyCode::Char('d') | KeyCode::Delete => AppAction::DeleteRecord,
            KeyCode::Char(' ')
                if self.main_window == MainWindow::Filters && self.level == ScreenLevel::List =>
            {
                AppAction::ToggleFilterSet
            }
            _ => AppAction::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::from(code)
    }

    #[test]
    fn fbc_are_only_enabled_on_list_level() {
        let mut app = AppState::default();
        assert_eq!(
            app.handle_key(key(KeyCode::Char('b'))),
            AppAction::SwitchWindow(MainWindow::Books)
        );
        assert_eq!(app.main_window, MainWindow::Books);

        app.level = ScreenLevel::Detail;
        assert_eq!(app.handle_key(key(KeyCode::Char('f'))), AppAction::None);
        assert_eq!(app.main_window, MainWindow::Books);
    }

    #[test]
    fn q_only_quits_on_list_level() {
        let mut app = AppState::default();
        assert_eq!(app.handle_key(key(KeyCode::Char('q'))), AppAction::Quit);

        app.level = ScreenLevel::Detail;
        assert_eq!(app.handle_key(key(KeyCode::Char('q'))), AppAction::None);
    }

    #[test]
    fn popup_keys_confirm_or_cancel() {
        let mut app = AppState {
            main_window: MainWindow::Filters,
            level: ScreenLevel::Popup,
        };

        assert_eq!(app.handle_key(key(KeyCode::Enter)), AppAction::ConfirmPopup);
        assert_eq!(app.level, ScreenLevel::Popup);

        assert_eq!(app.handle_key(key(KeyCode::Esc)), AppAction::CancelPopup);
        assert_eq!(app.level, ScreenLevel::Editing);
    }

    #[test]
    fn arrows_cycle_main_windows_from_list_level() {
        let mut app = AppState::default();
        assert_eq!(
            app.handle_key(key(KeyCode::Right)),
            AppAction::SwitchWindow(MainWindow::Books)
        );
        assert_eq!(
            app.handle_key(key(KeyCode::Right)),
            AppAction::SwitchWindow(MainWindow::Contents)
        );
        assert_eq!(
            app.handle_key(key(KeyCode::Right)),
            AppAction::SwitchWindow(MainWindow::Filters)
        );
        assert_eq!(
            app.handle_key(key(KeyCode::Left)),
            AppAction::SwitchWindow(MainWindow::Contents)
        );
    }
}

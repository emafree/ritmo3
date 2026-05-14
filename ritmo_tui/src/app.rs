use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout};
use ratatui::{
    prelude::Frame,
    widgets::{Block, Borders, Paragraph},
};
use ritmo_core::CoreContext;
use ritmo_errors::RitmoResult;
use ritmo_presenter::{
    build_book_detail, build_content_detail, build_person_role_views, BookDetail, ContentDetail,
};
use sqlx::SqlitePool;

use crate::screens::books::list::BookListScreen;
use crate::screens::contents::list::ContentListScreen;
use crate::widgets::statusbar::StatusBar;
use crate::widgets::table::TableAction;

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

pub struct AppState {
    pub pool: SqlitePool,
    pub books: Vec<BookDetail>,
    pub contents: Vec<ContentDetail>,
    pub main_window: MainWindow,
    pub level: ScreenLevel,
    book_list: BookListScreen,
    content_list: ContentListScreen,
    statusbar: StatusBar,
    should_quit: bool,
}

async fn load_books(ctx: &CoreContext) -> RitmoResult<Vec<BookDetail>> {
    let books = ritmo_core::book::list_all(ctx).await?;
    let mut result = Vec::new();
    for book in books {
        let people_roles = ritmo_core::book::list_people_with_roles(ctx, book.id).await?;
        let tags = ritmo_core::book::list_tags(ctx, book.id).await?;
        let format = ritmo_core::book::get_format_name(ctx, book.id).await?;
        let series = ritmo_core::book::get_series_name(ctx, book.id).await?;
        let views = build_person_role_views(&people_roles);
        result.push(build_book_detail(book, views, tags, vec![], format, series));
    }
    Ok(result)
}

async fn load_contents(ctx: &CoreContext) -> RitmoResult<Vec<ContentDetail>> {
    let contents = ritmo_core::content::list_all(ctx).await?;
    let mut result = Vec::new();
    for content in contents {
        let people_roles = ritmo_core::content::list_people_with_roles(ctx, content.id).await?;
        let tags = ritmo_core::content::list_tags(ctx, content.id).await?;
        let languages = ritmo_core::content::list_languages(ctx, content.id).await?;
        let genre = ritmo_core::content::get_genre_name(ctx, content.id).await?;
        let views = build_person_role_views(&people_roles);
        result.push(build_content_detail(
            content,
            views,
            tags,
            vec![],
            languages,
            genre,
        ));
    }
    Ok(result)
}

impl AppState {
    pub async fn new(pool: SqlitePool) -> RitmoResult<Self> {
        let ctx = CoreContext::from_pool(pool.clone());

        let books = load_books(&ctx).await?;
        let contents = load_contents(&ctx).await?;

        let book_list = BookListScreen::new(&books);
        let content_list = ContentListScreen::new(&contents);

        Ok(Self {
            pool,
            books,
            contents,
            main_window: MainWindow::Filters,
            level: ScreenLevel::List,
            book_list,
            content_list,
            statusbar: StatusBar::new(),
            should_quit: false,
        })
    }

    pub async fn reload_book(&mut self, id: i64) -> RitmoResult<()> {
        let ctx = CoreContext::from_pool(self.pool.clone());
        let book = ritmo_core::book::get(&ctx, id).await?;
        let people_roles = ritmo_core::book::list_people_with_roles(&ctx, id).await?;
        let tags = ritmo_core::book::list_tags(&ctx, id).await?;
        let format = ritmo_core::book::get_format_name(&ctx, id).await?;
        let series = ritmo_core::book::get_series_name(&ctx, id).await?;
        let views = build_person_role_views(&people_roles);
        let detail = build_book_detail(book, views, tags, vec![], format, series);
        if let Some(pos) = self.books.iter().position(|b| b.book.id == id) {
            self.books[pos] = detail;
        } else {
            self.books.push(detail);
        }
        self.book_list = BookListScreen::new(&self.books);
        Ok(())
    }

    pub async fn reload_content(&mut self, id: i64) -> RitmoResult<()> {
        let ctx = CoreContext::from_pool(self.pool.clone());
        let content = ritmo_core::content::get(&ctx, id).await?;
        let people_roles = ritmo_core::content::list_people_with_roles(&ctx, id).await?;
        let tags = ritmo_core::content::list_tags(&ctx, id).await?;
        let languages = ritmo_core::content::list_languages(&ctx, id).await?;
        let genre = ritmo_core::content::get_genre_name(&ctx, id).await?;
        let views = build_person_role_views(&people_roles);
        let detail = build_content_detail(content, views, tags, vec![], languages, genre);
        if let Some(pos) = self.contents.iter().position(|c| c.content.id == id) {
            self.contents[pos] = detail;
        } else {
            self.contents.push(detail);
        }
        self.content_list = ContentListScreen::new(&self.contents);
        Ok(())
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let chunks =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(frame.area());

        match self.main_window {
            MainWindow::Books => {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .title("Ritmo — Libri");
                let inner = block.inner(chunks[0]);
                frame.render_widget(block, chunks[0]);
                self.book_list.render(frame, inner);
            }
            MainWindow::Contents => {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .title("Ritmo — Contenuti");
                let inner = block.inner(chunks[0]);
                frame.render_widget(block, chunks[0]);
                self.content_list.render(frame, inner);
            }
            MainWindow::Filters => {
                let content = Paragraph::new("Filters — da implementare");
                frame.render_widget(
                    content.block(Block::default().borders(Borders::ALL).title("Ritmo")),
                    chunks[0],
                );
            }
        }

        let status = format!(
            " Window: {:?} | Level: {:?} | q: esci | f/b/c: cambia finestra",
            self.main_window, self.level
        );
        frame.render_widget(Paragraph::new(status), chunks[1]);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> AppAction {
        let action = if self.level == ScreenLevel::Popup {
            match key.code {
                KeyCode::Enter => AppAction::ConfirmPopup,
                KeyCode::Esc => {
                    self.level = self.level.ascend();
                    AppAction::CancelPopup
                }
                _ => AppAction::None,
            }
        } else {
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
                KeyCode::Esc => {
                    self.level = self.level.ascend();
                    AppAction::ExitLevel
                }
                KeyCode::Char('e') => AppAction::EditRecord,
                KeyCode::Char(' ')
                    if self.main_window == MainWindow::Filters
                        && self.level == ScreenLevel::List =>
                {
                    AppAction::ToggleFilterSet
                }
                _ => self.delegate_to_table(key),
            }
        };

        if matches!(action, AppAction::Quit) {
            self.should_quit = true;
        }

        action
    }

    fn delegate_to_table(&mut self, key: KeyEvent) -> AppAction {
        let table_action = match self.main_window {
            MainWindow::Books => self.book_list.table.handle_key(key),
            MainWindow::Contents => self.content_list.table.handle_key(key),
            MainWindow::Filters => TableAction::None,
        };

        match table_action {
            TableAction::ScrollUp => {
                self.update_statusbar_after_scroll();
                AppAction::ScrollUp
            }
            TableAction::ScrollDown => {
                self.update_statusbar_after_scroll();
                AppAction::ScrollDown
            }
            TableAction::Select => {
                self.level = self.level.descend();
                AppAction::EnterLevel
            }
            TableAction::New => AppAction::NewRecord,
            TableAction::Delete => AppAction::DeleteRecord,
            TableAction::Search => AppAction::Search,
            TableAction::None => AppAction::None,
        }
    }

    fn update_statusbar_after_scroll(&mut self) {
        match self.main_window {
            MainWindow::Books => {
                let total = self.book_list.items.len();
                let selected = if total == 0 {
                    0
                } else {
                    self.book_list.table.selected_index() + 1
                };
                self.statusbar
                    .set_info(format!("Book {selected} of {total}"));
            }
            MainWindow::Contents => {
                let total = self.content_list.items.len();
                let selected = if total == 0 {
                    0
                } else {
                    self.content_list.table.selected_index() + 1
                };
                self.statusbar
                    .set_info(format!("Contenuto {selected} di {total}"));
            }
            MainWindow::Filters => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::from(code)
    }

    async fn make_test_state() -> AppState {
        let pool = ritmo_db::create_sqlite_pool("sqlite::memory:")
            .await
            .expect("in-memory sqlite with schema");
        AppState::new(pool).await.expect("AppState::new")
    }

    #[tokio::test]
    async fn fbc_are_only_enabled_on_list_level() {
        let mut app = make_test_state().await;
        assert_eq!(
            app.handle_key(key(KeyCode::Char('b'))),
            AppAction::SwitchWindow(MainWindow::Books)
        );
        assert_eq!(app.main_window, MainWindow::Books);

        app.level = ScreenLevel::Detail;
        assert_eq!(app.handle_key(key(KeyCode::Char('f'))), AppAction::None);
        assert_eq!(app.main_window, MainWindow::Books);
    }

    #[tokio::test]
    async fn q_only_quits_on_list_level() {
        let mut app = make_test_state().await;
        assert_eq!(app.handle_key(key(KeyCode::Char('q'))), AppAction::Quit);

        let mut app2 = make_test_state().await;
        app2.level = ScreenLevel::Detail;
        assert_eq!(app2.handle_key(key(KeyCode::Char('q'))), AppAction::None);
    }

    #[tokio::test]
    async fn popup_keys_confirm_or_cancel() {
        let mut app = make_test_state().await;
        app.level = ScreenLevel::Popup;

        assert_eq!(app.handle_key(key(KeyCode::Enter)), AppAction::ConfirmPopup);
        assert_eq!(app.level, ScreenLevel::Popup);

        assert_eq!(app.handle_key(key(KeyCode::Esc)), AppAction::CancelPopup);
        assert_eq!(app.level, ScreenLevel::Editing);
    }

    #[tokio::test]
    async fn arrows_cycle_main_windows_from_list_level() {
        let mut app = make_test_state().await;
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

    #[tokio::test]
    async fn should_quit_is_set_after_quit_action() {
        let mut app = make_test_state().await;
        assert!(!app.should_quit());
        assert_eq!(app.handle_key(key(KeyCode::Char('q'))), AppAction::Quit);
        assert!(app.should_quit());
    }

    #[tokio::test]
    async fn table_scroll_keys_delegated_to_active_table() {
        let mut app = make_test_state().await;
        app.main_window = MainWindow::Books;

        assert_eq!(app.handle_key(key(KeyCode::Down)), AppAction::ScrollDown);
        assert_eq!(
            app.handle_key(key(KeyCode::Char('j'))),
            AppAction::ScrollDown
        );
        assert_eq!(app.handle_key(key(KeyCode::Up)), AppAction::ScrollUp);
        assert_eq!(app.handle_key(key(KeyCode::Char('k'))), AppAction::ScrollUp);

        app.main_window = MainWindow::Contents;
        assert_eq!(app.handle_key(key(KeyCode::Down)), AppAction::ScrollDown);
        assert_eq!(app.handle_key(key(KeyCode::Up)), AppAction::ScrollUp);
    }

    #[tokio::test]
    async fn table_action_keys_map_to_app_actions() {
        let mut app = make_test_state().await;
        app.main_window = MainWindow::Books;

        assert_eq!(
            app.handle_key(key(KeyCode::Char('n'))),
            AppAction::NewRecord
        );
        assert_eq!(
            app.handle_key(key(KeyCode::Char('+'))),
            AppAction::NewRecord
        );
        assert_eq!(
            app.handle_key(key(KeyCode::Char('d'))),
            AppAction::DeleteRecord
        );
        assert_eq!(
            app.handle_key(key(KeyCode::Delete)),
            AppAction::DeleteRecord
        );
        assert_eq!(app.handle_key(key(KeyCode::Char('/'))), AppAction::Search);
    }

    #[tokio::test]
    async fn enter_key_descends_level_via_table_select() {
        let mut app = make_test_state().await;
        app.main_window = MainWindow::Books;
        assert_eq!(app.level, ScreenLevel::List);

        assert_eq!(app.handle_key(key(KeyCode::Enter)), AppAction::EnterLevel);
        assert_eq!(app.level, ScreenLevel::Detail);
    }
}

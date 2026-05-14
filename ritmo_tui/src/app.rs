use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout};
use ratatui::{
    prelude::Frame,
    widgets::{Block, Borders, Paragraph},
};
use ritmo_errors::RitmoResult;
use ritmo_presenter::{BookDetail, ContentDetail};
use ritmo_repository::{BookRepository, ContentRepository, RepositoryContext};
use sqlx::SqlitePool;

use crate::screens::{books::list::BookListScreen, contents::list::ContentListScreen};

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

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub books: Vec<BookDetail>,
    pub contents: Vec<ContentDetail>,
    pub main_window: MainWindow,
    pub level: ScreenLevel,
    should_quit: bool,
}

impl AppState {
    pub async fn new(pool: SqlitePool) -> RitmoResult<Self> {
        let ctx = RepositoryContext::new(pool.clone());
        let book_repo = BookRepository::new(&ctx);
        let content_repo = ContentRepository::new(&ctx);

        let mut books = Vec::new();
        for book in book_repo.list_all().await? {
            let detail = book_repo.get(book.id).await?;
            books.push(present_book_detail(detail));
        }

        let mut contents = Vec::new();
        for content in content_repo.list_all().await? {
            let detail = content_repo.get(content.id).await?;
            contents.push(present_content_detail(detail));
        }

        Ok(Self {
            pool,
            books,
            contents,
            main_window: MainWindow::Filters,
            level: ScreenLevel::List,
            should_quit: false,
        })
    }

    pub async fn reload_book(&mut self, id: i64) -> RitmoResult<()> {
        let ctx = RepositoryContext::new(self.pool.clone());
        let repo = BookRepository::new(&ctx);
        let book = present_book_detail(repo.get(id).await?);

        if let Some(index) = self.books.iter().position(|item| item.book.id == id) {
            self.books[index] = book;
        } else {
            self.books.push(book);
        }

        Ok(())
    }

    pub async fn reload_content(&mut self, id: i64) -> RitmoResult<()> {
        let ctx = RepositoryContext::new(self.pool.clone());
        let repo = ContentRepository::new(&ctx);
        let content = present_content_detail(repo.get(id).await?);

        if let Some(index) = self.contents.iter().position(|item| item.content.id == id) {
            self.contents[index] = content;
        } else {
            self.contents.push(content);
        }

        Ok(())
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn render(&self, frame: &mut Frame) {
        let chunks =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(frame.area());

        // Contenuto principale
        match self.main_window {
            MainWindow::Books => {
                let mut screen = BookListScreen::new(&self.books);
                screen.render(frame, chunks[0]);
            }
            MainWindow::Contents => {
                let mut screen = ContentListScreen::new(&self.contents);
                screen.render(frame, chunks[0]);
            }
            MainWindow::Filters => frame.render_widget(
                Paragraph::new("Filters — da implementare")
                    .block(Block::default().borders(Borders::ALL).title("Ritmo")),
                chunks[0],
            ),
        }

        // Statusbar in basso
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
                    if self.main_window == MainWindow::Filters
                        && self.level == ScreenLevel::List =>
                {
                    AppAction::ToggleFilterSet
                }
                _ => AppAction::None,
            }
        };

        if matches!(action, AppAction::Quit) {
            self.should_quit = true;
        }

        action
    }
}

fn present_book_detail(book: ritmo_domain::Book) -> BookDetail {
    BookDetail {
        book,
        linked_contents: Vec::new(),
        people_with_roles: Vec::new(),
        tags: Vec::new(),
    }
}

fn present_content_detail(content: ritmo_domain::Content) -> ContentDetail {
    ContentDetail {
        content,
        linked_books: Vec::new(),
        people_with_roles: Vec::new(),
        tags: Vec::new(),
        languages: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ritmo_domain::{Book, Content};
    use ritmo_repository::create_pool;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::from(code)
    }

    fn app_state() -> AppState {
        AppState {
            pool: SqlitePool::connect_lazy("sqlite::memory:")
                .expect("sqlite::memory: must be a valid sqlite url"),
            books: vec![],
            contents: vec![],
            main_window: MainWindow::Filters,
            level: ScreenLevel::List,
            should_quit: false,
        }
    }

    #[tokio::test]
    async fn fbc_are_only_enabled_on_list_level() {
        let mut app = app_state();
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
        let mut app = app_state();
        assert_eq!(app.handle_key(key(KeyCode::Char('q'))), AppAction::Quit);

        app.level = ScreenLevel::Detail;
        assert_eq!(app.handle_key(key(KeyCode::Char('q'))), AppAction::None);
    }

    #[tokio::test]
    async fn popup_keys_confirm_or_cancel() {
        let mut app = app_state();
        app.level = ScreenLevel::Popup;

        assert_eq!(app.handle_key(key(KeyCode::Enter)), AppAction::ConfirmPopup);
        assert_eq!(app.level, ScreenLevel::Popup);

        assert_eq!(app.handle_key(key(KeyCode::Esc)), AppAction::CancelPopup);
        assert_eq!(app.level, ScreenLevel::Editing);
    }

    #[tokio::test]
    async fn arrows_cycle_main_windows_from_list_level() {
        let mut app = app_state();
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
        let mut app = app_state();
        assert!(!app.should_quit());
        assert_eq!(app.handle_key(key(KeyCode::Char('q'))), AppAction::Quit);
        assert!(app.should_quit());
    }

    #[tokio::test]
    async fn new_loads_books_and_contents_from_repository() {
        let pool = create_pool("sqlite::memory:")
            .await
            .expect("in-memory pool should be created");
        let ctx = RepositoryContext::new(pool.clone());

        let book_repo = BookRepository::new(&ctx);
        let content_repo = ContentRepository::new(&ctx);

        let book_id = book_repo
            .save(&Book {
                id: 0,
                title: "Dune".to_string(),
                isbn: None,
                publication_year: None,
                notes: None,
            })
            .await
            .expect("book should be saved");
        let content_id = content_repo
            .save(&Content {
                id: 0,
                title: "Messiah".to_string(),
                publication_year: None,
                notes: None,
            })
            .await
            .expect("content should be saved");

        let mut state = AppState::new(pool)
            .await
            .expect("state should load data from db");

        assert!(state.books.iter().any(|book| book.book.id == book_id));
        assert!(
            state
                .contents
                .iter()
                .any(|content| content.content.id == content_id)
        );

        book_repo
            .update(&Book {
                id: book_id,
                title: "Dune (Updated)".to_string(),
                isbn: None,
                publication_year: None,
                notes: None,
            })
            .await
            .expect("book should be updated");
        content_repo
            .update(&Content {
                id: content_id,
                title: "Messiah (Updated)".to_string(),
                publication_year: None,
                notes: None,
            })
            .await
            .expect("content should be updated");

        state.reload_book(book_id).await.expect("book should reload");
        state
            .reload_content(content_id)
            .await
            .expect("content should reload");

        assert!(
            state
                .books
                .iter()
                .any(|book| book.book.id == book_id && book.book.title == "Dune (Updated)")
        );
        assert!(
            state.contents.iter().any(|content| {
                content.content.id == content_id && content.content.title == "Messiah (Updated)"
            })
        );
    }
}

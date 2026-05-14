use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Rect, prelude::Frame};
use ritmo_presenter::BookDetail;

use crate::widgets::{statusbar::StatusBar, table::TableWidget};

#[derive(Debug, Clone)]
pub struct BookListScreen {
    pub table: TableWidget,
    pub items: Vec<BookDetail>,
}

impl BookListScreen {
    pub fn new(items: &[BookDetail]) -> Self {
        let rows = items
            .iter()
            .map(|item| {
                vec![
                    item.book.title.clone(),
                    item.people_with_roles
                        .iter()
                        .map(|person| person.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", "),
                    String::new(),
                    String::new(),
                ]
            })
            .collect();

        let table = TableWidget::new(
            vec![
                "Title".to_string(),
                "Authors".to_string(),
                "Format".to_string(),
                "Series".to_string(),
            ],
            rows,
        );

        Self {
            table,
            items: items.to_vec(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent, statusbar: &mut StatusBar) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.table.previous(),
            KeyCode::Down | KeyCode::Char('j') => self.table.next(self.items.len()),
            _ => return,
        }

        self.update_statusbar_info(statusbar);
    }

    pub fn selected_id(&self) -> Option<i64> {
        self.items
            .get(self.table.selected_index())
            .map(|item| item.book.id)
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.table.render(frame, area);
    }

    fn update_statusbar_info(&self, statusbar: &mut StatusBar) {
        let total = self.items.len();
        let selected = if total == 0 {
            0
        } else {
            self.table.selected_index() + 1
        };
        statusbar.set_info(format!("Book {selected} of {total}"));
    }
}

#[cfg(test)]
mod tests {
    use super::BookListScreen;
    use crate::widgets::statusbar::StatusBar;
    use crossterm::event::{KeyCode, KeyEvent};
    use ritmo_domain::Book;
    use ritmo_presenter::BookDetail;

    fn item(id: i64, title: &str) -> BookDetail {
        BookDetail {
            book: Book {
                id,
                title: title.to_string(),
                isbn: None,
                publication_year: None,
                notes: None,
            },
            linked_contents: vec![],
            people_with_roles: vec![],
            tags: vec![],
        }
    }

    #[test]
    fn new_builds_table_headers_and_rows() {
        let items = vec![item(1, "Dune")];
        let screen = BookListScreen::new(&items);

        assert_eq!(
            screen.table.headers,
            vec!["Title", "Authors", "Format", "Series"]
        );
        assert_eq!(
            screen.table.rows,
            vec![vec![
                "Dune".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string()
            ]]
        );
    }

    #[test]
    fn handle_key_moves_selection_and_updates_statusbar_info() {
        let items = vec![item(1, "A"), item(2, "B")];
        let mut screen = BookListScreen::new(&items);
        let mut statusbar = StatusBar::new();

        screen.handle_key(KeyEvent::from(KeyCode::Down), &mut statusbar);
        assert_eq!(screen.table.selected_index(), 1);
        assert_eq!(statusbar.info, "Book 2 of 2");

        screen.handle_key(KeyEvent::from(KeyCode::Char('k')), &mut statusbar);
        assert_eq!(screen.table.selected_index(), 0);
        assert_eq!(statusbar.info, "Book 1 of 2");
    }

    #[test]
    fn selected_id_returns_selected_book_id() {
        let items = vec![item(10, "A"), item(20, "B")];
        let mut screen = BookListScreen::new(&items);

        assert_eq!(screen.selected_id(), Some(10));
        screen.table.next(screen.items.len());
        assert_eq!(screen.selected_id(), Some(20));
    }

    #[test]
    fn selected_id_is_none_when_there_are_no_items() {
        let screen = BookListScreen::new(&[]);

        assert_eq!(screen.selected_id(), None);
    }
}

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Rect, prelude::Frame};
use ritmo_presenter::ContentDetail;

use crate::widgets::{statusbar::StatusBar, table::TableWidget};

#[derive(Debug, Clone)]
pub struct ContentListScreen {
    pub table: TableWidget,
    pub items: Vec<ContentDetail>,
}

impl ContentListScreen {
    pub fn new(items: &[ContentDetail]) -> Self {
        let rows = items
            .iter()
            .map(|item| {
                vec![
                    item.content.title.clone(),
                    item.people_with_roles
                        .iter()
                        .map(|person| person.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", "),
                    String::new(),
                ]
            })
            .collect();

        let table = TableWidget::new(
            vec![
                "Titolo".to_string(),
                "Autori".to_string(),
                "Genere".to_string(),
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
            .map(|item| item.content.id)
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
        statusbar.set_info(format!("Contenuto {selected} di {total}"));
    }
}

#[cfg(test)]
mod tests {
    use super::ContentListScreen;
    use crate::widgets::statusbar::StatusBar;
    use crossterm::event::{KeyCode, KeyEvent};
    use ritmo_domain::Content;
    use ritmo_presenter::ContentDetail;

    fn item(id: i64, title: &str) -> ContentDetail {
        ContentDetail {
            content: Content {
                id,
                title: title.to_string(),
                publication_year: None,
                notes: None,
            },
            linked_books: vec![],
            people_with_roles: vec![],
            tags: vec![],
            languages: vec![],
        }
    }

    #[test]
    fn new_builds_table_headers_and_rows() {
        let items = vec![item(1, "Il Nome della Rosa")];
        let screen = ContentListScreen::new(&items);

        assert_eq!(screen.table.headers, vec!["Titolo", "Autori", "Genere"]);
        assert_eq!(
            screen.table.rows,
            vec![vec![
                "Il Nome della Rosa".to_string(),
                "".to_string(),
                "".to_string(),
            ]]
        );
    }

    #[test]
    fn handle_key_moves_selection_and_updates_statusbar_info() {
        let items = vec![item(1, "A"), item(2, "B")];
        let mut screen = ContentListScreen::new(&items);
        let mut statusbar = StatusBar::new();

        screen.handle_key(KeyEvent::from(KeyCode::Down), &mut statusbar);
        assert_eq!(screen.table.selected_index(), 1);
        assert_eq!(statusbar.info, "Contenuto 2 di 2");

        screen.handle_key(KeyEvent::from(KeyCode::Char('k')), &mut statusbar);
        assert_eq!(screen.table.selected_index(), 0);
        assert_eq!(statusbar.info, "Contenuto 1 di 2");
    }

    #[test]
    fn selected_id_returns_selected_content_id() {
        let items = vec![item(10, "A"), item(20, "B")];
        let mut screen = ContentListScreen::new(&items);

        assert_eq!(screen.selected_id(), Some(10));
        screen.table.next(screen.items.len());
        assert_eq!(screen.selected_id(), Some(20));
    }

    #[test]
    fn selected_id_is_none_when_there_are_no_items() {
        let screen = ContentListScreen::new(&[]);

        assert_eq!(screen.selected_id(), None);
    }
}

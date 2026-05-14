use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::Frame,
    layout::Rect,
};
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
            .map(|detail| {
                let authors: Vec<String> = detail
                    .people_with_roles
                    .iter()
                    .filter(|p| p.role == "author")
                    .map(|p| p.name.clone())
                    .collect();
                vec![
                    detail.content.title.clone(),
                    authors.join(", "),
                    detail.genre.clone().unwrap_or_default(),
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
            .map(|detail| detail.content.id)
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
    use ritmo_presenter::{ContentDetail, PersonRoleView};

    fn detail(id: i64, title: &str, author: &str) -> ContentDetail {
        ContentDetail {
            content: Content {
                id,
                title: title.to_string(),
                publication_year: None,
                notes: None,
            },
            people_with_roles: vec![PersonRoleView {
                person_id: 1,
                name: author.to_string(),
                role: "author".to_string(),
            }],
            tags: vec![],
            linked_books: vec![],
            languages: vec![],
            genre: Some("Fantasy".to_string()),
        }
    }

    #[test]
    fn new_builds_table_headers_and_rows() {
        let screen = ContentListScreen::new(&[detail(1, "Il Nome della Rosa", "Umberto Eco")]);

        assert_eq!(
            screen.table.headers,
            vec!["Titolo", "Autori", "Genere"]
        );
        assert_eq!(
            screen.table.rows,
            vec![vec![
                "Il Nome della Rosa".to_string(),
                "Umberto Eco".to_string(),
                "Fantasy".to_string(),
            ]]
        );
    }

    #[test]
    fn handle_key_moves_selection_and_updates_statusbar_info() {
        let mut screen = ContentListScreen::new(&[detail(1, "A", "Autore A"), detail(2, "B", "Autore B")]);
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
        let mut screen = ContentListScreen::new(&[detail(10, "A", "A"), detail(20, "B", "B")]);

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

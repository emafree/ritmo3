use ratatui::{
    prelude::Frame,
    layout::Rect,
};
use ritmo_presenter::BookDetail;

use crate::widgets::table::TableWidget;

#[derive(Debug, Clone)]
pub struct BookListScreen {
    pub table: TableWidget,
    pub items: Vec<BookDetail>,
}

impl BookListScreen {
    pub fn new(items: &[BookDetail]) -> Self {
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
                    detail.book.title.clone(),
                    authors.join(", "),
                    detail.format.clone().unwrap_or_default(),
                    detail.series.clone().unwrap_or_default(),
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

    pub fn selected_id(&self) -> Option<i64> {
        self.items.get(self.table.selected_index()).map(|detail| detail.book.id)
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.table.render(frame, area);
    }
}

#[cfg(test)]
mod tests {
    use super::BookListScreen;
    use ritmo_domain::Book;
    use ritmo_presenter::{BookDetail, PersonRoleView};

    fn detail(id: i64, title: &str, author: &str) -> BookDetail {
        BookDetail {
            book: Book {
                id,
                title: title.to_string(),
                isbn: None,
                publication_year: None,
                notes: None,
            },
            people_with_roles: vec![PersonRoleView {
                person_id: 1,
                name: author.to_string(),
                role: "author".to_string(),
            }],
            tags: vec![],
            linked_contents: vec![],
            format: Some("Paperback".to_string()),
            series: Some("Saga".to_string()),
        }
    }

    #[test]
    fn new_builds_table_headers_and_rows() {
        let screen = BookListScreen::new(&[detail(1, "Dune", "Frank Herbert")]);

        assert_eq!(
            screen.table.headers,
            vec!["Title", "Authors", "Format", "Series"]
        );
        assert_eq!(
            screen.table.rows,
            vec![vec![
                "Dune".to_string(),
                "Frank Herbert".to_string(),
                "Paperback".to_string(),
                "Saga".to_string()
            ]]
        );
    }

    #[test]
    fn selected_id_returns_selected_book_id() {
        let mut screen = BookListScreen::new(&[detail(10, "A", "A"), detail(20, "B", "B")]);

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

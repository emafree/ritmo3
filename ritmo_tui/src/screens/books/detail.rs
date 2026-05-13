use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::Frame,
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};
use ritmo_domain::PartialDate;
use ritmo_presenter::BookDetail;

use crate::widgets::{input::InputWidget, statusbar::StatusBar};

const FIELD_LABELS: [&str; 7] = [
    "titolo",
    "isbn",
    "data di pubblicazione",
    "formato",
    "serie",
    "editore",
    "note",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetailMode {
    View,
    Edit,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BookDetailPendingChanges {
    pub title: Option<String>,
    pub isbn: Option<String>,
    pub publication_date: Option<String>,
    pub format: Option<String>,
    pub series: Option<String>,
    pub publisher: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BookDetailScreen {
    pub item: BookDetail,
    pub mode: DetailMode,
    pub selected_field: usize,
    pub input: InputWidget,
    editable_values: Vec<String>,
    initial_values: Vec<String>,
}

impl BookDetailScreen {
    pub fn new(item: BookDetail) -> Self {
        let editable_values = editable_values_from_item(&item);
        let initial_values = editable_values.clone();

        Self {
            item,
            mode: DetailMode::View,
            selected_field: 0,
            input: InputWidget::new(),
            editable_values,
            initial_values,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent, statusbar: &mut StatusBar) {
        match self.mode {
            DetailMode::View => match key.code {
                KeyCode::Up => self.previous_field(),
                KeyCode::Down => self.next_field(),
                KeyCode::Enter => self.start_editing(),
                _ => {}
            },
            DetailMode::Edit => match key.code {
                KeyCode::Enter => {
                    self.save_current_field();
                    self.mode = DetailMode::View;
                }
                KeyCode::Tab => {
                    self.save_current_field();
                    self.next_field();
                    self.start_editing();
                }
                KeyCode::Esc => {
                    self.input.clear();
                    self.mode = DetailMode::View;
                }
                KeyCode::Char(c) => self.input.handle_char(c),
                KeyCode::Backspace => self.input.handle_backspace(),
                KeyCode::Up => self.input.handle_up(),
                KeyCode::Down => self.input.handle_down(),
                _ => {}
            },
        }

        self.update_statusbar(statusbar);
    }

    pub fn pending_changes(&self) -> Option<BookDetailPendingChanges> {
        let changes = BookDetailPendingChanges {
            title: changed_value(&self.initial_values, &self.editable_values, 0),
            isbn: changed_value(&self.initial_values, &self.editable_values, 1),
            publication_date: changed_value(&self.initial_values, &self.editable_values, 2),
            format: changed_value(&self.initial_values, &self.editable_values, 3),
            series: changed_value(&self.initial_values, &self.editable_values, 4),
            publisher: changed_value(&self.initial_values, &self.editable_values, 5),
            notes: changed_value(&self.initial_values, &self.editable_values, 6),
        };

        if changes == BookDetailPendingChanges::default() {
            None
        } else {
            Some(changes)
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let editable_height = match self.mode {
            DetailMode::View => 9,
            DetailMode::Edit => 11,
        };
        let chunks =
            Layout::vertical([Constraint::Length(editable_height), Constraint::Min(0)]).split(area);

        self.render_editable_fields(frame, chunks[0]);
        self.render_read_only_sections(frame, chunks[1]);
    }

    fn previous_field(&mut self) {
        self.selected_field = self.selected_field.saturating_sub(1);
    }

    fn next_field(&mut self) {
        self.selected_field = (self.selected_field + 1).min(FIELD_LABELS.len() - 1);
    }

    fn start_editing(&mut self) {
        self.mode = DetailMode::Edit;
        self.input.value = self.editable_values[self.selected_field].clone();
        self.input.cursor = self.input.value.chars().count();
        self.input.suggestions.clear();
        self.input.selected_suggestion = None;
    }

    fn save_current_field(&mut self) {
        self.editable_values[self.selected_field] = self.input.accept();
    }

    fn update_statusbar(&self, statusbar: &mut StatusBar) {
        let keys = match self.mode {
            DetailMode::View => "↑↓: naviga | Enter: modifica | Esc: torna alla lista",
            DetailMode::Edit => "Enter: salva | Tab: prossimo campo | Esc: annulla",
        };
        statusbar.set_keys(vec![keys.to_string()]);
        statusbar.set_info(format!(
            "Campo {}/{}: {}",
            self.selected_field + 1,
            FIELD_LABELS.len(),
            FIELD_LABELS[self.selected_field]
        ));
    }

    fn render_editable_fields(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title("Dettagli libro")
            .borders(Borders::ALL);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let mut constraints = Vec::with_capacity(FIELD_LABELS.len());
        for (idx, _) in FIELD_LABELS.iter().enumerate() {
            let is_selected_edit = self.mode == DetailMode::Edit && idx == self.selected_field;
            constraints.push(if is_selected_edit {
                Constraint::Length(3)
            } else {
                Constraint::Length(1)
            });
        }
        let lines = Layout::vertical(constraints).split(inner);

        for (idx, field_area) in lines.iter().enumerate() {
            if self.mode == DetailMode::Edit && idx == self.selected_field {
                self.input.render(frame, *field_area);
                continue;
            }

            let value = display_value(self.editable_values[idx].as_str());
            let text = format!("{}: {value}", FIELD_LABELS[idx]);
            let style = if self.mode == DetailMode::View && idx == self.selected_field {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            frame.render_widget(Paragraph::new(text).style(style), *field_area);
        }
    }

    fn render_read_only_sections(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(area);

        let linked_contents = self
            .item
            .linked_contents
            .iter()
            .map(|item| item.title.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        let people_with_roles = self
            .item
            .people_with_roles
            .iter()
            .map(|person| format!("{} ({})", person.name, person.role))
            .collect::<Vec<_>>()
            .join("\n");

        let tags = self.item.tags.join("\n");
        let languages = self.read_only_languages().join("\n");

        self.render_read_only_block(
            frame,
            chunks[0],
            "contenuti collegati",
            linked_contents.as_str(),
        );
        self.render_read_only_block(
            frame,
            chunks[1],
            "persone con ruoli",
            people_with_roles.as_str(),
        );
        self.render_read_only_block(frame, chunks[2], "tag", tags.as_str());
        self.render_read_only_block(frame, chunks[3], "lingue", languages.as_str());
    }

    fn render_read_only_block(&self, frame: &mut Frame, area: Rect, title: &str, content: &str) {
        let display = if content.is_empty() { "—" } else { content };
        let paragraph =
            Paragraph::new(display).block(Block::default().title(title).borders(Borders::ALL));
        frame.render_widget(paragraph, area);
    }

    fn read_only_languages(&self) -> Vec<String> {
        // `BookDetail` currently does not expose languages directly.
        // Until presenter support is added, this section is intentionally empty/read-only.
        Vec::new()
    }
}

fn changed_value(
    initial_values: &[String],
    current_values: &[String],
    index: usize,
) -> Option<String> {
    if initial_values[index] == current_values[index] {
        None
    } else {
        Some(current_values[index].clone())
    }
}

fn editable_values_from_item(item: &BookDetail) -> Vec<String> {
    vec![
        item.book.title.clone(),
        item.book.isbn.clone().unwrap_or_default(),
        format_partial_date(item.book.publication_year.as_ref()),
        String::new(), // format placeholder: not available in `BookDetail` yet.
        String::new(), // series placeholder: not available in `BookDetail` yet.
        String::new(), // publisher placeholder: not available in `BookDetail` yet.
        item.book.notes.clone().unwrap_or_default(),
    ]
}

fn format_partial_date(date: Option<&PartialDate>) -> String {
    let Some(date) = date else {
        return String::new();
    };

    match (date.year, date.month, date.day) {
        (Some(year), Some(month), Some(day)) => format!("{year:04}-{month:02}-{day:02}"),
        (Some(year), Some(month), None) => format!("{year:04}-{month:02}"),
        (Some(year), None, None) => format!("{year:04}"),
        // Be permissive for partially populated legacy values and keep formatting stable.
        (None, Some(month), Some(day)) => format!("--{month:02}-{day:02}"),
        (None, Some(month), None) => format!("--{month:02}"),
        (None, None, Some(day)) => format!("---{day:02}"),
        (Some(year), None, Some(day)) => format!("{year:04}---{day:02}"),
        (None, None, None) => String::new(),
    }
}

fn display_value(value: &str) -> &str {
    if value.is_empty() {
        "—"
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::{BookDetailPendingChanges, BookDetailScreen, DetailMode};
    use crate::widgets::statusbar::StatusBar;
    use crossterm::event::{KeyCode, KeyEvent};
    use ratatui::{backend::TestBackend, Terminal};
    use ritmo_domain::{Book, PartialDate};
    use ritmo_presenter::{BookDetail, ContentListItem, PersonRoleView};

    fn detail() -> BookDetail {
        BookDetail {
            book: Book {
                id: 1,
                title: "Titolo".to_string(),
                isbn: Some("123".to_string()),
                publication_year: Some(PartialDate {
                    year: Some(2020),
                    month: Some(5),
                    day: None,
                    circa: false,
                }),
                notes: Some("Note".to_string()),
            },
            linked_contents: vec![ContentListItem {
                id: 10,
                title: "Contenuto A".to_string(),
                authors: vec!["Autore".to_string()],
                genre: None,
            }],
            people_with_roles: vec![PersonRoleView {
                person_id: 5,
                name: "Persona".to_string(),
                role: "Autore".to_string(),
            }],
            tags: vec!["tag1".to_string()],
        }
    }

    #[test]
    fn enter_switches_to_edit_and_loads_selected_value() {
        let mut screen = BookDetailScreen::new(detail());
        let mut statusbar = StatusBar::new();

        screen.handle_key(KeyEvent::from(KeyCode::Enter), &mut statusbar);

        assert_eq!(screen.mode, DetailMode::Edit);
        assert_eq!(screen.input.value, "Titolo");
        assert_eq!(screen.input.cursor, 6);
        assert_eq!(
            statusbar.keys,
            vec!["Enter: salva | Tab: prossimo campo | Esc: annulla"]
        );
    }

    #[test]
    fn enter_in_edit_saves_field_and_returns_to_view() {
        let mut screen = BookDetailScreen::new(detail());
        let mut statusbar = StatusBar::new();

        screen.handle_key(KeyEvent::from(KeyCode::Enter), &mut statusbar);
        screen.input.value = "Nuovo titolo".to_string();
        screen.input.cursor = screen.input.value.chars().count();
        screen.handle_key(KeyEvent::from(KeyCode::Enter), &mut statusbar);

        assert_eq!(screen.mode, DetailMode::View);
        assert_eq!(
            screen.pending_changes(),
            Some(BookDetailPendingChanges {
                title: Some("Nuovo titolo".to_string()),
                ..BookDetailPendingChanges::default()
            })
        );
    }

    #[test]
    fn tab_saves_and_moves_to_next_field_in_edit_mode() {
        let mut screen = BookDetailScreen::new(detail());
        let mut statusbar = StatusBar::new();

        screen.handle_key(KeyEvent::from(KeyCode::Enter), &mut statusbar);
        screen.input.value = "Nuovo titolo".to_string();
        screen.input.cursor = screen.input.value.chars().count();
        screen.handle_key(KeyEvent::from(KeyCode::Tab), &mut statusbar);

        assert_eq!(screen.mode, DetailMode::Edit);
        assert_eq!(screen.selected_field, 1);
        assert_eq!(screen.input.value, "123");
        assert_eq!(
            screen.pending_changes(),
            Some(BookDetailPendingChanges {
                title: Some("Nuovo titolo".to_string()),
                ..BookDetailPendingChanges::default()
            })
        );
    }

    #[test]
    fn esc_in_edit_cancels_changes_for_current_field() {
        let mut screen = BookDetailScreen::new(detail());
        let mut statusbar = StatusBar::new();

        screen.handle_key(KeyEvent::from(KeyCode::Enter), &mut statusbar);
        screen.input.value = "Annullato".to_string();
        screen.input.cursor = screen.input.value.chars().count();
        screen.handle_key(KeyEvent::from(KeyCode::Esc), &mut statusbar);

        assert_eq!(screen.mode, DetailMode::View);
        assert_eq!(screen.pending_changes(), None);
    }

    #[test]
    fn render_shows_editable_and_read_only_sections() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut screen = BookDetailScreen::new(detail());

        terminal
            .draw(|frame| screen.render(frame, frame.area()))
            .unwrap();

        let buffer = terminal.backend().buffer().clone();
        let rendered = (0..buffer.area.height)
            .map(|y| {
                (0..buffer.area.width)
                    .map(|x| buffer[(x, y)].symbol())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(rendered.contains("Dettagli libro"));
        assert!(rendered.contains("titolo"));
        assert!(rendered.contains("contenuti collegati"));
        assert!(rendered.contains("persone con ruoli"));
        assert!(rendered.contains("tag"));
        assert!(rendered.contains("lingue"));
    }
}

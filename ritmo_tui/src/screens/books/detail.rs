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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RelatedSection {
    LinkedContents,
    PeopleWithRoles,
    Tags,
    Languages,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookDetailRelatedAction {
    AddLinkedContent,
    RemoveLinkedContent { content_id: i64 },
    OpenLinkedContent { content_id: i64 },
    AddPersonWithRole,
    RemovePersonWithRole { person_id: i64 },
    OpenPersonWithRole { person_id: i64 },
    AddTag,
    RemoveTag { tag: String },
    AddLanguage,
    RemoveLanguage { language: String },
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
    in_related_sections: bool,
    related_section: RelatedSection,
    selected_linked_content: usize,
    selected_person_with_role: usize,
    selected_tag: usize,
    selected_language: usize,
    pending_related_action: Option<BookDetailRelatedAction>,
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
            in_related_sections: false,
            related_section: RelatedSection::LinkedContents,
            selected_linked_content: 0,
            selected_person_with_role: 0,
            selected_tag: 0,
            selected_language: 0,
            pending_related_action: None,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent, statusbar: &mut StatusBar) {
        match self.mode {
            DetailMode::View => match key.code {
                KeyCode::Up => self.previous_section(),
                KeyCode::Down => self.next_section(),
                KeyCode::Char('k') => self.previous_related_item(),
                KeyCode::Char('j') => self.next_related_item(),
                KeyCode::Enter => self.handle_enter_on_section(),
                KeyCode::Char('n') => self.handle_add_related_item(),
                KeyCode::Char('d') | KeyCode::Delete => self.handle_remove_related_item(),
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

    pub fn take_related_action(&mut self) -> Option<BookDetailRelatedAction> {
        self.pending_related_action.take()
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

    fn previous_section(&mut self) {
        if self.in_related_sections {
            match self.related_section {
                RelatedSection::LinkedContents => self.in_related_sections = false,
                RelatedSection::PeopleWithRoles => {
                    self.related_section = RelatedSection::LinkedContents
                }
                RelatedSection::Tags => self.related_section = RelatedSection::PeopleWithRoles,
                RelatedSection::Languages => self.related_section = RelatedSection::Tags,
            }
            return;
        }

        self.previous_field();
    }

    fn next_section(&mut self) {
        if !self.in_related_sections {
            if self.selected_field + 1 < FIELD_LABELS.len() {
                self.next_field();
                return;
            }

            self.in_related_sections = true;
            self.related_section = RelatedSection::LinkedContents;
            return;
        }

        self.related_section = match self.related_section {
            RelatedSection::LinkedContents => RelatedSection::PeopleWithRoles,
            RelatedSection::PeopleWithRoles => RelatedSection::Tags,
            RelatedSection::Tags => RelatedSection::Languages,
            RelatedSection::Languages => RelatedSection::Languages,
        };
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

    fn handle_enter_on_section(&mut self) {
        if !self.in_related_sections {
            self.start_editing();
            return;
        }

        self.pending_related_action = match self.related_section {
            RelatedSection::LinkedContents => self
                .selected_linked_content_id()
                .map(|content_id| BookDetailRelatedAction::OpenLinkedContent { content_id }),
            RelatedSection::PeopleWithRoles => self
                .selected_person_with_role_id()
                .map(|person_id| BookDetailRelatedAction::OpenPersonWithRole { person_id }),
            RelatedSection::Tags | RelatedSection::Languages => None,
        };
    }

    fn handle_add_related_item(&mut self) {
        if !self.in_related_sections {
            return;
        }

        self.pending_related_action = Some(match self.related_section {
            RelatedSection::LinkedContents => BookDetailRelatedAction::AddLinkedContent,
            RelatedSection::PeopleWithRoles => BookDetailRelatedAction::AddPersonWithRole,
            RelatedSection::Tags => BookDetailRelatedAction::AddTag,
            RelatedSection::Languages => BookDetailRelatedAction::AddLanguage,
        });
    }

    fn handle_remove_related_item(&mut self) {
        if !self.in_related_sections {
            return;
        }

        self.pending_related_action = match self.related_section {
            RelatedSection::LinkedContents => self
                .selected_linked_content_id()
                .map(|content_id| BookDetailRelatedAction::RemoveLinkedContent { content_id }),
            RelatedSection::PeopleWithRoles => self
                .selected_person_with_role_id()
                .map(|person_id| BookDetailRelatedAction::RemovePersonWithRole { person_id }),
            RelatedSection::Tags => self
                .selected_tag()
                .map(|tag| BookDetailRelatedAction::RemoveTag { tag }),
            RelatedSection::Languages => self
                .selected_language()
                .map(|language| BookDetailRelatedAction::RemoveLanguage { language }),
        };
    }

    fn previous_related_item(&mut self) {
        if !self.in_related_sections {
            return;
        }

        match self.related_section {
            RelatedSection::LinkedContents => {
                self.selected_linked_content = self.selected_linked_content.saturating_sub(1);
            }
            RelatedSection::PeopleWithRoles => {
                self.selected_person_with_role = self.selected_person_with_role.saturating_sub(1);
            }
            RelatedSection::Tags => {
                self.selected_tag = self.selected_tag.saturating_sub(1);
            }
            RelatedSection::Languages => {
                self.selected_language = self.selected_language.saturating_sub(1);
            }
        }
    }

    fn next_related_item(&mut self) {
        if !self.in_related_sections {
            return;
        }

        match self.related_section {
            RelatedSection::LinkedContents => {
                let max = self.item.linked_contents.len().saturating_sub(1);
                self.selected_linked_content = (self.selected_linked_content + 1).min(max);
            }
            RelatedSection::PeopleWithRoles => {
                let max = self.item.people_with_roles.len().saturating_sub(1);
                self.selected_person_with_role = (self.selected_person_with_role + 1).min(max);
            }
            RelatedSection::Tags => {
                let max = self.item.tags.len().saturating_sub(1);
                self.selected_tag = (self.selected_tag + 1).min(max);
            }
            RelatedSection::Languages => {
                let max = self.read_only_languages().len().saturating_sub(1);
                self.selected_language = (self.selected_language + 1).min(max);
            }
        }
    }

    fn update_statusbar(&self, statusbar: &mut StatusBar) {
        let keys = match self.mode {
            DetailMode::View if !self.in_related_sections => {
                "↑↓: naviga sezioni | Enter: modifica campo | Esc: torna alla lista"
            }
            DetailMode::View => match self.related_section {
                RelatedSection::LinkedContents => {
                    "↑↓: naviga sezioni | j/k: seleziona | n: aggiungi | d/Del: rimuovi | Enter: apri"
                }
                RelatedSection::PeopleWithRoles => {
                    "↑↓: naviga sezioni | j/k: seleziona | n: aggiungi | d/Del: rimuovi | Enter: apri"
                }
                RelatedSection::Tags => {
                    "↑↓: naviga sezioni | j/k: seleziona | n: aggiungi | d/Del: rimuovi"
                }
                RelatedSection::Languages => {
                    "↑↓: naviga sezioni | j/k: seleziona | n: aggiungi | d/Del: rimuovi"
                }
            },
            DetailMode::Edit => "Enter: salva | Tab: prossimo campo | Esc: annulla",
        };
        statusbar.set_keys(vec![keys.to_string()]);

        if !self.in_related_sections {
            statusbar.set_info(format!(
                "Campo {}/{}: {}",
                self.selected_field + 1,
                FIELD_LABELS.len(),
                FIELD_LABELS[self.selected_field]
            ));
            return;
        }

        let info = match self.related_section {
            RelatedSection::LinkedContents => format!(
                "Sezione: contenuti collegati ({}/{})",
                current_position(
                    self.selected_linked_content,
                    self.item.linked_contents.len()
                ),
                self.item.linked_contents.len()
            ),
            RelatedSection::PeopleWithRoles => format!(
                "Sezione: persone con ruoli ({}/{})",
                current_position(
                    self.selected_person_with_role,
                    self.item.people_with_roles.len(),
                ),
                self.item.people_with_roles.len()
            ),
            RelatedSection::Tags => format!(
                "Sezione: tag ({}/{})",
                current_position(self.selected_tag, self.item.tags.len()),
                self.item.tags.len()
            ),
            RelatedSection::Languages => {
                let languages = self.read_only_languages();
                format!(
                    "Sezione: lingue ({}/{})",
                    current_position(self.selected_language, languages.len()),
                    languages.len()
                )
            }
        };
        statusbar.set_info(info);
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
            let style = if self.mode == DetailMode::View
                && !self.in_related_sections
                && idx == self.selected_field
            {
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
            .map(|item| item.title.clone())
            .collect::<Vec<_>>();

        let people_with_roles = self
            .item
            .people_with_roles
            .iter()
            .map(|person| format!("{} ({})", person.name, person.role))
            .collect::<Vec<_>>();

        let tags = self.item.tags.clone();
        let languages = self.read_only_languages();

        self.render_related_block(
            frame,
            chunks[0],
            "contenuti collegati",
            &linked_contents,
            self.related_section == RelatedSection::LinkedContents
                && self.is_related_section_active(),
            self.selected_linked_content,
        );
        self.render_related_block(
            frame,
            chunks[1],
            "persone con ruoli",
            &people_with_roles,
            self.related_section == RelatedSection::PeopleWithRoles
                && self.is_related_section_active(),
            self.selected_person_with_role,
        );
        self.render_related_block(
            frame,
            chunks[2],
            "tag",
            &tags,
            self.related_section == RelatedSection::Tags && self.is_related_section_active(),
            self.selected_tag,
        );
        self.render_related_block(
            frame,
            chunks[3],
            "lingue",
            &languages,
            self.related_section == RelatedSection::Languages && self.is_related_section_active(),
            self.selected_language,
        );
    }

    fn render_related_block(
        &self,
        frame: &mut Frame,
        area: Rect,
        title: &str,
        items: &[String],
        is_active: bool,
        selected_index: usize,
    ) {
        let block = if is_active {
            Block::default()
                .title(format!("{title} *"))
                .borders(Borders::ALL)
                .border_style(Style::default().add_modifier(Modifier::REVERSED))
        } else {
            Block::default().title(title).borders(Borders::ALL)
        };

        let display = if items.is_empty() {
            "—".to_string()
        } else {
            items
                .iter()
                .enumerate()
                .map(|(idx, item)| {
                    if is_active && idx == clamped_index(selected_index, items.len()) {
                        format!("> {item}")
                    } else {
                        format!("  {item}")
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
        };

        let paragraph = Paragraph::new(display).block(block);
        frame.render_widget(paragraph, area);
    }

    fn is_related_section_active(&self) -> bool {
        self.mode == DetailMode::View && self.in_related_sections
    }

    fn read_only_languages(&self) -> Vec<String> {
        // `BookDetail` currently does not expose languages directly.
        // Until presenter support is added, this section is intentionally empty/read-only.
        Vec::new()
    }

    fn selected_linked_content_id(&self) -> Option<i64> {
        self.item
            .linked_contents
            .get(clamped_index(
                self.selected_linked_content,
                self.item.linked_contents.len(),
            ))
            .map(|item| item.id)
    }

    fn selected_person_with_role_id(&self) -> Option<i64> {
        self.item
            .people_with_roles
            .get(clamped_index(
                self.selected_person_with_role,
                self.item.people_with_roles.len(),
            ))
            .map(|person| person.person_id)
    }

    fn selected_tag(&self) -> Option<String> {
        self.item
            .tags
            .get(clamped_index(self.selected_tag, self.item.tags.len()))
            .cloned()
    }

    fn selected_language(&self) -> Option<String> {
        let languages = self.read_only_languages();
        languages
            .get(clamped_index(self.selected_language, languages.len()))
            .cloned()
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

fn clamped_index(selected: usize, total: usize) -> usize {
    selected.min(total.saturating_sub(1))
}

fn current_position(selected: usize, total: usize) -> usize {
    if total == 0 {
        0
    } else {
        clamped_index(selected, total) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::{BookDetailPendingChanges, BookDetailRelatedAction, BookDetailScreen, DetailMode};
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
            format: None,
            series: None,
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
    fn related_sections_update_statusbar_and_emit_actions() {
        let mut screen = BookDetailScreen::new(detail());
        let mut statusbar = StatusBar::new();

        for _ in 0..7 {
            screen.handle_key(KeyEvent::from(KeyCode::Down), &mut statusbar);
        }
        assert_eq!(
            statusbar.keys,
            vec![
                "↑↓: naviga sezioni | j/k: seleziona | n: aggiungi | d/Del: rimuovi | Enter: apri"
            ]
        );
        assert_eq!(statusbar.info, "Sezione: contenuti collegati (1/1)");

        screen.handle_key(KeyEvent::from(KeyCode::Char('n')), &mut statusbar);
        assert_eq!(
            screen.take_related_action(),
            Some(BookDetailRelatedAction::AddLinkedContent)
        );

        screen.handle_key(KeyEvent::from(KeyCode::Enter), &mut statusbar);
        assert_eq!(
            screen.take_related_action(),
            Some(BookDetailRelatedAction::OpenLinkedContent { content_id: 10 })
        );

        screen.handle_key(KeyEvent::from(KeyCode::Down), &mut statusbar);
        screen.handle_key(KeyEvent::from(KeyCode::Delete), &mut statusbar);
        assert_eq!(
            screen.take_related_action(),
            Some(BookDetailRelatedAction::RemovePersonWithRole { person_id: 5 })
        );

        screen.handle_key(KeyEvent::from(KeyCode::Down), &mut statusbar);
        assert_eq!(
            statusbar.keys,
            vec!["↑↓: naviga sezioni | j/k: seleziona | n: aggiungi | d/Del: rimuovi"]
        );
        screen.handle_key(KeyEvent::from(KeyCode::Char('d')), &mut statusbar);
        assert_eq!(
            screen.take_related_action(),
            Some(BookDetailRelatedAction::RemoveTag {
                tag: "tag1".to_string()
            })
        );
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

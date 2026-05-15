use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::Frame,
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};
use ritmo_domain::{Content, PartialDate};

use crate::widgets::{
    input::InputWidget,
    language::LanguageWidget,
    partial_date::{PartialDateField, PartialDateWidget},
    person::PersonWidget,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentField {
    Title,
    PublicationDate,
    Genre,
    Notes,
    People,
    Languages,
    Tags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentCreateAction {
    None,
    Submit,
    Cancel,
}

#[derive(Debug, Clone)]
pub struct ContentCreateScreen {
    pub active_field: ContentField,
    pub title: InputWidget,
    pub publication_date: PartialDateWidget,
    pub genre: InputWidget,
    pub notes: InputWidget,
    pub people: Vec<(PersonWidget, InputWidget)>,
    pub languages: Vec<(LanguageWidget, InputWidget)>,
    pub tags: Vec<InputWidget>,
}

impl ContentCreateScreen {
    pub fn new() -> Self {
        Self {
            active_field: ContentField::Title,
            title: InputWidget::new(),
            publication_date: PartialDateWidget::new(),
            genre: InputWidget::new(),
            notes: InputWidget::new(),
            people: Vec::new(),
            languages: Vec::new(),
            tags: Vec::new(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> ContentCreateAction {
        if key.code == KeyCode::Esc {
            return ContentCreateAction::Cancel;
        }
        if key.code == KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL) {
            return ContentCreateAction::Submit;
        }

        // Tab/BackTab always take priority for field navigation.
        // Inside PublicationDate, Tab cycles sub-fields until Circa (the last),
        // at which point it exits to the next field. BackTab cycles backwards until
        // Year (the first), at which point it exits to the previous field.
        match key.code {
            KeyCode::Tab => {
                if self.active_field == ContentField::PublicationDate
                    && self.publication_date.active_field != PartialDateField::Circa
                {
                    self.publication_date.handle_key(key);
                } else {
                    self.next_field();
                }
                return ContentCreateAction::None;
            }
            KeyCode::BackTab => {
                if self.active_field == ContentField::PublicationDate
                    && self.publication_date.active_field != PartialDateField::Year
                {
                    self.publication_date.handle_key(key);
                } else {
                    self.previous_field();
                }
                return ContentCreateAction::None;
            }
            _ => {}
        }

        if key.code == KeyCode::Enter {
            self.next_field();
            return ContentCreateAction::None;
        }

        if self.active_field == ContentField::PublicationDate {
            self.publication_date.handle_key(key);
            return ContentCreateAction::None;
        }

        match self.active_field {
            ContentField::Title => self.title.handle_key(key),
            ContentField::Genre => self.genre.handle_key(key),
            ContentField::Notes => self.notes.handle_key(key),
            ContentField::People => match key.code {
                KeyCode::Char('n') => self.people.push((PersonWidget::new(), InputWidget::new())),
                KeyCode::Char('d') | KeyCode::Delete => {
                    self.people.pop();
                }
                _ => {
                    if let Some((person, _)) = self.people.last_mut() {
                        person.handle_key(key);
                    }
                }
            },
            ContentField::Languages => match key.code {
                KeyCode::Char('n') => {
                    self.languages
                        .push((LanguageWidget::new(), InputWidget::new()))
                }
                KeyCode::Char('d') | KeyCode::Delete => {
                    self.languages.pop();
                }
                _ => {
                    if let Some((language, _)) = self.languages.last_mut() {
                        language.handle_key(key);
                    }
                }
            },
            ContentField::Tags => match key.code {
                KeyCode::Char('n') => self.tags.push(InputWidget::new()),
                KeyCode::Char('d') | KeyCode::Delete => {
                    self.tags.pop();
                }
                _ => {
                    if let Some(last) = self.tags.last_mut() {
                        last.handle_key(key);
                    }
                }
            },
            ContentField::PublicationDate => {}
        }

        ContentCreateAction::None
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let layout = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(area);
        let form_area = layout[0];
        let hint_area = layout[1];

        let hint = match self.active_field {
            ContentField::PublicationDate => {
                "Tab/BackTab: sotto-campi data | Enter: prossimo campo | Ctrl+S: salva | Esc: annulla"
            }
            ContentField::People | ContentField::Languages | ContentField::Tags => {
                "Tab: avanti | BackTab: indietro | n: aggiungi | d: rimuovi | Ctrl+S: salva | Esc: annulla"
            }
            _ => {
                "Tab: avanti | BackTab: indietro | Enter: prossimo campo | Ctrl+S: salva | Esc: annulla"
            }
        };
        frame.render_widget(Paragraph::new(hint), hint_area);

        let outer_block = Block::default().title("Crea Contenuto").borders(Borders::ALL);
        let inner = outer_block.inner(form_area);
        frame.render_widget(outer_block, form_area);

        let af = self.active_field;
        let constraints = [
            text_field_height(af, ContentField::Title),
            Constraint::Length(3),
            text_field_height(af, ContentField::Genre),
            text_field_height(af, ContentField::Notes),
            collection_height(af, ContentField::People),
            collection_height(af, ContentField::Languages),
            collection_height(af, ContentField::Tags),
        ];
        let rows = Layout::vertical(constraints).split(inner);

        let title_val = self.title.value.clone();
        let genre_val = self.genre.value.clone();
        let notes_val = self.notes.value.clone();
        let people_display = collection_text_summary(self.people.iter().map(|(p, _)| p.input.value.as_str()));
        let languages_display =
            collection_text_summary(self.languages.iter().map(|(l, _)| l.input.value.as_str()));
        let tags_display = collection_text_summary(self.tags.iter().map(|t| t.value.as_str()));

        if af == ContentField::Title {
            self.title.render(frame, rows[0]);
        } else {
            render_label_row(frame, rows[0], "Titolo", &title_val);
        }

        {
            let is_active = af == ContentField::PublicationDate;
            let block = labeled_block("Data di pubblicazione", is_active);
            let date_inner = block.inner(rows[1]);
            frame.render_widget(block, rows[1]);
            self.publication_date.render(frame, date_inner, is_active);
        }

        if af == ContentField::Genre {
            self.genre.render(frame, rows[2]);
        } else {
            render_label_row(frame, rows[2], "Genere", &genre_val);
        }

        if af == ContentField::Notes {
            self.notes.render(frame, rows[3]);
        } else {
            render_label_row(frame, rows[3], "Note", &notes_val);
        }

        {
            let is_active = af == ContentField::People;
            let block = labeled_block("Persone", is_active);
            let coll_inner = block.inner(rows[4]);
            frame.render_widget(block, rows[4]);
            if is_active {
                if let Some((person, role)) = self.people.last_mut() {
                    let split = Layout::vertical([Constraint::Length(3), Constraint::Length(3)])
                        .split(coll_inner);
                    person.render(frame, split[0]);
                    role.render(frame, split[1]);
                } else {
                    frame.render_widget(Paragraph::new("—"), coll_inner);
                }
            } else {
                frame.render_widget(Paragraph::new(people_display), coll_inner);
            }
        }

        {
            let is_active = af == ContentField::Languages;
            let block = labeled_block("Lingue", is_active);
            let coll_inner = block.inner(rows[5]);
            frame.render_widget(block, rows[5]);
            if is_active {
                if let Some((language, role)) = self.languages.last_mut() {
                    let split = Layout::vertical([Constraint::Length(3), Constraint::Length(3)])
                        .split(coll_inner);
                    language.render(frame, split[0]);
                    role.render(frame, split[1]);
                } else {
                    frame.render_widget(Paragraph::new("—"), coll_inner);
                }
            } else {
                frame.render_widget(Paragraph::new(languages_display), coll_inner);
            }
        }

        {
            let is_active = af == ContentField::Tags;
            let block = labeled_block("Tag", is_active);
            let coll_inner = block.inner(rows[6]);
            frame.render_widget(block, rows[6]);
            if is_active && !self.tags.is_empty() {
                self.tags.last_mut().unwrap().render(frame, coll_inner);
            } else {
                frame.render_widget(Paragraph::new(tags_display), coll_inner);
            }
        }
    }

    pub fn to_content(&self) -> Option<Content> {
        let title = self.title.value.trim().to_string();
        if title.is_empty() {
            return None;
        }

        Some(Content {
            id: 0,
            title,
            publication_year: date_to_opt(self.publication_date.value()),
            notes: to_opt(&self.notes.value),
        })
    }

    fn next_field(&mut self) {
        self.active_field = match self.active_field {
            ContentField::Title => ContentField::PublicationDate,
            ContentField::PublicationDate => ContentField::Genre,
            ContentField::Genre => ContentField::Notes,
            ContentField::Notes => ContentField::People,
            ContentField::People => ContentField::Languages,
            ContentField::Languages => ContentField::Tags,
            ContentField::Tags => ContentField::Title,
        };
    }

    fn previous_field(&mut self) {
        self.active_field = match self.active_field {
            ContentField::Title => ContentField::Tags,
            ContentField::PublicationDate => ContentField::Title,
            ContentField::Genre => ContentField::PublicationDate,
            ContentField::Notes => ContentField::Genre,
            ContentField::People => ContentField::Notes,
            ContentField::Languages => ContentField::People,
            ContentField::Tags => ContentField::Languages,
        };
    }
}

impl Default for ContentCreateScreen {
    fn default() -> Self {
        Self::new()
    }
}

fn text_field_height(active: ContentField, field: ContentField) -> Constraint {
    if active == field {
        Constraint::Length(3)
    } else {
        Constraint::Length(1)
    }
}

fn collection_height(active: ContentField, field: ContentField) -> Constraint {
    if active == field {
        Constraint::Length(8)
    } else {
        Constraint::Length(1)
    }
}

fn labeled_block(title: &str, is_active: bool) -> Block<'_> {
    let mut block = Block::default().title(title).borders(Borders::ALL);
    if is_active {
        block = block.border_style(Style::default().add_modifier(Modifier::REVERSED));
    }
    block
}

fn render_label_row(frame: &mut Frame, area: Rect, label: &str, value: &str) {
    let display = if value.trim().is_empty() { "—" } else { value };
    frame.render_widget(Paragraph::new(format!("{label}: {display}")), area);
}

fn collection_text_summary<'a>(values: impl Iterator<Item = &'a str>) -> String {
    let non_empty: Vec<&str> = values.filter(|v| !v.trim().is_empty()).collect();
    if non_empty.is_empty() {
        "—".to_string()
    } else {
        non_empty.join(", ")
    }
}

fn to_opt(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn date_to_opt(date: PartialDate) -> Option<PartialDate> {
    if date.year.is_none() && date.month.is_none() && date.day.is_none() && !date.circa {
        None
    } else {
        Some(date)
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ratatui::{backend::TestBackend, Terminal};

    use super::{ContentCreateAction, ContentCreateScreen, ContentField};

    #[test]
    fn new_starts_at_title_with_empty_state() {
        let screen = ContentCreateScreen::new();

        assert_eq!(screen.active_field, ContentField::Title);
        assert!(screen.title.value.is_empty());
        assert!(screen.people.is_empty());
        assert!(screen.languages.is_empty());
        assert!(screen.tags.is_empty());
    }

    #[test]
    fn tab_and_backtab_navigate_fields_with_wrap() {
        let mut screen = ContentCreateScreen::new();

        // Tab from Title → PublicationDate (lands on Year sub-field)
        screen.handle_key(KeyEvent::from(KeyCode::Tab));
        assert_eq!(screen.active_field, ContentField::PublicationDate);

        // BackTab from PublicationDate while Year is active → exits to Title
        screen.handle_key(KeyEvent::from(KeyCode::BackTab));
        assert_eq!(screen.active_field, ContentField::Title);

        // Enter from Title → PublicationDate
        screen.handle_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(screen.active_field, ContentField::PublicationDate);

        // BackTab from PublicationDate (Year sub-field) → back to Title
        screen.handle_key(KeyEvent::from(KeyCode::BackTab));
        assert_eq!(screen.active_field, ContentField::Title);

        // Wrapping: BackTab from Title → Tags
        let mut wrap_screen = ContentCreateScreen::new();
        wrap_screen.handle_key(KeyEvent::from(KeyCode::BackTab));
        assert_eq!(wrap_screen.active_field, ContentField::Tags);
    }

    #[test]
    fn esc_and_ctrl_s_return_actions() {
        let mut screen = ContentCreateScreen::new();

        let cancel = screen.handle_key(KeyEvent::from(KeyCode::Esc));
        assert_eq!(cancel, ContentCreateAction::Cancel);

        let submit =
            screen.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL));
        assert_eq!(submit, ContentCreateAction::Submit);
    }

    #[test]
    fn collections_support_add_and_remove() {
        let mut screen = ContentCreateScreen::new();

        for _ in 0..4 {
            screen.handle_key(KeyEvent::from(KeyCode::Enter));
        }
        assert_eq!(screen.active_field, ContentField::People);
        screen.handle_key(KeyEvent::from(KeyCode::Char('n')));
        assert_eq!(screen.people.len(), 1);
        screen.handle_key(KeyEvent::from(KeyCode::Char('d')));
        assert!(screen.people.is_empty());

        screen.handle_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(screen.active_field, ContentField::Languages);
        screen.handle_key(KeyEvent::from(KeyCode::Char('n')));
        assert_eq!(screen.languages.len(), 1);

        screen.handle_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(screen.active_field, ContentField::Tags);
        screen.handle_key(KeyEvent::from(KeyCode::Char('n')));
        assert_eq!(screen.tags.len(), 1);
    }

    #[test]
    fn to_content_validates_title_and_maps_optional_fields() {
        let mut screen = ContentCreateScreen::new();
        assert!(screen.to_content().is_none());

        screen.title.value = "Il nome della rosa".into();
        screen.notes.value = "  Romanzo storico  ".into();

        screen.active_field = ContentField::PublicationDate;
        screen.handle_key(KeyEvent::from(KeyCode::Char('1')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('9')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('8')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('0')));

        let content = screen.to_content().unwrap();
        assert_eq!(content.id, 0);
        assert_eq!(content.title, "Il nome della rosa");
        assert_eq!(content.notes.as_deref(), Some("Romanzo storico"));
        assert_eq!(content.publication_year.unwrap().year, Some(1980));
    }

    #[test]
    fn render_produces_output_with_all_field_labels() {
        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut screen = ContentCreateScreen::new();
        screen.title.value = "Dune".to_string();

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

        assert!(rendered.contains("Crea Contenuto"));
        assert!(rendered.contains("Data di pubblicazione"));
        assert!(rendered.contains("Genere"));
        assert!(rendered.contains("Note"));
        assert!(rendered.contains("Persone"));
        assert!(rendered.contains("Lingue"));
        assert!(rendered.contains("Tag"));
    }
}

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::Frame,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
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

#[derive(Debug, Clone)]
pub struct ContentDraft {
    pub name: String,
    pub original_title: Option<String>,
    pub type_id: Option<i64>,
    pub genre_id: Option<i64>,
    pub publication_date: Option<PartialDate>,
    pub notes: Option<String>,
}

impl PartialEq for ContentDraft {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.original_title == other.original_title
            && self.type_id == other.type_id
            && self.genre_id == other.genre_id
            && self.notes == other.notes
            && partial_date_eq(&self.publication_date, &other.publication_date)
    }
}

impl Eq for ContentDraft {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentCreateAction {
    None,
    Submit(ContentDraft),
    CreatePersonForContent(String),
    Cancel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentPersonRole {
    pub person_id: i64,
    pub person_name: String,
    pub role_id: i64,
    pub role_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PeopleAddStep {
    SelectPerson,
    SelectRole,
}

#[derive(Debug, Clone)]
struct PeopleAddFlow {
    step: PeopleAddStep,
    person_widget: PersonWidget,
    selected_person: Option<(i64, String)>,
    role_index: usize,
}

impl PeopleAddFlow {
    fn new() -> Self {
        Self {
            step: PeopleAddStep::SelectPerson,
            person_widget: PersonWidget::new(),
            selected_person: None,
            role_index: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContentCreateScreen {
    pub active_field: ContentField,
    pub title: InputWidget,
    pub publication_date: PartialDateWidget,
    pub genre: InputWidget,
    pub notes: InputWidget,
    pub people: Vec<ContentPersonRole>,
    pub people_options: Vec<(i64, String)>,
    pub role_options: Vec<(i64, String)>,
    people_add_flow: Option<PeopleAddFlow>,
    pub languages: Vec<(LanguageWidget, InputWidget)>,
    pub tags: Vec<InputWidget>,
}

impl ContentCreateScreen {
    pub fn new() -> Self {
        Self::new_with_options(vec![], vec![])
    }

    pub fn new_with_options(
        people_options: Vec<(i64, String)>,
        role_options: Vec<(i64, String)>,
    ) -> Self {
        Self {
            active_field: ContentField::Title,
            title: InputWidget::new(),
            publication_date: PartialDateWidget::new(),
            genre: InputWidget::new(),
            notes: InputWidget::new(),
            people: Vec::new(),
            people_options,
            role_options,
            people_add_flow: None,
            languages: Vec::new(),
            tags: Vec::new(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> ContentCreateAction {
        if key.code == KeyCode::Esc {
            if self.handle_escape() {
                return ContentCreateAction::None;
            }
            return ContentCreateAction::Cancel;
        }
        if key.code == KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL) {
            return ContentCreateAction::Submit(self.to_draft());
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
            if self.active_field == ContentField::People && self.people_add_flow.is_some() {
                return self.confirm_people_flow();
            }
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
                KeyCode::Char('n') => {
                    if self.people_add_flow.is_none() {
                        let mut flow = PeopleAddFlow::new();
                        flow.person_widget.set_options(self.people_options.clone());
                        self.people_add_flow = Some(flow);
                    }
                }
                KeyCode::Char('d') | KeyCode::Delete => {
                    if self.people_add_flow.is_none() {
                        self.people.pop();
                    }
                }
                KeyCode::Up | KeyCode::Down => {
                    self.handle_people_flow_directional_key(key);
                }
                _ => {
                    if let Some(flow) = self.people_add_flow.as_mut() {
                        if flow.step == PeopleAddStep::SelectPerson {
                            flow.person_widget.handle_key(key);
                        }
                    }
                }
            },
            ContentField::Languages => match key.code {
                KeyCode::Char('n') => self
                    .languages
                    .push((LanguageWidget::new(), InputWidget::new())),
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

    pub fn complete_person_creation(&mut self, person_id: i64, person_name: String) {
        if let Some(flow) = self.people_add_flow.as_mut() {
            if flow.step == PeopleAddStep::SelectPerson {
                self.people_options.push((person_id, person_name.clone()));
                self.people_options.sort_by(|a, b| a.1.cmp(&b.1));
                flow.selected_person = Some((person_id, person_name));
                flow.step = PeopleAddStep::SelectRole;
                flow.role_index = 0;
            }
        }
    }

    fn confirm_people_flow(&mut self) -> ContentCreateAction {
        let Some(flow) = self.people_add_flow.as_mut() else {
            return ContentCreateAction::None;
        };

        match flow.step {
            PeopleAddStep::SelectPerson => {
                let Some(person) = flow.person_widget.accept_or_draft_person() else {
                    return ContentCreateAction::None;
                };
                if person.id > 0 {
                    flow.selected_person = Some((person.id, person.name));
                    flow.step = PeopleAddStep::SelectRole;
                    flow.role_index = 0;
                    return ContentCreateAction::None;
                }
                ContentCreateAction::CreatePersonForContent(person.name)
            }
            PeopleAddStep::SelectRole => {
                let Some((role_id, role_name)) = self.role_options.get(flow.role_index).cloned()
                else {
                    self.people_add_flow = None;
                    return ContentCreateAction::None;
                };
                let Some((person_id, person_name)) = flow.selected_person.clone() else {
                    self.people_add_flow = None;
                    return ContentCreateAction::None;
                };
                self.people.push(ContentPersonRole {
                    person_id,
                    person_name,
                    role_id,
                    role_name,
                });
                self.people_add_flow = None;
                ContentCreateAction::None
            }
        }
    }

    fn handle_people_flow_directional_key(&mut self, key: KeyEvent) {
        let Some(flow) = self.people_add_flow.as_mut() else {
            return;
        };

        match flow.step {
            PeopleAddStep::SelectPerson => {
                flow.person_widget.handle_key(key);
            }
            PeopleAddStep::SelectRole => {
                if self.role_options.is_empty() {
                    return;
                }
                match key.code {
                    KeyCode::Up => {
                        flow.role_index = flow.role_index.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        flow.role_index = (flow.role_index + 1).min(self.role_options.len() - 1);
                    }
                    _ => {}
                }
            }
        }
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
                "Tab: avanti | BackTab: indietro | n: aggiungi | d: rimuovi | Enter: conferma | Ctrl+S: salva | Esc: annulla"
            }
            _ => {
                "Tab: avanti | BackTab: indietro | Enter: prossimo campo | Ctrl+S: salva | Esc: annulla"
            }
        };
        frame.render_widget(Paragraph::new(hint), hint_area);

        let outer_block = Block::default()
            .title("Crea Contenuto")
            .borders(Borders::ALL);
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
        let people_display = collection_text_summary(
            self.people
                .iter()
                .map(|entry| format!("{} ({})", entry.person_name, entry.role_name))
                .collect::<Vec<_>>()
                .iter()
                .map(|entry| entry.as_str()),
        );
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
                if let Some(flow) = self.people_add_flow.as_mut() {
                    match flow.step {
                        PeopleAddStep::SelectPerson => {
                            flow.person_widget.render(frame, coll_inner);
                        }
                        PeopleAddStep::SelectRole => {
                            render_role_popup(
                                frame,
                                coll_inner,
                                &self.role_options,
                                flow.role_index,
                            );
                        }
                    }
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
        let draft = self.to_draft();
        if draft.name.is_empty() {
            None
        } else {
            Some(draft.into())
        }
    }

    fn to_draft(&self) -> ContentDraft {
        ContentDraft {
            name: self.title.value.trim().to_string(),
            original_title: None,
            type_id: None,
            genre_id: self.genre.value.trim().parse().ok(),
            publication_date: date_to_opt(self.publication_date.value()),
            notes: to_opt(&self.notes.value),
        }
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

    fn handle_escape(&mut self) -> bool {
        match self.active_field {
            ContentField::PublicationDate => {
                if self.publication_date.active_field != PartialDateField::Year {
                    self.publication_date.active_field = PartialDateField::Year;
                    return true;
                }
            }
            ContentField::People => {
                if let Some(flow) = self.people_add_flow.as_mut() {
                    let _ = flow.person_widget.input.dismiss_suggestions();
                    self.people_add_flow = None;
                    return true;
                }
            }
            ContentField::Languages => {
                if let Some((language, _)) = self.languages.last_mut() {
                    return language.input.dismiss_suggestions();
                }
            }
            _ => {}
        }
        false
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
    let display = if value.trim().is_empty() {
        "—"
    } else {
        value
    };
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

fn render_role_popup(
    frame: &mut Frame,
    area: Rect,
    roles: &[(i64, String)],
    selected_index: usize,
) {
    let items = roles
        .iter()
        .enumerate()
        .map(|(index, (_, role_name))| {
            let style = if index == selected_index {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            ListItem::new(role_name.as_str()).style(style)
        })
        .collect::<Vec<_>>();
    let popup = List::new(items).block(
        Block::default()
            .title("Seleziona ruolo (↑/↓, Enter)")
            .borders(Borders::ALL),
    );
    frame.render_widget(popup, area);
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

impl From<ContentDraft> for Content {
    fn from(value: ContentDraft) -> Self {
        Content {
            id: 0,
            title: value.name,
            publication_year: value.publication_date,
            notes: value.notes,
        }
    }
}

fn partial_date_eq(a: &Option<PartialDate>, b: &Option<PartialDate>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(left), Some(right)) => {
            left.year == right.year
                && left.month == right.month
                && left.day == right.day
                && left.circa == right.circa
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ratatui::{backend::TestBackend, Terminal};

    use crate::widgets::{input::InputWidget, language::LanguageWidget};

    use super::PartialDateField;
    use super::{ContentCreateAction, ContentCreateScreen, ContentDraft, ContentField};

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

        let submit = screen.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL));
        assert_eq!(
            submit,
            ContentCreateAction::Submit(ContentDraft {
                name: "".into(),
                original_title: None,
                type_id: None,
                genre_id: None,
                publication_date: None,
                notes: None,
            })
        );
    }

    #[test]
    fn esc_inside_publication_date_returns_to_year_without_cancelling_screen() {
        let mut screen = ContentCreateScreen::new();
        screen.active_field = ContentField::PublicationDate;
        screen.publication_date.active_field = PartialDateField::Day;

        let action = screen.handle_key(KeyEvent::from(KeyCode::Esc));

        assert_eq!(action, ContentCreateAction::None);
        assert_eq!(screen.active_field, ContentField::PublicationDate);
        assert_eq!(screen.publication_date.active_field, PartialDateField::Year);
    }

    #[test]
    fn esc_closes_people_dropdown_without_cancelling_screen() {
        let mut screen = ContentCreateScreen::new_with_options(
            vec![(1, "Ada Lovelace".into()), (2, "Alan Turing".into())],
            vec![(1, "author".into())],
        );
        screen.active_field = ContentField::People;
        screen.handle_key(KeyEvent::from(KeyCode::Char('n')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('a')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('d')));

        let action = screen.handle_key(KeyEvent::from(KeyCode::Esc));

        assert_eq!(action, ContentCreateAction::None);
        assert!(screen.people_add_flow.is_none());
        assert!(screen.people.is_empty());
    }

    #[test]
    fn esc_closes_languages_dropdown_without_cancelling_screen() {
        let mut screen = ContentCreateScreen::new();
        screen.active_field = ContentField::Languages;
        screen
            .languages
            .push((LanguageWidget::new(), InputWidget::new()));
        let language = &mut screen.languages[0].0;
        language.input.value = "it".into();
        language.input.cursor = 2;
        language.set_suggestions(vec![(1, "Italiano".into()), (2, "English".into())]);
        language.input.selected_suggestion = Some(0);

        let action = screen.handle_key(KeyEvent::from(KeyCode::Esc));

        assert_eq!(action, ContentCreateAction::None);
        assert!(screen.languages[0].0.input.suggestions.is_empty());
        assert_eq!(screen.languages[0].0.input.selected_suggestion, None);
        assert_eq!(screen.languages[0].0.input.value, "it");
    }

    #[test]
    fn collections_support_add_and_remove() {
        let mut screen = ContentCreateScreen::new_with_options(
            vec![(1, "Ada Lovelace".into())],
            vec![(1, "author".into())],
        );

        for _ in 0..4 {
            screen.handle_key(KeyEvent::from(KeyCode::Enter));
        }
        assert_eq!(screen.active_field, ContentField::People);
        screen.handle_key(KeyEvent::from(KeyCode::Char('n')));
        assert!(screen.people_add_flow.is_some());
        screen.handle_key(KeyEvent::from(KeyCode::Char('a')));
        screen.handle_key(KeyEvent::from(KeyCode::Down));
        screen.handle_key(KeyEvent::from(KeyCode::Enter));
        screen.handle_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(screen.people.len(), 1);
        screen.handle_key(KeyEvent::from(KeyCode::Char('d')));
        assert_eq!(screen.people.len(), 0);

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
    fn people_flow_existing_person_then_role_adds_pair() {
        let mut screen = ContentCreateScreen::new_with_options(
            vec![(1, "Ada Lovelace".into()), (2, "Alan Turing".into())],
            vec![(7, "author".into()), (8, "editor".into())],
        );
        screen.active_field = ContentField::People;

        screen.handle_key(KeyEvent::from(KeyCode::Char('n')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('a')));
        screen.handle_key(KeyEvent::from(KeyCode::Down));
        let action = screen.handle_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(action, ContentCreateAction::None);
        assert!(screen.people_add_flow.is_some());

        screen.handle_key(KeyEvent::from(KeyCode::Down));
        let action = screen.handle_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(action, ContentCreateAction::None);
        assert!(screen.people_add_flow.is_none());
        assert_eq!(screen.people.len(), 1);
        assert_eq!(screen.people[0].person_id, 1);
        assert_eq!(screen.people[0].role_id, 8);
    }

    #[test]
    fn people_flow_new_name_emits_create_action() {
        let mut screen = ContentCreateScreen::new_with_options(vec![], vec![(7, "author".into())]);
        screen.active_field = ContentField::People;
        screen.handle_key(KeyEvent::from(KeyCode::Char('n')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('M')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('a')));

        let action = screen.handle_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(
            action,
            ContentCreateAction::CreatePersonForContent("Ma".to_string())
        );
    }

    #[test]
    fn complete_person_creation_moves_to_role_step_and_can_confirm() {
        let mut screen = ContentCreateScreen::new_with_options(vec![], vec![(7, "author".into())]);
        screen.active_field = ContentField::People;
        screen.handle_key(KeyEvent::from(KeyCode::Char('n')));

        screen.complete_person_creation(10, "Nuovo Nome".into());
        let action = screen.handle_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(action, ContentCreateAction::None);
        assert_eq!(screen.people.len(), 1);
        assert_eq!(screen.people[0].person_id, 10);
        assert_eq!(screen.people[0].role_id, 7);
    }

    #[test]
    fn esc_in_role_step_cancels_people_add_flow() {
        let mut screen = ContentCreateScreen::new_with_options(
            vec![(1, "Ada Lovelace".into())],
            vec![(7, "author".into())],
        );
        screen.active_field = ContentField::People;
        screen.handle_key(KeyEvent::from(KeyCode::Char('n')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('a')));
        screen.handle_key(KeyEvent::from(KeyCode::Down));
        screen.handle_key(KeyEvent::from(KeyCode::Enter));

        let action = screen.handle_key(KeyEvent::from(KeyCode::Esc));

        assert_eq!(action, ContentCreateAction::None);
        assert!(screen.people_add_flow.is_none());
        assert!(screen.people.is_empty());
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

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::Frame,
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};
use ritmo_domain::{PartialDate, Person};

use crate::widgets::{
    input::InputWidget,
    language::LanguageWidget,
    partial_date::{PartialDateField, PartialDateWidget},
    place::PlaceWidget,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersonField {
    Name,
    GivenName,
    Surname,
    DisplayName,
    MiddleNames,
    Title,
    Suffix,
    BirthDate,
    DeathDate,
    Biography,
    Aliases,
    Places,
    Languages,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersonCreateAction {
    None,
    Submit(PersonDraft),
    Cancel,
}

#[derive(Debug, Clone)]
pub struct PersonDraft {
    pub name: String,
    pub display_name: Option<String>,
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_names: Option<String>,
    pub title: Option<String>,
    pub suffix: Option<String>,
    pub birth_date: Option<PartialDate>,
    pub death_date: Option<PartialDate>,
    pub biography: Option<String>,
}

impl PartialEq for PersonDraft {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.display_name == other.display_name
            && self.given_name == other.given_name
            && self.surname == other.surname
            && self.middle_names == other.middle_names
            && self.title == other.title
            && self.suffix == other.suffix
            && self.biography == other.biography
            && partial_date_eq(&self.birth_date, &other.birth_date)
            && partial_date_eq(&self.death_date, &other.death_date)
    }
}

impl Eq for PersonDraft {}

#[derive(Debug, Clone)]
pub struct PersonCreateScreen {
    pub active_field: PersonField,
    pub name: InputWidget,
    pub given_name: InputWidget,
    pub surname: InputWidget,
    pub display_name: InputWidget,
    pub middle_names: InputWidget,
    pub title: InputWidget,
    pub suffix: InputWidget,
    pub birth_date: PartialDateWidget,
    pub death_date: PartialDateWidget,
    pub biography: InputWidget,
    pub aliases: Vec<InputWidget>,
    pub places: Vec<PlaceWidget>,
    pub languages: Vec<LanguageWidget>,
}

impl PersonCreateScreen {
    pub fn new() -> Self {
        Self {
            active_field: PersonField::Name,
            name: InputWidget::new(),
            given_name: InputWidget::new(),
            surname: InputWidget::new(),
            display_name: InputWidget::new(),
            middle_names: InputWidget::new(),
            title: InputWidget::new(),
            suffix: InputWidget::new(),
            birth_date: PartialDateWidget::new(),
            death_date: PartialDateWidget::new(),
            biography: InputWidget::new(),
            aliases: Vec::new(),
            places: Vec::new(),
            languages: Vec::new(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> PersonCreateAction {
        if key.code == KeyCode::Esc {
            if self.handle_escape() {
                return PersonCreateAction::None;
            }
            return PersonCreateAction::Cancel;
        }
        if key.code == KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL) {
            return PersonCreateAction::Submit(self.to_draft());
        }
        // Enter always advances to the next field.
        if key.code == KeyCode::Enter {
            self.next_field();
            return PersonCreateAction::None;
        }

        // Date fields: delegate all keys (including Tab/BackTab for sub-field navigation)
        // to the date widget directly.
        match self.active_field {
            PersonField::BirthDate => {
                self.birth_date.handle_key(key);
                return PersonCreateAction::None;
            }
            PersonField::DeathDate => {
                self.death_date.handle_key(key);
                return PersonCreateAction::None;
            }
            _ => {}
        }

        // For non-date fields, Tab/BackTab navigate between PersonFields.
        match key.code {
            KeyCode::Tab => {
                self.next_field();
                return PersonCreateAction::None;
            }
            KeyCode::BackTab => {
                self.previous_field();
                return PersonCreateAction::None;
            }
            _ => {}
        }

        // Delegate remaining keys to the active widget.
        match self.active_field {
            PersonField::Name => self.name.handle_key(key),
            PersonField::GivenName => self.given_name.handle_key(key),
            PersonField::Surname => self.surname.handle_key(key),
            PersonField::DisplayName => self.display_name.handle_key(key),
            PersonField::MiddleNames => self.middle_names.handle_key(key),
            PersonField::Title => self.title.handle_key(key),
            PersonField::Suffix => self.suffix.handle_key(key),
            PersonField::Biography => self.biography.handle_key(key),
            PersonField::Aliases => match key.code {
                KeyCode::Char('n') => self.aliases.push(InputWidget::new()),
                KeyCode::Char('d') | KeyCode::Delete => {
                    self.aliases.pop();
                }
                _ => {
                    if let Some(last) = self.aliases.last_mut() {
                        last.handle_key(key);
                    }
                }
            },
            PersonField::Places => match key.code {
                KeyCode::Char('n') => self.places.push(PlaceWidget::new()),
                KeyCode::Char('d') | KeyCode::Delete => {
                    self.places.pop();
                }
                _ => {
                    if let Some(last) = self.places.last_mut() {
                        last.handle_key(key);
                    }
                }
            },
            PersonField::Languages => match key.code {
                KeyCode::Char('n') => self.languages.push(LanguageWidget::new()),
                KeyCode::Char('d') | KeyCode::Delete => {
                    self.languages.pop();
                }
                _ => {
                    if let Some(last) = self.languages.last_mut() {
                        last.handle_key(key);
                    }
                }
            },
            // Already handled above.
            PersonField::BirthDate | PersonField::DeathDate => {}
        }

        PersonCreateAction::None
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let layout = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(area);
        let form_area = layout[0];
        let hint_area = layout[1];

        let hint = match self.active_field {
            PersonField::BirthDate | PersonField::DeathDate => {
                "Tab/BackTab: sotto-campi data | Enter: prossimo campo | Ctrl+S: salva | Esc: annulla"
            }
            PersonField::Aliases | PersonField::Places | PersonField::Languages => {
                "Tab: avanti | BackTab: indietro | n: aggiungi | d: rimuovi | Ctrl+S: salva | Esc: annulla"
            }
            _ => "Tab: avanti | BackTab: indietro | Enter: prossimo campo | Ctrl+S: salva | Esc: annulla",
        };
        frame.render_widget(Paragraph::new(hint), hint_area);

        let outer_block = Block::default().title("Crea Persona").borders(Borders::ALL);
        let inner = outer_block.inner(form_area);
        frame.render_widget(outer_block, form_area);

        let af = self.active_field;
        let constraints = [
            text_field_height(af, PersonField::Name),
            text_field_height(af, PersonField::GivenName),
            text_field_height(af, PersonField::Surname),
            text_field_height(af, PersonField::DisplayName),
            text_field_height(af, PersonField::MiddleNames),
            text_field_height(af, PersonField::Title),
            text_field_height(af, PersonField::Suffix),
            Constraint::Length(3), // BirthDate: always 3 rows (block border + 1 content)
            Constraint::Length(3), // DeathDate: always 3 rows
            text_field_height(af, PersonField::Biography),
            collection_height(af, PersonField::Aliases),
            collection_height(af, PersonField::Places),
            collection_height(af, PersonField::Languages),
        ];

        let rows = Layout::vertical(constraints).split(inner);

        // Pre-collect values for inactive field display (avoid borrow conflicts).
        let name_val = self.name.value.clone();
        let given_name_val = self.given_name.value.clone();
        let surname_val = self.surname.value.clone();
        let display_name_val = self.display_name.value.clone();
        let middle_names_val = self.middle_names.value.clone();
        let title_val = self.title.value.clone();
        let suffix_val = self.suffix.value.clone();
        let biography_val = self.biography.value.clone();
        let aliases_display =
            collection_text_summary(self.aliases.iter().map(|a| a.value.as_str()));
        let places_count = self.places.len();
        let languages_display =
            collection_text_summary(self.languages.iter().map(|l| l.input.value.as_str()));

        // ── Simple text fields ──────────────────────────────────────────────
        if af == PersonField::Name {
            self.name.render(frame, rows[0]);
        } else {
            render_label_row(frame, rows[0], "Nome", &name_val);
        }

        if af == PersonField::GivenName {
            self.given_name.render(frame, rows[1]);
        } else {
            render_label_row(frame, rows[1], "Nome proprio", &given_name_val);
        }

        if af == PersonField::Surname {
            self.surname.render(frame, rows[2]);
        } else {
            render_label_row(frame, rows[2], "Cognome", &surname_val);
        }

        if af == PersonField::DisplayName {
            self.display_name.render(frame, rows[3]);
        } else {
            render_label_row(frame, rows[3], "Nome visualizzato", &display_name_val);
        }

        if af == PersonField::MiddleNames {
            self.middle_names.render(frame, rows[4]);
        } else {
            render_label_row(frame, rows[4], "Secondo nome", &middle_names_val);
        }

        if af == PersonField::Title {
            self.title.render(frame, rows[5]);
        } else {
            render_label_row(frame, rows[5], "Titolo", &title_val);
        }

        if af == PersonField::Suffix {
            self.suffix.render(frame, rows[6]);
        } else {
            render_label_row(frame, rows[6], "Suffisso", &suffix_val);
        }

        // ── Date fields ─────────────────────────────────────────────────────
        {
            let is_active = af == PersonField::BirthDate;
            let block = labeled_block("Data di nascita", is_active);
            let date_inner = block.inner(rows[7]);
            frame.render_widget(block, rows[7]);
            self.birth_date.render(frame, date_inner, is_active);
        }
        {
            let is_active = af == PersonField::DeathDate;
            let block = labeled_block("Data di morte", is_active);
            let date_inner = block.inner(rows[8]);
            frame.render_widget(block, rows[8]);
            self.death_date.render(frame, date_inner, is_active);
        }

        // ── Biography ───────────────────────────────────────────────────────
        if af == PersonField::Biography {
            self.biography.render(frame, rows[9]);
        } else {
            render_label_row(frame, rows[9], "Biografia", &biography_val);
        }

        // ── Collections ─────────────────────────────────────────────────────
        // Aliases
        {
            let is_active = af == PersonField::Aliases;
            let block = labeled_block("Alias", is_active);
            let coll_inner = block.inner(rows[10]);
            frame.render_widget(block, rows[10]);
            if is_active && !self.aliases.is_empty() {
                self.aliases.last_mut().unwrap().render(frame, coll_inner);
            } else {
                frame.render_widget(Paragraph::new(aliases_display), coll_inner);
            }
        }
        // Places
        {
            let is_active = af == PersonField::Places;
            let block = labeled_block("Luoghi", is_active);
            let coll_inner = block.inner(rows[11]);
            frame.render_widget(block, rows[11]);
            if is_active && !self.places.is_empty() {
                self.places.last().unwrap().render(frame, coll_inner);
            } else {
                let text = if places_count == 0 {
                    "—".to_string()
                } else {
                    format!("{places_count} luogo/luoghi")
                };
                frame.render_widget(Paragraph::new(text), coll_inner);
            }
        }
        // Languages
        {
            let is_active = af == PersonField::Languages;
            let block = labeled_block("Lingue", is_active);
            let coll_inner = block.inner(rows[12]);
            frame.render_widget(block, rows[12]);
            if is_active && !self.languages.is_empty() {
                self.languages.last_mut().unwrap().render(frame, coll_inner);
            } else {
                frame.render_widget(Paragraph::new(languages_display), coll_inner);
            }
        }
    }

    pub fn to_person(&self) -> Option<Person> {
        let draft = self.to_draft();
        if draft.name.is_empty() {
            return None;
        }
        Some(draft.into())
    }

    fn to_draft(&self) -> PersonDraft {
        PersonDraft {
            name: self.name.value.trim().to_string(),
            display_name: to_opt(&self.display_name.value),
            given_name: to_opt(&self.given_name.value),
            surname: to_opt(&self.surname.value),
            middle_names: to_opt(&self.middle_names.value),
            title: to_opt(&self.title.value),
            suffix: to_opt(&self.suffix.value),
            birth_date: date_to_opt(self.birth_date.value()),
            death_date: date_to_opt(self.death_date.value()),
            biography: to_opt(&self.biography.value),
        }
    }

    fn next_field(&mut self) {
        self.active_field = match self.active_field {
            PersonField::Name => PersonField::GivenName,
            PersonField::GivenName => PersonField::Surname,
            PersonField::Surname => PersonField::DisplayName,
            PersonField::DisplayName => PersonField::MiddleNames,
            PersonField::MiddleNames => PersonField::Title,
            PersonField::Title => PersonField::Suffix,
            PersonField::Suffix => PersonField::BirthDate,
            PersonField::BirthDate => PersonField::DeathDate,
            PersonField::DeathDate => PersonField::Biography,
            PersonField::Biography => PersonField::Aliases,
            PersonField::Aliases => PersonField::Places,
            PersonField::Places => PersonField::Languages,
            PersonField::Languages => PersonField::Name,
        };
    }

    fn previous_field(&mut self) {
        self.active_field = match self.active_field {
            PersonField::Name => PersonField::Languages,
            PersonField::GivenName => PersonField::Name,
            PersonField::Surname => PersonField::GivenName,
            PersonField::DisplayName => PersonField::Surname,
            PersonField::MiddleNames => PersonField::DisplayName,
            PersonField::Title => PersonField::MiddleNames,
            PersonField::Suffix => PersonField::Title,
            PersonField::BirthDate => PersonField::Suffix,
            PersonField::DeathDate => PersonField::BirthDate,
            PersonField::Biography => PersonField::DeathDate,
            PersonField::Aliases => PersonField::Biography,
            PersonField::Places => PersonField::Aliases,
            PersonField::Languages => PersonField::Places,
        };
    }

    fn handle_escape(&mut self) -> bool {
        match self.active_field {
            PersonField::BirthDate => {
                if self.birth_date.active_field != PartialDateField::Year {
                    self.birth_date.active_field = PartialDateField::Year;
                    return true;
                }
            }
            PersonField::DeathDate => {
                if self.death_date.active_field != PartialDateField::Year {
                    self.death_date.active_field = PartialDateField::Year;
                    return true;
                }
            }
            PersonField::Languages => {
                if let Some(language) = self.languages.last_mut() {
                    return language.input.dismiss_suggestions();
                }
            }
            _ => {}
        }
        false
    }
}

impl Default for PersonCreateScreen {
    fn default() -> Self {
        Self::new()
    }
}

// ── Helper functions ────────────────────────────────────────────────────────

fn text_field_height(active: PersonField, field: PersonField) -> Constraint {
    if active == field {
        Constraint::Length(3)
    } else {
        Constraint::Length(1)
    }
}

fn collection_height(active: PersonField, field: PersonField) -> Constraint {
    // Active collection: 5 rows (2 for border + 3 for the inner widget/text).
    // Inactive: 1 row for a compact summary line.
    if active == field {
        Constraint::Length(5)
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

impl From<PersonDraft> for Person {
    fn from(value: PersonDraft) -> Self {
        Person {
            id: 0,
            name: value.name,
            display_name: value.display_name,
            given_name: value.given_name,
            surname: value.surname,
            middle_names: value.middle_names,
            title: value.title,
            suffix: value.suffix,
            birth_date: value.birth_date,
            death_date: value.death_date,
            biography: value.biography,
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

    use super::{PersonCreateAction, PersonCreateScreen, PersonDraft, PersonField};

    #[test]
    fn new_starts_at_name_field_with_empty_state() {
        let screen = PersonCreateScreen::new();

        assert_eq!(screen.active_field, PersonField::Name);
        assert!(screen.name.value.is_empty());
        assert!(screen.aliases.is_empty());
        assert!(screen.places.is_empty());
        assert!(screen.languages.is_empty());
        assert_eq!(screen.birth_date.year, None);
        assert_eq!(screen.death_date.year, None);
    }

    #[test]
    fn tab_advances_non_date_fields_enter_advances_date_fields() {
        let mut screen = PersonCreateScreen::new();

        // Tab advances through non-date fields normally.
        let tab_steps = [
            PersonField::GivenName,
            PersonField::Surname,
            PersonField::DisplayName,
            PersonField::MiddleNames,
            PersonField::Title,
            PersonField::Suffix,
            PersonField::BirthDate,
        ];
        for expected in tab_steps {
            screen.handle_key(KeyEvent::from(KeyCode::Tab));
            assert_eq!(screen.active_field, expected);
        }

        // Tab in a date field advances its sub-fields, not PersonField.
        // Enter must be used to leave a date field.
        screen.handle_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(screen.active_field, PersonField::DeathDate);

        screen.handle_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(screen.active_field, PersonField::Biography);

        // After the date fields, Tab works normally again.
        screen.handle_key(KeyEvent::from(KeyCode::Tab));
        assert_eq!(screen.active_field, PersonField::Aliases);

        screen.handle_key(KeyEvent::from(KeyCode::Tab));
        assert_eq!(screen.active_field, PersonField::Places);

        screen.handle_key(KeyEvent::from(KeyCode::Tab));
        assert_eq!(screen.active_field, PersonField::Languages);

        // Tab wraps back to Name.
        screen.handle_key(KeyEvent::from(KeyCode::Tab));
        assert_eq!(screen.active_field, PersonField::Name);
    }

    #[test]
    fn enter_advances_through_all_fields_and_wraps() {
        let mut screen = PersonCreateScreen::new();

        // Enter always advances PersonField regardless of the active widget type.
        let expected_order = [
            PersonField::GivenName,
            PersonField::Surname,
            PersonField::DisplayName,
            PersonField::MiddleNames,
            PersonField::Title,
            PersonField::Suffix,
            PersonField::BirthDate,
            PersonField::DeathDate,
            PersonField::Biography,
            PersonField::Aliases,
            PersonField::Places,
            PersonField::Languages,
            PersonField::Name, // wraps back
        ];

        for expected in expected_order {
            screen.handle_key(KeyEvent::from(KeyCode::Enter));
            assert_eq!(screen.active_field, expected);
        }
    }

    #[test]
    fn backtab_goes_to_previous_field_and_wraps() {
        let mut screen = PersonCreateScreen::new();

        screen.handle_key(KeyEvent::from(KeyCode::BackTab));
        assert_eq!(screen.active_field, PersonField::Languages);

        screen.handle_key(KeyEvent::from(KeyCode::BackTab));
        assert_eq!(screen.active_field, PersonField::Places);
    }

    #[test]
    fn enter_advances_to_next_field() {
        let mut screen = PersonCreateScreen::new();

        screen.handle_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(screen.active_field, PersonField::GivenName);
    }

    #[test]
    fn esc_returns_cancel_action_when_no_popup_is_open() {
        let mut screen = PersonCreateScreen::new();

        let action = screen.handle_key(KeyEvent::from(KeyCode::Esc));

        assert_eq!(action, PersonCreateAction::Cancel);
    }

    #[test]
    fn ctrl_s_returns_submit_action() {
        let mut screen = PersonCreateScreen::new();

        let action = screen.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL));

        assert_eq!(
            action,
            PersonCreateAction::Submit(PersonDraft {
                name: "".into(),
                display_name: None,
                given_name: None,
                surname: None,
                middle_names: None,
                title: None,
                suffix: None,
                birth_date: None,
                death_date: None,
                biography: None,
            })
        );
    }

    #[test]
    fn typing_populates_name_field() {
        let mut screen = PersonCreateScreen::new();

        screen.handle_key(KeyEvent::from(KeyCode::Char('A')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('d')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('a')));

        assert_eq!(screen.name.value, "Ada");
    }

    #[test]
    fn typing_in_date_field_delegates_to_partial_date_widget() {
        let mut screen = PersonCreateScreen::new();

        // Navigate to BirthDate
        for _ in 0..7 {
            screen.handle_key(KeyEvent::from(KeyCode::Tab));
        }
        assert_eq!(screen.active_field, PersonField::BirthDate);

        // Type a year
        screen.handle_key(KeyEvent::from(KeyCode::Char('1')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('9')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('8')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('0')));

        assert_eq!(screen.birth_date.year, Some(1980));
    }

    #[test]
    fn tab_in_date_field_navigates_sub_fields_not_person_fields() {
        let mut screen = PersonCreateScreen::new();

        // Navigate to BirthDate
        for _ in 0..7 {
            screen.handle_key(KeyEvent::from(KeyCode::Tab));
        }
        assert_eq!(screen.active_field, PersonField::BirthDate);

        // Tab while in BirthDate → goes to date sub-field (Month), NOT to DeathDate
        screen.handle_key(KeyEvent::from(KeyCode::Tab));
        assert_eq!(screen.active_field, PersonField::BirthDate);

        // Enter advances to next PersonField
        screen.handle_key(KeyEvent::from(KeyCode::Enter));
        assert_eq!(screen.active_field, PersonField::DeathDate);
    }

    #[test]
    fn aliases_can_be_added_and_removed() {
        let mut screen = PersonCreateScreen::new();

        // Navigate to Aliases using Enter (advances PersonField regardless of active widget).
        for _ in 0..10 {
            screen.handle_key(KeyEvent::from(KeyCode::Enter));
        }
        assert_eq!(screen.active_field, PersonField::Aliases);

        screen.handle_key(KeyEvent::from(KeyCode::Char('n')));
        assert_eq!(screen.aliases.len(), 1);

        screen.handle_key(KeyEvent::from(KeyCode::Char('n')));
        assert_eq!(screen.aliases.len(), 2);

        screen.handle_key(KeyEvent::from(KeyCode::Char('d')));
        assert_eq!(screen.aliases.len(), 1);
    }

    #[test]
    fn typing_populates_active_alias() {
        let mut screen = PersonCreateScreen::new();

        // Use Enter to navigate to Aliases (Enter always advances PersonField).
        for _ in 0..10 {
            screen.handle_key(KeyEvent::from(KeyCode::Enter));
        }
        screen.handle_key(KeyEvent::from(KeyCode::Char('n')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('A')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('L')));

        assert_eq!(screen.aliases[0].value, "AL");
    }

    #[test]
    fn to_person_returns_none_when_name_is_empty() {
        let screen = PersonCreateScreen::new();

        assert!(screen.to_person().is_none());
    }

    #[test]
    fn to_person_returns_person_with_valid_name() {
        let mut screen = PersonCreateScreen::new();
        screen.name.value = "Ada".to_string();
        screen.name.cursor = 3;

        let person = screen.to_person().unwrap();

        assert_eq!(person.id, 0);
        assert_eq!(person.name, "Ada");
        assert_eq!(person.given_name, None);
        assert_eq!(person.display_name, None);
        assert_eq!(person.biography, None);
    }

    #[test]
    fn to_person_maps_optional_fields() {
        let mut screen = PersonCreateScreen::new();
        screen.name.value = "Lovelace".to_string();
        screen.display_name.value = "Ada Lovelace".to_string();
        screen.given_name.value = "Ada".to_string();
        screen.surname.value = "Lovelace".to_string();
        screen.biography.value = "Matematica".to_string();

        let person = screen.to_person().unwrap();

        assert_eq!(person.display_name.as_deref(), Some("Ada Lovelace"));
        assert_eq!(person.given_name.as_deref(), Some("Ada"));
        assert_eq!(person.surname.as_deref(), Some("Lovelace"));
        assert_eq!(person.biography.as_deref(), Some("Matematica"));
    }

    #[test]
    fn to_person_ignores_whitespace_only_fields() {
        let mut screen = PersonCreateScreen::new();
        screen.name.value = "Ada".to_string();
        screen.surname.value = "  ".to_string();
        screen.title.value = "\t".to_string();

        let person = screen.to_person().unwrap();

        assert_eq!(person.surname, None);
        assert_eq!(person.title, None);
    }

    #[test]
    fn to_person_includes_birth_date_when_set() {
        use crossterm::event::KeyCode;
        let mut screen = PersonCreateScreen::new();
        screen.name.value = "Ada".to_string();

        // Set birth year directly via the widget
        for _ in 0..7 {
            screen.handle_key(KeyEvent::from(KeyCode::Tab));
        }
        screen.handle_key(KeyEvent::from(KeyCode::Char('1')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('8')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('1')));
        screen.handle_key(KeyEvent::from(KeyCode::Char('5')));

        let person = screen.to_person().unwrap();
        let birth = person.birth_date.unwrap();
        assert_eq!(birth.year, Some(1815));
    }

    #[test]
    fn render_produces_output_with_all_field_labels() {
        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut screen = PersonCreateScreen::new();
        screen.name.value = "Ada".to_string();

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

        assert!(rendered.contains("Crea Persona"));
        assert!(rendered.contains("Nome proprio"));
        assert!(rendered.contains("Cognome"));
        assert!(rendered.contains("Data di nascita"));
        assert!(rendered.contains("Data di morte"));
        assert!(rendered.contains("Biografia"));
        assert!(rendered.contains("Alias"));
        assert!(rendered.contains("Luoghi"));
        assert!(rendered.contains("Lingue"));
    }

    #[test]
    fn esc_in_birth_date_subfield_returns_to_year_without_cancelling() {
        let mut screen = PersonCreateScreen::new();
        screen.active_field = PersonField::BirthDate;
        screen.birth_date.active_field = super::PartialDateField::Day;

        let action = screen.handle_key(KeyEvent::from(KeyCode::Esc));

        assert_eq!(action, PersonCreateAction::None);
        assert_eq!(screen.active_field, PersonField::BirthDate);
        assert_eq!(
            screen.birth_date.active_field,
            super::PartialDateField::Year
        );
    }

    #[test]
    fn esc_closes_language_dropdown_without_cancelling() {
        let mut screen = PersonCreateScreen::new();
        screen.active_field = PersonField::Languages;
        screen
            .languages
            .push(crate::widgets::language::LanguageWidget::new());
        let language = &mut screen.languages[0];
        language.input.value = "it".into();
        language.input.cursor = 2;
        language.set_suggestions(vec![(1, "Italiano".into()), (2, "English".into())]);
        language.input.selected_suggestion = Some(0);

        let action = screen.handle_key(KeyEvent::from(KeyCode::Esc));

        assert_eq!(action, PersonCreateAction::None);
        assert!(screen.languages[0].input.suggestions.is_empty());
        assert_eq!(screen.languages[0].input.selected_suggestion, None);
        assert_eq!(screen.languages[0].input.value, "it");
    }
}

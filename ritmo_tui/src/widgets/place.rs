use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::{Constraint, Frame, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};
use ritmo_domain::Place;

use crate::widgets::input::InputWidget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceField {
    Continent,
    Country,
    City,
    Circa,
    Disputed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceWidget {
    pub continent: InputWidget,
    pub country: InputWidget,
    pub city: InputWidget,
    pub circa: bool,
    pub disputed: bool,
    pub active_field: PlaceField,
}

impl PlaceWidget {
    pub fn new() -> Self {
        Self {
            continent: InputWidget::new(),
            country: InputWidget::new(),
            city: InputWidget::new(),
            circa: false,
            disputed: false,
            active_field: PlaceField::Continent,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab if key.modifiers.contains(KeyModifiers::ALT) => self.previous_field(),
            KeyCode::Tab => self.next_field(),
            KeyCode::Char(' ') => match self.active_field {
                PlaceField::Circa => self.circa = !self.circa,
                PlaceField::Disputed => self.disputed = !self.disputed,
                _ => {}
            },
            KeyCode::Char(c) => {
                if let Some(input) = self.active_input_mut() {
                    input.handle_char(c);
                }
            }
            KeyCode::Backspace => {
                if let Some(input) = self.active_input_mut() {
                    input.handle_backspace();
                }
            }
            KeyCode::Up => {
                if let Some(input) = self.active_input_mut() {
                    input.handle_up();
                }
            }
            KeyCode::Down => {
                if let Some(input) = self.active_input_mut() {
                    input.handle_down();
                }
            }
            _ => {}
        }
    }

    pub fn value(&self) -> Place {
        Place {
            id: 0,
            continent: value_to_option(&self.continent.value),
            country: value_to_option(&self.country.value),
            city: value_to_option(&self.city.value),
            circa: self.circa,
            disputed: self.disputed,
        }
    }

    pub fn set_value(&mut self, place: Place) {
        set_input_value(
            &mut self.continent,
            place.continent.as_deref().unwrap_or_default(),
        );
        set_input_value(
            &mut self.country,
            place.country.as_deref().unwrap_or_default(),
        );
        set_input_value(&mut self.city, place.city.as_deref().unwrap_or_default());
        self.circa = place.circa;
        self.disputed = place.disputed;
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let chunks = Layout::horizontal([
            Constraint::Min(14),
            Constraint::Min(14),
            Constraint::Min(14),
            Constraint::Length(12),
            Constraint::Length(16),
        ])
        .split(area);

        render_text_field(
            frame,
            chunks[0],
            "Continente",
            &self.continent.value,
            self.active_field == PlaceField::Continent,
        );
        render_text_field(
            frame,
            chunks[1],
            "Paese",
            &self.country.value,
            self.active_field == PlaceField::Country,
        );
        render_text_field(
            frame,
            chunks[2],
            "Città",
            &self.city.value,
            self.active_field == PlaceField::City,
        );
        render_checkbox(
            frame,
            chunks[3],
            "Circa",
            self.circa,
            self.active_field == PlaceField::Circa,
        );
        render_checkbox(
            frame,
            chunks[4],
            "Disputato",
            self.disputed,
            self.active_field == PlaceField::Disputed,
        );

        if let Some((area, cursor)) = self.active_input_with_area(chunks) {
            let cursor_x = area.x + 1 + cursor.min(area.width.saturating_sub(2) as usize) as u16;
            let cursor_y = area.y + 1;
            frame.set_cursor_position((cursor_x, cursor_y));
        }
    }

    fn active_input_mut(&mut self) -> Option<&mut InputWidget> {
        match self.active_field {
            PlaceField::Continent => Some(&mut self.continent),
            PlaceField::Country => Some(&mut self.country),
            PlaceField::City => Some(&mut self.city),
            PlaceField::Circa | PlaceField::Disputed => None,
        }
    }

    fn active_input_with_area(&self, chunks: std::rc::Rc<[Rect]>) -> Option<(Rect, usize)> {
        match self.active_field {
            PlaceField::Continent => Some((chunks[0], self.continent.cursor)),
            PlaceField::Country => Some((chunks[1], self.country.cursor)),
            PlaceField::City => Some((chunks[2], self.city.cursor)),
            PlaceField::Circa | PlaceField::Disputed => None,
        }
    }

    fn next_field(&mut self) {
        self.active_field = match self.active_field {
            PlaceField::Continent => PlaceField::Country,
            PlaceField::Country => PlaceField::City,
            PlaceField::City => PlaceField::Circa,
            PlaceField::Circa => PlaceField::Disputed,
            PlaceField::Disputed => PlaceField::Continent,
        };
    }

    fn previous_field(&mut self) {
        self.active_field = match self.active_field {
            PlaceField::Continent => PlaceField::Disputed,
            PlaceField::Country => PlaceField::Continent,
            PlaceField::City => PlaceField::Country,
            PlaceField::Circa => PlaceField::City,
            PlaceField::Disputed => PlaceField::Circa,
        };
    }
}

fn value_to_option(value: &str) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn set_input_value(input: &mut InputWidget, value: &str) {
    input.value = value.to_string();
    input.cursor = input.value.chars().count();
    input.suggestions.clear();
    input.selected_suggestion = None;
}

fn render_text_field(frame: &mut Frame, area: Rect, label: &str, value: &str, active: bool) {
    let mut block = Block::default().borders(Borders::ALL).title(label);
    if active {
        block = block.style(Style::default().add_modifier(Modifier::REVERSED));
    }
    frame.render_widget(Paragraph::new(value.to_string()).block(block), area);
}

fn render_checkbox(frame: &mut Frame, area: Rect, label: &str, checked: bool, active: bool) {
    let marker = if checked { "x" } else { " " };
    let mut block = Block::default().borders(Borders::ALL);
    if active {
        block = block.style(Style::default().add_modifier(Modifier::REVERSED));
    }
    frame.render_widget(
        Paragraph::new(format!("{label}: [{marker}]")).block(block),
        area,
    );
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ratatui::{backend::TestBackend, Terminal};
    use ritmo_domain::Place;

    use super::{PlaceField, PlaceWidget};

    #[test]
    fn tab_and_alt_tab_cycle_fields() {
        let mut widget = PlaceWidget::new();
        assert_eq!(widget.active_field, PlaceField::Continent);

        widget.handle_key(KeyEvent::from(KeyCode::Tab));
        assert_eq!(widget.active_field, PlaceField::Country);

        widget.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::ALT));
        assert_eq!(widget.active_field, PlaceField::Continent);
    }

    #[test]
    fn space_toggles_only_active_checkbox() {
        let mut widget = PlaceWidget::new();
        widget.active_field = PlaceField::Circa;
        widget.handle_key(KeyEvent::from(KeyCode::Char(' ')));
        assert!(widget.circa);
        assert!(!widget.disputed);

        widget.active_field = PlaceField::Disputed;
        widget.handle_key(KeyEvent::from(KeyCode::Char(' ')));
        assert!(widget.disputed);
    }

    #[test]
    fn text_keys_are_delegated_to_active_input() {
        let mut widget = PlaceWidget::new();
        widget.active_field = PlaceField::City;

        widget.handle_key(KeyEvent::from(KeyCode::Char('R')));
        widget.handle_key(KeyEvent::from(KeyCode::Char('o')));
        widget.handle_key(KeyEvent::from(KeyCode::Backspace));

        assert_eq!(widget.city.value, "R");
        assert_eq!(widget.city.cursor, 1);
    }

    #[test]
    fn value_and_set_value_roundtrip_place() {
        let mut widget = PlaceWidget::new();
        widget.set_value(Place {
            id: 99,
            continent: Some("Europa".into()),
            country: Some("Italia".into()),
            city: Some("Roma".into()),
            circa: true,
            disputed: true,
        });

        let place = widget.value();
        assert_eq!(place.id, 0);
        assert_eq!(place.continent.as_deref(), Some("Europa"));
        assert_eq!(place.country.as_deref(), Some("Italia"));
        assert_eq!(place.city.as_deref(), Some("Roma"));
        assert!(place.circa);
        assert!(place.disputed);
    }

    #[test]
    fn render_shows_labels_and_values() {
        let backend = TestBackend::new(100, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut widget = PlaceWidget::new();
        widget.continent.value = "Europa".into();
        widget.country.value = "Italia".into();
        widget.city.value = "Roma".into();
        widget.circa = true;

        terminal
            .draw(|frame| widget.render(frame, frame.area()))
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

        assert!(rendered.contains("Continente"));
        assert!(rendered.contains("Paese"));
        assert!(rendered.contains("Città"));
        assert!(rendered.contains("Europa"));
        assert!(rendered.contains("Italia"));
        assert!(rendered.contains("Roma"));
        assert!(rendered.contains("Circa: [x]"));
        assert!(rendered.contains("Disputato: [ ]"));
    }
}

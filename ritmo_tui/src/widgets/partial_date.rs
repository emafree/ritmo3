use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::{Frame, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
use ritmo_domain::PartialDate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartialDateField {
    Year,
    Month,
    Day,
    Circa,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialDateWidget {
    pub year: Option<i32>,
    pub month: Option<u8>,
    pub day: Option<u8>,
    pub circa: bool,
    pub active_field: PartialDateField,
    year_input: String,
    month_input: String,
    day_input: String,
}

impl PartialDateWidget {
    pub fn new() -> Self {
        Self {
            year: None,
            month: None,
            day: None,
            circa: false,
            active_field: PartialDateField::Year,
            year_input: String::new(),
            month_input: String::new(),
            day_input: String::new(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::BackTab => self.previous_field(),
            KeyCode::Tab if key.modifiers.contains(KeyModifiers::ALT) => self.previous_field(),
            KeyCode::Tab => self.next_field(),
            KeyCode::Backspace => self.handle_backspace(),
            KeyCode::Char(' ') if self.active_field == PartialDateField::Circa => {
                self.circa = !self.circa;
            }
            KeyCode::Char(c) if c.is_ascii_digit() || c == '-' => self.handle_char(c),
            _ => {}
        }
    }

    pub fn value(&self) -> PartialDate {
        PartialDate {
            year: self.year,
            month: self.month,
            day: self.day,
            circa: self.circa,
        }
    }

    pub fn set_value(&mut self, date: PartialDate) {
        self.year = date.year;
        self.month = date.month;
        self.day = date.day;
        self.circa = date.circa;
        self.year_input = date.year.map(|value| value.to_string()).unwrap_or_default();
        self.month_input = date
            .month
            .map(|value| value.to_string())
            .unwrap_or_default();
        self.day_input = date.day.map(|value| value.to_string()).unwrap_or_default();
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let year_width = self.year_input.len().max(4);
        let month_width = 2;
        let day_width = 2;

        let year_text = padded_field(&self.year_input, year_width);
        let month_text = padded_field(&self.month_input, month_width);
        let day_text = padded_field(&self.day_input, day_width);
        let circa_text = if self.circa { "x" } else { " " };

        let line = Line::from(vec![
            Span::raw("Anno: ["),
            styled_field(year_text, self.active_field == PartialDateField::Year),
            Span::raw("] Mese: ["),
            styled_field(month_text, self.active_field == PartialDateField::Month),
            Span::raw("] Giorno: ["),
            styled_field(day_text, self.active_field == PartialDateField::Day),
            Span::raw("] Circa: ["),
            styled_field(
                circa_text.to_string(),
                self.active_field == PartialDateField::Circa,
            ),
            Span::raw("]"),
        ]);

        frame.render_widget(Paragraph::new(line), area);

        let cursor_x = match self.active_field {
            PartialDateField::Year => area.x + 7 + self.year_input.len() as u16,
            PartialDateField::Month => area.x + 18 + self.month_input.len() as u16,
            PartialDateField::Day => area.x + 31 + self.day_input.len() as u16,
            PartialDateField::Circa => area.x + 43,
        };
        frame.set_cursor_position((cursor_x.min(area.x + area.width.saturating_sub(1)), area.y));
    }

    fn next_field(&mut self) {
        self.active_field = match self.active_field {
            PartialDateField::Year => PartialDateField::Month,
            PartialDateField::Month => PartialDateField::Day,
            PartialDateField::Day => PartialDateField::Circa,
            PartialDateField::Circa => PartialDateField::Year,
        };
    }

    fn previous_field(&mut self) {
        self.active_field = match self.active_field {
            PartialDateField::Year => PartialDateField::Circa,
            PartialDateField::Month => PartialDateField::Year,
            PartialDateField::Day => PartialDateField::Month,
            PartialDateField::Circa => PartialDateField::Day,
        };
    }

    fn handle_char(&mut self, c: char) {
        match self.active_field {
            PartialDateField::Year => self.handle_year_char(c),
            PartialDateField::Month => self.handle_bounded_char(c, 12, DatePart::Month),
            PartialDateField::Day => self.handle_bounded_char(c, 31, DatePart::Day),
            PartialDateField::Circa => {}
        }
    }

    fn handle_backspace(&mut self) {
        match self.active_field {
            PartialDateField::Year => {
                self.year_input.pop();
                self.year = parse_year(&self.year_input);
            }
            PartialDateField::Month => {
                self.month_input.pop();
                self.month = parse_bounded(&self.month_input, 12);
            }
            PartialDateField::Day => {
                self.day_input.pop();
                self.day = parse_bounded(&self.day_input, 31);
            }
            PartialDateField::Circa => {}
        }
    }

    fn handle_year_char(&mut self, c: char) {
        let mut candidate = self.year_input.clone();
        if c == '-' {
            if candidate.is_empty() {
                candidate.push(c);
            } else {
                return;
            }
        } else {
            candidate.push(c);
        }

        if candidate == "-" || candidate.parse::<i32>().is_ok() {
            self.year_input = candidate;
            self.year = parse_year(&self.year_input);
        }
    }

    fn handle_bounded_char(&mut self, c: char, max: u8, part: DatePart) {
        if !c.is_ascii_digit() {
            return;
        }

        let buffer = match part {
            DatePart::Month => &self.month_input,
            DatePart::Day => &self.day_input,
        };
        if buffer.len() >= 2 {
            return;
        }

        let candidate = format!("{buffer}{c}");
        let Some(value) = parse_bounded(&candidate, max) else {
            return;
        };

        match part {
            DatePart::Month => {
                self.month_input = candidate;
                self.month = Some(value);
            }
            DatePart::Day => {
                self.day_input = candidate;
                self.day = Some(value);
            }
        }
    }
}

impl Default for PartialDateWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
enum DatePart {
    Month,
    Day,
}

fn styled_field(value: String, is_active: bool) -> Span<'static> {
    if is_active {
        Span::styled(value, Style::default().add_modifier(Modifier::REVERSED))
    } else {
        Span::raw(value)
    }
}

fn padded_field(value: &str, width: usize) -> String {
    let padding = width.saturating_sub(value.len());
    format!("{value}{}", " ".repeat(padding))
}

fn parse_year(value: &str) -> Option<i32> {
    if value.is_empty() || value == "-" {
        None
    } else {
        value.parse::<i32>().ok()
    }
}

fn parse_bounded(value: &str, max: u8) -> Option<u8> {
    let parsed = value.parse::<u8>().ok()?;
    (1..=max).contains(&parsed).then_some(parsed)
}

#[cfg(test)]
mod tests {
    use super::{PartialDateField, PartialDateWidget};
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ratatui::{backend::TestBackend, Terminal};
    use ritmo_domain::PartialDate;

    #[test]
    fn new_starts_empty_with_year_active() {
        let widget = PartialDateWidget::new();

        assert_eq!(widget.year, None);
        assert_eq!(widget.month, None);
        assert_eq!(widget.day, None);
        assert!(!widget.circa);
        assert_eq!(widget.active_field, PartialDateField::Year);
        let value = widget.value();
        assert_eq!(value.year, None);
        assert_eq!(value.month, None);
        assert_eq!(value.day, None);
        assert!(!value.circa);
    }

    #[test]
    fn year_accepts_negative_values_and_backspace_clears_partial_input() {
        let mut widget = PartialDateWidget::new();

        widget.handle_key(KeyEvent::from(KeyCode::Char('-')));
        assert_eq!(widget.year, None);

        widget.handle_key(KeyEvent::from(KeyCode::Char('4')));
        widget.handle_key(KeyEvent::from(KeyCode::Char('2')));
        assert_eq!(widget.year, Some(-42));

        widget.handle_key(KeyEvent::from(KeyCode::Backspace));
        assert_eq!(widget.year, Some(-4));

        widget.handle_key(KeyEvent::from(KeyCode::Backspace));
        assert_eq!(widget.year, None);
    }

    #[test]
    fn month_and_day_ignore_out_of_range_values() {
        let mut widget = PartialDateWidget::new();

        widget.handle_key(KeyEvent::from(KeyCode::Tab));
        widget.handle_key(KeyEvent::from(KeyCode::Char('1')));
        widget.handle_key(KeyEvent::from(KeyCode::Char('3')));
        assert_eq!(widget.month, Some(1));

        widget.handle_key(KeyEvent::from(KeyCode::Backspace));
        widget.handle_key(KeyEvent::from(KeyCode::Char('9')));
        assert_eq!(widget.month, Some(9));

        widget.handle_key(KeyEvent::from(KeyCode::Tab));
        widget.handle_key(KeyEvent::from(KeyCode::Char('3')));
        widget.handle_key(KeyEvent::from(KeyCode::Char('2')));
        assert_eq!(widget.day, Some(3));
    }

    #[test]
    fn tab_and_alt_tab_cycle_fields() {
        let mut widget = PartialDateWidget::new();

        widget.handle_key(KeyEvent::from(KeyCode::Tab));
        widget.handle_key(KeyEvent::from(KeyCode::Tab));
        assert_eq!(widget.active_field, PartialDateField::Day);

        widget.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::ALT));
        assert_eq!(widget.active_field, PartialDateField::Month);

        widget.handle_key(KeyEvent::new(KeyCode::BackTab, KeyModifiers::NONE));
        assert_eq!(widget.active_field, PartialDateField::Year);
    }

    #[test]
    fn space_only_toggles_circa_on_circa_field() {
        let mut widget = PartialDateWidget::new();

        widget.handle_key(KeyEvent::from(KeyCode::Char(' ')));
        assert!(!widget.circa);

        for _ in 0..3 {
            widget.handle_key(KeyEvent::from(KeyCode::Tab));
        }
        assert_eq!(widget.active_field, PartialDateField::Circa);

        widget.handle_key(KeyEvent::from(KeyCode::Char(' ')));
        assert!(widget.circa);
    }

    #[test]
    fn set_value_populates_buffers_and_value_round_trips() {
        let mut widget = PartialDateWidget::new();
        let date = PartialDate {
            year: Some(-1200),
            month: Some(7),
            day: Some(9),
            circa: true,
        };

        widget.set_value(date.clone());

        assert_eq!(widget.year, Some(-1200));
        assert_eq!(widget.month, Some(7));
        assert_eq!(widget.day, Some(9));
        assert!(widget.circa);
        let value = widget.value();
        assert_eq!(value.year, date.year);
        assert_eq!(value.month, date.month);
        assert_eq!(value.day, date.day);
        assert_eq!(value.circa, date.circa);
    }

    #[test]
    fn render_shows_partial_date_layout() {
        let backend = TestBackend::new(60, 1);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut widget = PartialDateWidget::new();
        widget.set_value(PartialDate {
            year: Some(2024),
            month: Some(5),
            day: Some(14),
            circa: true,
        });

        terminal
            .draw(|frame| widget.render(frame, frame.area()))
            .unwrap();

        let buffer = terminal.backend().buffer().clone();
        let rendered = (0..buffer.area.width)
            .map(|x| buffer[(x, 0)].symbol())
            .collect::<String>();

        assert!(rendered.contains("Anno: [2024]"));
        assert!(rendered.contains("Mese: [5 "));
        assert!(rendered.contains("Giorno: [14]"));
        assert!(rendered.contains("Circa: [x]"));
    }
}

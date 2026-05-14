use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::{Constraint, Frame, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InputWidget {
    pub value: String,
    pub cursor: usize,
    pub suggestions: Vec<String>,
    pub selected_suggestion: Option<usize>,
}

impl InputWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => self.handle_char(c),
            KeyCode::Backspace => self.handle_backspace(),
            KeyCode::Up => self.handle_up(),
            KeyCode::Down => self.handle_down(),
            _ => {}
        }
    }

    pub fn handle_char(&mut self, c: char) {
        self.value.push(c);
        self.cursor = self.value.chars().count();
        self.selected_suggestion = None;
    }

    pub fn handle_backspace(&mut self) {
        self.value.pop();
        self.cursor = self.value.chars().count();
    }

    pub fn handle_up(&mut self) {
        if self.suggestions.is_empty() {
            self.selected_suggestion = None;
            return;
        }

        self.selected_suggestion = Some(match self.selected_suggestion {
            Some(index) => index.saturating_sub(1),
            None => self.suggestions.len() - 1,
        });
    }

    pub fn handle_down(&mut self) {
        if self.suggestions.is_empty() {
            self.selected_suggestion = None;
            return;
        }

        let last_index = self.suggestions.len() - 1;
        self.selected_suggestion = Some(match self.selected_suggestion {
            Some(index) => (index + 1).min(last_index),
            None => 0,
        });
    }

    pub fn accept(&mut self) -> String {
        let accepted = self
            .selected_suggestion
            .and_then(|index| self.suggestions.get(index).cloned())
            .unwrap_or_else(|| self.value.clone());
        self.clear();
        accepted
    }

    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor = 0;
        self.suggestions.clear();
        self.selected_suggestion = None;
    }

    pub fn set_suggestions(&mut self, suggestions: Vec<String>) {
        self.suggestions = suggestions;
        self.selected_suggestion = match (self.suggestions.is_empty(), self.selected_suggestion) {
            (true, _) => None,
            (false, Some(index)) => Some(index.min(self.suggestions.len() - 1)),
            (false, None) => None,
        };
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let input_height = area.height.min(3);
        if input_height == 0 || area.width == 0 {
            return;
        }

        let layout =
            Layout::vertical([Constraint::Length(input_height), Constraint::Min(0)]).split(area);
        let input_area = layout[0];

        let input = Paragraph::new(self.value.as_str())
            .block(Block::default().borders(Borders::ALL).title("Input"));
        frame.render_widget(input, input_area);

        let inner_width = input_area.width.saturating_sub(2) as usize;
        let visible_cursor = self.cursor.min(inner_width);
        let cursor_x = if input_area.width > 2 {
            input_area.x + 1 + visible_cursor as u16
        } else {
            input_area.x
        };
        let cursor_y = if input_area.height > 2 {
            input_area.y + 1
        } else {
            input_area.y
        };
        frame.set_cursor_position((cursor_x, cursor_y));

        if self.suggestions.is_empty() || layout[1].height == 0 {
            return;
        }

        let popup_height = (self.suggestions.len() as u16 + 2).min(layout[1].height);
        if popup_height == 0 {
            return;
        }

        let popup_area = Rect {
            x: area.x,
            y: input_area.y + input_area.height,
            width: area.width,
            height: popup_height,
        };

        let items = self
            .suggestions
            .iter()
            .enumerate()
            .map(|(index, suggestion)| {
                let style = if Some(index) == self.selected_suggestion {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };

                ListItem::new(suggestion.as_str()).style(style)
            });

        let popup =
            List::new(items).block(Block::default().borders(Borders::ALL).title("Suggestions"));

        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup, popup_area);
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent};

    use super::InputWidget;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn editing_updates_value_cursor_and_resets_selection() {
        let mut input = InputWidget::new();
        input.selected_suggestion = Some(1);

        input.handle_char('a');
        input.handle_char('ß');
        input.handle_backspace();

        assert_eq!(input.value, "a");
        assert_eq!(input.cursor, 1);
        assert_eq!(input.selected_suggestion, None);
    }

    #[test]
    fn handle_key_routes_supported_keys() {
        let mut input = InputWidget::new();
        input.set_suggestions(vec!["alpha".into(), "beta".into()]);

        input.handle_key(KeyEvent::from(KeyCode::Down));
        input.handle_key(KeyEvent::from(KeyCode::Char('ß')));
        input.handle_key(KeyEvent::from(KeyCode::Backspace));

        assert_eq!(input.selected_suggestion, None);
        assert!(input.value.is_empty());
        assert_eq!(input.cursor, 0);
    }

    #[test]
    fn suggestion_navigation_clamps_within_bounds() {
        let mut input = InputWidget::new();
        input.set_suggestions(vec!["alpha".into(), "beta".into(), "gamma".into()]);

        input.handle_down();
        assert_eq!(input.selected_suggestion, Some(0));

        input.handle_down();
        assert_eq!(input.selected_suggestion, Some(1));

        input.handle_up();
        assert_eq!(input.selected_suggestion, Some(0));

        input.handle_up();
        assert_eq!(input.selected_suggestion, Some(0));
    }

    #[test]
    fn accept_returns_selected_suggestion_and_clears_state() {
        let mut input = InputWidget::new();
        input.value = "alp".into();
        input.cursor = 3;
        input.set_suggestions(vec!["alpha".into(), "alpine".into()]);
        input.selected_suggestion = Some(1);

        let accepted = input.accept();

        assert_eq!(accepted, "alpine");
        assert_eq!(input, InputWidget::new());
    }

    #[test]
    fn accept_returns_current_value_when_no_suggestion_is_selected() {
        let mut input = InputWidget::new();
        input.value = "plain".into();
        input.cursor = 5;
        input.set_suggestions(vec!["planet".into()]);

        let accepted = input.accept();

        assert_eq!(accepted, "plain");
        assert_eq!(input, InputWidget::new());
    }

    #[test]
    fn set_suggestions_clears_selection_when_list_becomes_empty() {
        let mut input = InputWidget::new();
        input.set_suggestions(vec!["alpha".into()]);
        input.selected_suggestion = Some(0);

        input.set_suggestions(Vec::new());

        assert!(input.suggestions.is_empty());
        assert_eq!(input.selected_suggestion, None);
    }

    #[test]
    fn render_shows_input_and_suggestions() {
        let backend = TestBackend::new(30, 8);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut input = InputWidget::new();
        input.value = "alp".into();
        input.cursor = 3;
        input.set_suggestions(vec!["alpha".into(), "alpine".into()]);
        input.selected_suggestion = Some(1);

        terminal
            .draw(|frame| input.render(frame, frame.area()))
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

        assert!(rendered.contains("Input"));
        assert!(rendered.contains("alp"));
        assert!(rendered.contains("Suggestions"));
        assert!(rendered.contains("alpha"));
        assert!(rendered.contains("alpine"));
    }
}

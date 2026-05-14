use crossterm::event::KeyEvent;
use ratatui::prelude::{Frame, Rect};

use crate::widgets::input::InputWidget;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PersonWidget {
    pub input: InputWidget,
    pub selected: Option<i64>,
    suggestions: Vec<(i64, String)>,
}

impl PersonWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        self.input.handle_key(key);
        self.sync_selected();
    }

    pub fn set_suggestions(&mut self, suggestions: Vec<(i64, String)>) {
        self.suggestions = suggestions;
        self.input.set_suggestions(
            self.suggestions
                .iter()
                .map(|(_, name)| name.clone())
                .collect(),
        );
        self.sync_selected();
    }

    pub fn selected_id(&self) -> Option<i64> {
        self.selected
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.input.render(frame, area);
    }

    fn sync_selected(&mut self) {
        self.selected = self
            .input
            .selected_suggestion
            .and_then(|index| self.suggestions.get(index).map(|(id, _)| *id));
    }
}

#[cfg(test)]
mod tests {
    use super::PersonWidget;
    use crossterm::event::{KeyCode, KeyEvent};
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn selected_id_follows_navigation() {
        let mut widget = PersonWidget::new();
        widget.set_suggestions(vec![(1, "Ada Lovelace".into()), (2, "Alan Turing".into())]);

        assert_eq!(widget.selected_id(), None);

        widget.handle_key(KeyEvent::from(KeyCode::Down));
        assert_eq!(widget.selected_id(), Some(1));

        widget.handle_key(KeyEvent::from(KeyCode::Down));
        assert_eq!(widget.selected_id(), Some(2));
    }

    #[test]
    fn handle_key_updates_embedded_input() {
        let mut widget = PersonWidget::new();

        widget.handle_key(KeyEvent::from(KeyCode::Char('a')));
        widget.handle_key(KeyEvent::from(KeyCode::Char('b')));

        assert_eq!(widget.input.value, "ab");
        assert_eq!(widget.input.cursor, 2);
    }

    #[test]
    fn render_shows_person_suggestions() {
        let backend = TestBackend::new(40, 8);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut widget = PersonWidget::new();
        widget.input.value = "ada".into();
        widget.input.cursor = 3;
        widget.set_suggestions(vec![(1, "Ada Lovelace".into())]);

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

        assert!(rendered.contains("Input"));
        assert!(rendered.contains("ada"));
        assert!(rendered.contains("Suggestions"));
        assert!(rendered.contains("Ada Lovelace"));
    }
}

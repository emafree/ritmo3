use crossterm::event::KeyEvent;
use ratatui::prelude::{Frame, Rect};

use crate::widgets::input::InputWidget;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LanguageWidget {
    pub input: InputWidget,
    pub selected: Option<i64>,
    suggestions: Vec<(i64, String)>,
}

impl LanguageWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        self.input.handle_key(key);
        self.sync_selected();
    }

    pub fn set_suggestions(&mut self, suggestions: Vec<(i64, String)>) {
        self.input.set_suggestions(
            suggestions
                .iter()
                .map(|(_, label)| label.clone())
                .collect::<Vec<_>>(),
        );
        self.suggestions = suggestions;
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
    use crossterm::event::{KeyCode, KeyEvent};
    use ratatui::{backend::TestBackend, Terminal};

    use super::LanguageWidget;

    #[test]
    fn new_starts_without_selection() {
        let widget = LanguageWidget::new();

        assert_eq!(widget.selected_id(), None);
        assert!(widget.input.value.is_empty());
    }

    #[test]
    fn handle_key_updates_selected_language_id() {
        let mut widget = LanguageWidget::new();
        widget.set_suggestions(vec![
            (10, "Italiano (ita, it)".into()),
            (20, "English (eng, en)".into()),
        ]);

        widget.handle_key(KeyEvent::from(KeyCode::Down));
        assert_eq!(widget.selected_id(), Some(10));

        widget.handle_key(KeyEvent::from(KeyCode::Down));
        assert_eq!(widget.selected_id(), Some(20));

        widget.handle_key(KeyEvent::from(KeyCode::Char('e')));
        assert_eq!(widget.selected_id(), None);
    }

    #[test]
    fn set_suggestions_keeps_selection_in_bounds() {
        let mut widget = LanguageWidget::new();
        widget.set_suggestions(vec![(10, "Italiano".into()), (20, "English".into())]);
        widget.handle_key(KeyEvent::from(KeyCode::Down));
        widget.handle_key(KeyEvent::from(KeyCode::Down));

        widget.set_suggestions(vec![(30, "Deutsch".into())]);

        assert_eq!(widget.input.selected_suggestion, Some(0));
        assert_eq!(widget.selected_id(), Some(30));
    }

    #[test]
    fn render_shows_input_and_language_suggestions() {
        let backend = TestBackend::new(36, 8);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut widget = LanguageWidget::new();
        widget.input.value = "it".into();
        widget.input.cursor = 2;
        widget.set_suggestions(vec![
            (10, "Italiano (ita, it)".into()),
            (20, "English (eng, en)".into()),
        ]);
        widget.handle_key(KeyEvent::from(KeyCode::Down));

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
        assert!(rendered.contains("it"));
        assert!(rendered.contains("Suggestions"));
        assert!(rendered.contains("Italiano"));
        assert!(rendered.contains("English"));
    }
}

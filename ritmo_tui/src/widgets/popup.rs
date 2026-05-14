use ratatui::{
    layout::Alignment,
    prelude::{Frame, Rect},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

const POPUP_HINT: &str = "Enter: confirm / Esc: cancel";
const POPUP_HORIZONTAL_PADDING: u16 = 4;
const POPUP_HINT_LINES: u16 = 1;
const POPUP_BORDER_HEIGHT: u16 = 2;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PopupWidget {
    pub message: String,
    pub visible: bool,
}

impl PopupWidget {
    pub fn new(message: String) -> Self {
        Self {
            message,
            visible: false,
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if !self.visible || area.width == 0 || area.height == 0 {
            return;
        }

        let popup_area = centered_popup_area(area, &self.message);
        let popup = Paragraph::new(format!("{}\n{}", self.message, POPUP_HINT))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup, popup_area);
    }
}

fn centered_popup_area(area: Rect, message: &str) -> Rect {
    let desired_width = (max_line_width(message).max(POPUP_HINT.chars().count()) as u16)
        .saturating_add(POPUP_HORIZONTAL_PADDING);
    let width = desired_width.min(max_popup_width(area)).max(3);
    let inner_width = width.saturating_sub(2).max(1);

    let desired_height = wrapped_line_count(message, inner_width)
        .saturating_add(POPUP_HINT_LINES)
        .saturating_add(POPUP_BORDER_HEIGHT);
    let height = desired_height.min(max_popup_height(area)).max(3);

    Rect::new(
        area.x + area.width.saturating_sub(width) / 2,
        area.y + area.height.saturating_sub(height) / 2,
        width,
        height,
    )
}

fn max_popup_width(area: Rect) -> u16 {
    if area.width > 2 {
        area.width - 2
    } else {
        area.width.max(1)
    }
}

fn max_popup_height(area: Rect) -> u16 {
    if area.height > 2 {
        area.height - 2
    } else {
        area.height.max(1)
    }
}

fn max_line_width(text: &str) -> usize {
    text.lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
}

fn wrapped_line_count(text: &str, width: u16) -> u16 {
    if width == 0 {
        return 1;
    }

    let line_count: u16 = text
        .lines()
        .map(|line| {
            let line_width = line.chars().count().max(1);
            line_width.div_ceil(width as usize) as u16
        })
        .sum();

    line_count.max(1)
}

#[cfg(test)]
mod tests {
    use super::PopupWidget;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn new_initializes_hidden_popup() {
        let popup = PopupWidget::new("Delete book?".to_string());

        assert_eq!(popup.message, "Delete book?");
        assert!(!popup.visible);
    }

    #[test]
    fn show_and_hide_toggle_visibility() {
        let mut popup = PopupWidget::new("Delete book?".to_string());

        popup.show();
        assert!(popup.visible);

        popup.hide();
        assert!(!popup.visible);
    }

    #[test]
    fn hidden_popup_does_not_render() {
        let popup = PopupWidget::new("Delete book?".to_string());
        let backend = TestBackend::new(40, 7);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|frame| popup.render(frame, frame.area()))
            .unwrap();

        terminal
            .backend()
            .assert_buffer_lines(["                                        "; 7]);
    }

    #[test]
    fn visible_popup_renders_message_and_hint() {
        let mut popup = PopupWidget::new("Delete book?".to_string());
        let backend = TestBackend::new(40, 7);
        let mut terminal = Terminal::new(backend).unwrap();
        popup.show();

        terminal
            .draw(|frame| popup.render(frame, frame.area()))
            .unwrap();

        terminal.backend().assert_buffer_lines([
            "                                        ",
            "    ┌──────────────────────────────┐    ",
            "    │         Delete book?         │    ",
            "    │ Enter: confirm / Esc: cancel │    ",
            "    └──────────────────────────────┘    ",
            "                                        ",
            "                                        ",
        ]);
    }
}

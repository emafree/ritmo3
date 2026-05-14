use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    prelude::Frame,
    text::Line,
    widgets::{Paragraph, Widget},
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StatusBar {
    pub keys: Vec<String>,
    pub info: String,
    pub message: String,
}

impl StatusBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_keys(&mut self, keys: Vec<String>) {
        self.keys = keys;
    }

    pub fn set_info(&mut self, info: String) {
        self.info = info;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self, area);
    }

    fn keys_text(&self) -> String {
        self.keys.join("  ")
    }
}

impl Widget for &StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let sections = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(area);

        Paragraph::new(Line::from(self.keys_text()))
            .alignment(Alignment::Left)
            .render(sections[0], buf);
        Paragraph::new(Line::from(self.info.as_str()))
            .alignment(Alignment::Center)
            .render(sections[1], buf);
        Paragraph::new(Line::from(self.message.as_str()))
            .alignment(Alignment::Right)
            .render(sections[2], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::StatusBar;
    use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

    fn buffer_line(buffer: &Buffer, width: u16, y: u16) -> String {
        let mut line = String::new();
        for x in 0..width {
            line.push_str(buffer[(x, y)].symbol());
        }
        line
    }

    #[test]
    fn setters_update_status_bar_state() {
        let mut status_bar = StatusBar::new();

        status_bar.set_keys(vec!["q Quit".to_string(), "Enter Open".to_string()]);
        status_bar.set_info("Book 3 of 47".to_string());
        status_bar.set_message("Saved".to_string());

        assert_eq!(status_bar.keys, vec!["q Quit", "Enter Open"]);
        assert_eq!(status_bar.info, "Book 3 of 47");
        assert_eq!(status_bar.message, "Saved");

        status_bar.clear_message();
        assert_eq!(status_bar.message, "");
    }

    #[test]
    fn render_places_keys_info_and_message_in_three_zones() {
        let status_bar = StatusBar {
            keys: vec!["q Quit".to_string(), "Enter Open".to_string()],
            info: "Book 3 of 47".to_string(),
            message: "Saved".to_string(),
        };
        let area = Rect::new(0, 0, 60, 1);
        let mut buffer = Buffer::empty(area);

        <&StatusBar as Widget>::render(&status_bar, area, &mut buffer);

        let line = buffer_line(&buffer, area.width, 0);
        assert_eq!(&line[0..20], "q Quit  Enter Open  ");
        assert_eq!(&line[20..40], "    Book 3 of 47    ");
        assert_eq!(&line[40..60], "               Saved");
    }
}

use ratatui::{
    prelude::{Constraint, Frame, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TableWidgetState {
    pub selected: usize,
    pub offset: usize,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TableWidget {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub state: TableWidgetState,
}

impl TableWidget {
    pub fn new(headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        let mut table = Self {
            headers,
            rows,
            state: TableWidgetState::default(),
        };
        table.clamp_state();
        table
    }

    pub fn next(&mut self, visible_rows: usize) {
        if self.rows.is_empty() {
            return;
        }

        let last_index = self.rows.len() - 1;
        self.state.selected = (self.state.selected + 1).min(last_index);

        if visible_rows == 0 {
            self.state.offset = self.state.selected;
            return;
        }

        if self.state.selected >= self.state.offset + visible_rows {
            self.state.offset = self.state.selected + 1 - visible_rows;
        }
    }

    pub fn previous(&mut self) {
        if self.rows.is_empty() {
            return;
        }

        self.state.selected = self.state.selected.saturating_sub(1);
        if self.state.selected < self.state.offset {
            self.state.offset = self.state.selected;
        }
    }

    pub fn selected_index(&self) -> usize {
        self.state.selected
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.clamp_state();

        let column_count = self
            .headers
            .len()
            .max(self.rows.iter().map(Vec::len).max().unwrap_or(0))
            .max(1);
        let widths = vec![Constraint::Min(1); column_count];

        let header = Row::new((0..column_count).map(|index| {
            Cell::from(
                self.headers
                    .get(index)
                    .map(String::as_str)
                    .unwrap_or_default(),
            )
        }))
        .style(Style::default().add_modifier(Modifier::BOLD));

        let visible_rows = usize::from(area.height.saturating_sub(3));
        let rows = self
            .rows
            .iter()
            .enumerate()
            .skip(self.state.offset)
            .take(visible_rows)
            .map(|(index, row)| {
                let cells = (0..column_count).map(|cell_index| {
                    Cell::from(row.get(cell_index).map(String::as_str).unwrap_or_default())
                });

                let mut rendered_row = Row::new(cells);
                if index == self.state.selected {
                    rendered_row =
                        rendered_row.style(Style::default().add_modifier(Modifier::REVERSED));
                }
                rendered_row
            });

        let table = Table::new(rows, widths)
            .header(header)
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(table, area);
    }

    fn clamp_state(&mut self) {
        if self.rows.is_empty() {
            self.state.selected = 0;
            self.state.offset = 0;
            return;
        }

        let last_index = self.rows.len() - 1;
        self.state.selected = self.state.selected.min(last_index);
        self.state.offset = self.state.offset.min(self.state.selected);
    }
}

#[cfg(test)]
mod tests {
    use super::{TableWidget, TableWidgetState};

    #[test]
    fn new_initializes_state() {
        let table = TableWidget::new(vec!["Title".to_string()], vec![vec!["Dune".to_string()]]);

        assert_eq!(table.headers, vec!["Title"]);
        assert_eq!(table.rows, vec![vec!["Dune"]]);
        assert_eq!(table.state, TableWidgetState::default());
    }

    #[test]
    fn next_moves_selection_and_updates_offset() {
        let mut table = TableWidget::new(
            vec!["Title".to_string()],
            vec![
                vec!["A".to_string()],
                vec!["B".to_string()],
                vec!["C".to_string()],
            ],
        );

        table.next(2);
        assert_eq!(table.selected_index(), 1);
        assert_eq!(table.state.offset, 0);

        table.next(2);
        assert_eq!(table.selected_index(), 2);
        assert_eq!(table.state.offset, 1);
    }

    #[test]
    fn previous_moves_selection_and_updates_offset() {
        let mut table = TableWidget::new(
            vec!["Title".to_string()],
            vec![
                vec!["A".to_string()],
                vec!["B".to_string()],
                vec!["C".to_string()],
            ],
        );
        table.state.selected = 2;
        table.state.offset = 1;

        table.previous();
        assert_eq!(table.selected_index(), 1);
        assert_eq!(table.state.offset, 1);

        table.previous();
        assert_eq!(table.selected_index(), 0);
        assert_eq!(table.state.offset, 0);
    }
}

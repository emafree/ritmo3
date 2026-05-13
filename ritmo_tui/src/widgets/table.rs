#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TableWidget {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::TableWidget;

    #[test]
    fn default_table_widget_is_empty() {
        let table = TableWidget::default();
        assert!(table.headers.is_empty());
        assert!(table.rows.is_empty());
    }

    #[test]
    fn table_widget_stores_headers_and_rows() {
        let table = TableWidget {
            headers: vec!["Title".to_string(), "Author".to_string()],
            rows: vec![vec!["Dune".to_string(), "Frank Herbert".to_string()]],
        };

        assert_eq!(table.headers, vec!["Title", "Author"]);
        assert_eq!(table.rows, vec![vec!["Dune", "Frank Herbert"]]);
    }
}

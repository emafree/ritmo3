use ritmo_errors::reporter::RitmoReporter;

pub struct CliReporter;

impl RitmoReporter for CliReporter {
    fn status(&mut self, message: &str) {
        println!("{message}");
    }

    fn progress(&mut self, message: &str) {
        println!("{message}");
    }

    fn error(&mut self, message: &str) {
        eprintln!("{message}");
    }
}

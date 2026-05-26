mod import_books;
mod import_people;
mod model;
mod reporter;

use std::env;
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use dotenv::dotenv;
use ritmo_errors::reporter::RitmoReporter;
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::RepositoryContext;

use crate::import_books::import_books_file;
use crate::import_people::import_people_file;
use crate::model::{BookFile, PersonFile};
use crate::reporter::CliReporter;

#[derive(Parser)]
#[command(name = "ritmo_import", about = "Batch importer from TOML files")]
struct Cli {
    /// One or more people TOML files to import
    #[arg(long = "people", value_name = "FILE", num_args = 1..)]
    people: Vec<PathBuf>,

    /// One or more books TOML files to import
    #[arg(long = "books", value_name = "FILE", num_args = 1..)]
    books: Vec<PathBuf>,
}

#[tokio::main]
async fn main() -> RitmoResult<()> {
    dotenv().ok();

    let cli = Cli::parse();

    if cli.people.is_empty() && cli.books.is_empty() {
        eprintln!("No files specified. Use --people and/or --books.");
        std::process::exit(1);
    }

    let database_url = env::var("DATABASE_URL").map_err(|_| RitmoErr::ConfigNotFound)?;
    let pool = ritmo_db::create_sqlite_pool(&database_url).await?;
    let repo_ctx = RepositoryContext::new(pool);

    let mut reporter = CliReporter;

    // Process all people files first, then all books files
    for path in &cli.people {
        let file_name = path.display().to_string();
        reporter.status(&format!("── {file_name} ──"));

        let content = fs::read_to_string(path).map_err(|e| RitmoErr::FileAccess(e))?;
        let person_file: PersonFile = toml::from_str(&content).map_err(|e| {
            RitmoErr::ConfigParseError(format!("TOML parse error in {file_name}: {e}"))
        })?;

        import_people_file(&repo_ctx, &person_file, &mut reporter).await?;
    }

    for path in &cli.books {
        let file_name = path.display().to_string();
        reporter.status(&format!("── {file_name} ──"));

        let content = fs::read_to_string(path).map_err(|e| RitmoErr::FileAccess(e))?;
        let book_file: BookFile = toml::from_str(&content).map_err(|e| {
            RitmoErr::ConfigParseError(format!("TOML parse error in {file_name}: {e}"))
        })?;

        import_books_file(&repo_ctx, &book_file, &mut reporter).await?;
    }

    reporter.status("Importazione completata.");
    Ok(())
}

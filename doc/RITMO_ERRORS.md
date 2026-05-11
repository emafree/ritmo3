## `ritmo_errors`

### Responsabilità

Centralizza tutti i tipi di errore del progetto e fornisce un'astrazione per il reporting verso qualunque interfaccia utente.

### Contenuto

#### `RitmoErr`

Enum che definisce tutti i tipi di errore del progetto, organizzati per dominio:

-   **Database:** `DatabaseConnection`, `DatabaseQuery`, `DatabaseMigration`, `DatabaseInsert`, `DatabaseDelete`, `DatabaseTransaction`, `RecordNotFound`, `DataIntegrity`
-   **File system:** `FileNotFound`, `FileAccess`, `PathError`
-   **Import/Export:** `ImportError`, `ExportError`
-   **Configurazione:** `ConfigNotFound`, `ConfigParseError`
-   **Dominio:** `InvalidInput`, `NameParsingError`, `MergeError`
-   **ML/Serializzazione:** `SerializationError`, `MLError`
-   **Generico:** `UnknownError`

#### `RitmoResult<T>`

Alias di tipo `Result<T, RitmoErr>` usato in tutto il progetto.

#### `RitmoReporter`

Trait per il reporting di stato, progresso ed errori verso qualunque interfaccia utente. Disaccoppia il codice di libreria dall'output concreto.

Metodi: `status`, `progress`, `error`.

Implementazioni previste:

-   `SilentReporter` — no-op, per librerie e test (incluso in questo crate)
-   `CliReporter` — output su stdout/stderr (implementato in `ritmo_cli`)
-   `GuiReporter` — aggiornamento componenti GUI (implementato nell'interfaccia GUI futura)

### Dipendenze esterne

-   `thiserror` — derivazione macro per `RitmoErr`
-   `sqlx` — conversione `From<sqlx::Error>`
-   `serde_json` — conversione `From<serde_json::Error>`
-   `toml` — conversione `From<toml::de::Error>` e `From<toml::ser::Error>`

### Dipendenze interne

Nessuna. Questo crate non dipende da nessun altro crate del progetto.

### Regole

-   Nuovi tipi di errore si aggiungono qui e solo qui.
-   Nessun altro crate definisce tipi di errore propri.
-   `RitmoReporter` non viene implementato in questo crate salvo `SilentReporter`.

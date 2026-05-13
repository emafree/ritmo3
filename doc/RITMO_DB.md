## `ritmo_db`

### Responsabilità

Contiene lo schema SQLite, le migrations, il seeding iniziale e la connessione al database. Non definisce struct di dominio, non contiene logica applicativa, non esegue operazioni CRUD.

### Contenuto

#### Schema (`schema.sql`)

Definisce tutte le tabelle, gli indici e i trigger del database.

**Tabelle di dominio (`d_`):** `d_books`, `d_contents`, `d_people`, `d_publishers`, `d_series`, `d_formats`, `d_genres`, `d_types`, `d_roles`, `d_tags`, `d_aliases`, `d_languages`, `d_places`

**Tabelle di relazione (`x_`):** `x_books_contents`, `x_books_people_roles`, `x_books_tags`, `x_contents_people_roles`, `x_contents_tags`, `x_content_languages`, `x_book_languages`, `x_person_languages`, `x_person_places`, `x_publisher_places`

**Tabelle di sistema (`s_`):** `s_system_config`, `s_audit_log`, `s_stats_cache`, `s_pending_metadata_sync`, `s_metadata`, `s_ml_data`, `s_page_fields`, `s_place_types`, `s_place_type_translations`, `s_person_language_roles`, `s_person_language_role_translations`, `s_content_language_roles`, `s_content_language_role_translations`, `s_book_language_roles`, `s_book_language_role_translations`, `s_role_translations`, `s_format_translations`, `s_genre_translations`, `s_type_translations`, `s_filter_sets`, `s_filter_conditions`.

**Trigger:** Normalizzazione nomi, aggiornamento timestamp, audit log su `d_people`, cleanup automatico di `s_audit_log` e `s_stats_cache`.

#### Seeding (`seed_lookups.sql`, `seed_page_fields.sql`)

Dati iniziali obbligatori per il funzionamento del sistema. Eseguiti dopo la creazione dello schema. Idempotenti — usano DELETE + INSERT per garantire stato deterministico.

#### Connessione

Gestisce la creazione del pool SQLite e l'esecuzione delle migrations e seeding all'avvio.

### File SQL

-   `schema.sql` — schema completo, tabelle, indici, trigger
-   `seed_lookups.sql` — dati iniziali per le tabelle di lookup
-   `s_page_fields.sql` — definizione dei campi per le pagine di editing

### Dipendenze esterne

-   `sqlx` — connessione e pool SQLite
-   `tokio` — runtime async
-   `ritmo_errors` — errori di connessione e migration

### Dipendenze interne

-   `ritmo_errors`

### Regole

-   Lo schema SQL non si modifica senza aggiornare questo documento.
-   Nessuna struct di dominio viene definita in questo crate.
-   Nessuna logica applicativa, nessun CRUD.
-   Le migrations sono l'unico meccanismo per modificare lo schema in produzione.
-   I file di seeding sono idempotenti — devono poter essere rieseguiti senza effetti collaterali.
-   Tutte le tabelle rispettano la convenzione di prefisso definita in `ARCHITECTURE.md`.

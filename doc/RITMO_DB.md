## `ritmo_db`

### Responsabilità

Contiene lo schema SQLite, le migrations, il seeding iniziale e la connessione al database. Non definisce struct di dominio, non contiene logica applicativa, non esegue operazioni CRUD.

### Contenuto

#### Schema (`schema.sql`)

Definisce tutte le tabelle, gli indici e i trigger del database.

**Tabelle di dominio (`d_`):** `d_books`, `d_contents`, `d_people`, `d_publishers`, `d_series`, `d_formats`, `d_types`, `d_roles`, `d_tags`, `d_aliases`, `d_languages`, `d_places`

**Tabelle di relazione (`x_`):** `x_books_contents`, `x_books_people_roles`, `x_books_tags`, `x_contents_people_roles`, `x_contents_tags`, `x_content_languages`, `x_book_languages`, `x_person_languages`, `x_person_places`, `x_publisher_places`

**Tabelle di sistema (`s_`):** `s_system_config`, `s_audit_log`, `s_stats_cache`, `s_pending_metadata_sync`, `s_metadata`, `s_ml_data`, `s_page_fields`, `s_filter_sets`, `s_filter_conditions`, `s_place_types`, `s_place_type_translations`, `s_person_language_roles`, `s_person_language_role_translations`, `s_content_language_roles`, `s_content_language_role_translations`, `s_book_language_roles`, `s_book_language_role_translations`, `s_role_translations`, `s_format_translations`, `s_type_translations`

**Trigger:** Normalizzazione nomi, aggiornamento timestamp, audit log su `d_people`, cleanup automatico di `s_audit_log` e `s_stats_cache`.

#### Seeding (`seed_lookups.sql`, `seed_page_fields.sql`)

Dati iniziali obbligatori per il funzionamento del sistema. Eseguiti dopo la creazione dello schema. Idempotenti — usano `INSERT OR IGNORE`.

Il seeding popola esclusivamente le lookup di sistema: tabelle con chiave canonica e traduzioni i18n associate (`d_roles`, `d_types`, `d_formats`, `d_languages`, `s_person_language_roles`, `s_place_types`, `s_content_language_roles`, `s_book_language_roles`).

Le tabelle di dominio gestite dall'utente (`d_tags`, `d_publishers`, `d_series`, ecc.) non vengono pre-popolate — partono vuote.

#### Connessione

Gestisce la creazione del pool SQLite e l'esecuzione delle migrations e seeding all'avvio.

### Modello dei dati — decisioni rilevanti

#### `d_contents` è l'entità principale

`d_contents` rappresenta un'opera letteraria nella sua forma testuale specifica, inclusa la lingua. La stessa opera in lingue diverse è un `content` distinto. `d_books` è il contenitore fisico o digitale — un'edizione. La relazione è molti-a-molti tramite `x_books_contents`.

La distinzione ha conseguenze sul posizionamento delle relazioni con le persone:
- Autore, traduttore, prefatore, commentatore, illustratore del testo → `x_contents_people_roles`
- Cover artist, consulente editoriale, fotografo → `x_books_people_roles`

Questa distinzione è una convenzione d'uso, non un vincolo di schema. `d_roles` è editabile dall'utente e non ha attributi tecnici di scope.

#### `d_tags` — tag liberi tipizzati, senza seeding

`d_tags` gestisce etichette libere associabili a libri e contenuti. La colonna `tag_type` distingue la funzione semantica:

- `genre` — genere o sottogenere letterario (es. fantascienza, noir, ucronia)
- `mood` — atmosfera o tono (es. cupo, ironico, epico)
- `setting` — ambientazione (es. spazio, medioevo, futuro prossimo)
- `personal` — annotazioni personali (es. da rileggere, prestato)

I tag di tipo `genre` sostituiscono la precedente tabella `d_genres`, eliminata. La scelta è motivata da due ragioni: i generi letterari reali non si classificano in una lista chiusa (un romanzo può essere thriller, satira sociale e dark humor contemporaneamente), e una lookup controllata con i18n non è gestibile dall'utente senza attrito eccessivo.

`d_tags` non viene pre-popolata dal seeding — l'utente inserisce i tag liberamente durante l'uso. Il sistema ML normalizza e suggerisce tag nel tempo. `tag_type` aiuta il ML a distinguere segnali semantici diversi.

#### Date bibliografiche

Le date usano quattro colonne nullable per campo (`_year`, `_month`, `_day`, `_circa`), distinte dai campi tecnici Unix timestamp (`created_at`, `updated_at`). Su `d_contents` la data è quella dell'opera; su `d_books` è quella dell'edizione.

#### Ricerca full-text

Non ancora implementata. Da aggiungere tramite virtual table FTS5 su titoli, note e nomi delle persone collegate. SQLite FTS5 è disponibile senza dipendenze aggiuntive.

### File SQL

-   `schema.sql` — schema completo, tabelle, indici, trigger
-   `seed_lookups.sql` — dati iniziali per le tabelle di lookup di sistema
-   `seed_page_fields.sql` — definizione dei campi per le pagine di editing

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
-   Il seeding non pre-popola tabelle di dominio gestite dall'utente.
-   Tutte le tabelle rispettano la convenzione di prefisso definita in `ARCHITECTURE.md`.

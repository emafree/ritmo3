# Ritmo3 — Contesto del progetto

## Cos'è

Applicazione Rust per la gestione di una biblioteca personale di ~12.000 EPUB.
Strutturata come workspace Cargo multi-crate. Interfaccia testuale (TUI) con Ratatui.
Il database viene caricato integralmente in memoria all'avvio — nessuna query lazy durante la navigazione.

---

## Crate e responsabilità

| Crate | Stato | Responsabilità |
|---|---|---|
| `ritmo_errors` | ✅ completo | Tipi di errore centralizzati, `RitmoResult<T>`, trait `RitmoReporter` |
| `ritmo_domain` | ✅ completo | Struct di dominio: `Book`, `Content`, `Person`, `Publisher`, `Series`, `Format`, `Genre`, `Role`, `Tag`, `Language`, `Alias`, `Place`, `PlaceType`, `PartialDate`. Strutture di filtro: `Filter`, `FilterSet`, `FilterField`, `FilterOperator`, `FilterValue`, `LogicalOperator` |
| `ritmo_db` | ✅ completo — schema SQL aggiornato | Schema SQLite, seeding, connessione e pool. File SQL in `ritmo_db/schema/`. Tutti i `CREATE TABLE` e `CREATE TRIGGER` hanno `IF NOT EXISTS`. Il seeding usa `INSERT OR IGNORE` |
| `ritmo_repository` | ⚠️ da aggiornare | Operazioni CRUD per tutte le entità (un file per entità). Filtro dinamico in `filter_books.rs`, `filter_contents.rs`, `filter_sets.rs`. **I nomi delle tabelle nelle query SQL hardcodate devono essere allineati ai nuovi nomi (vedi sezione schema SQL).** |
| `ritmo_core` | ✅ completo | Logica applicativa, policy di delete, gestione relazioni. Non ancora pienamente collegato alla TUI |
| `ritmo_presenter` | ✅ completo | View model per tutte le entità, trait `I18nDisplayable`. I18n per `Format`, `Genre`, `Role` tramite `rust-i18n` |
| `ritmo_tui` | 🔄 in sviluppo | Interfaccia TUI con Ratatui (vedi dettaglio sotto) |
| `ritmo_app` | ✅ funzionante | Punto di ingresso. Carica `.env`, inizializza il database, lancia la TUI |

---

## Sessione del 16 maggio 2026 — operazioni eseguite

### Revisione e correzione dello schema SQL (`schema.sql`)

Identificati e corretti due errori bloccanti (FK su tabelle inesistenti) e 30 violazioni della convenzione di prefisso definita in `ARCHITECTURE.md`.

**Errori bloccanti corretti:**
- `x_person_places`: FK su `d_people` — la tabella si chiamava `people` (senza prefisso)
- `x_publisher_places`: FK su `d_publishers` — la tabella si chiamava `publishers` (senza prefisso)

**Convenzione di prefisso applicata a tutte le tabelle** (erano prive di prefisso):

| Categoria | Tabelle rinominate |
|---|---|
| `d_` (dominio) | `formats→d_formats`, `publishers→d_publishers`, `series→d_series`, `roles→d_roles`, `tags→d_tags`, `genres→d_genres`, `types→d_types`, `languages→d_languages`, `people→d_people`, `aliases→d_aliases`, `books→d_books`, `contents→d_contents` |
| `s_` (sistema) | `system_config→s_system_config`, `audit_log→s_audit_log`, `stats_cache→s_stats_cache`, `ml_data→s_ml_data`, `metadata→s_metadata`, `pending_metadata_sync→s_pending_metadata_sync`, `page_fields→s_page_fields`, `role_translations→s_role_translations`, `format_translations→s_format_translations`, `genre_translations→s_genre_translations`, `type_translations→s_type_translations`, `person_language_roles→s_person_language_roles`, `person_language_role_translations→s_person_language_role_translations`, `content_language_roles→s_content_language_roles`, `content_language_role_translations→s_content_language_role_translations`, `book_language_roles→s_book_language_roles`, `book_language_role_translations→s_book_language_role_translations` |
| `x_` (relazione) | `person_languages→x_person_languages`, `content_languages→x_content_languages`, `book_languages→x_book_languages` |

Tutti i trigger e gli indici aggiornati di conseguenza. Schema validato con SQLite in memoria — zero errori.

### Correzione dei file di seeding

`seed_lookups.sql` e `seed_page_fields.sql` allineati ai nuovi nomi di tabella:

- Tutti i nomi di tabella negli `INSERT INTO` aggiornati
- Tutte le subquery `SELECT id FROM <tabella>` nei valori FK aggiornate
- `seed_page_fields.sql`: `DELETE FROM page_fields` → `DELETE FROM s_page_fields`; `target_table = 'publishers'` → `'d_publishers'`

Validati insieme allo schema su database in memoria — zero errori, tutti i conteggi corretti.

### File prodotti (pronti per il repository)

- `schema.sql` — schema completo corretto
- `seed_lookups.sql` — seed lookup corretto
- `seed_page_fields.sql` — seed page fields corretto

---

## Passi successivi

### 0. Aggiornare `ritmo_repository` ← PRIORITÀ IMMEDIATA
Tutti i nomi di tabella nelle query SQL hardcodate in `ritmo_repository` devono essere allineati ai nuovi nomi. Questo riguarda ogni file del crate che contiene stringhe SQL (`books.rs`, `contents.rs`, `people.rs`, `filter_books.rs`, `filter_contents.rs`, `filter_sets.rs`, ecc.).

### 1. Rifinire `ContentCreateScreen`
Ci sono problemi minori di comportamento emersi durante i test manuali, da analizzare e correggere.

### 2. Collegare `Submit` a `ritmo_core`
`Ctrl+S` in `ContentCreateScreen` restituisce `ContentCreateAction::Submit`, che in `app.rs` chiude la schermata senza salvare. Va collegato a `ritmo_core::content::create`, con reload della lista dopo il salvataggio.

### 3. Sviluppare `BookCreateScreen`
Non ancora implementata. Va creata seguendo il modello di `ContentCreateScreen`, poi collegata alla navigazione.

### 4. Collegare `PersonCreateScreen`
Il file esiste (`people/create.rs`) ma non è ancora collegato alla navigazione in `app.rs`.

### 5. Implementare Filters
Mostrare lista `FilterSet` salvati, spunta attivo/non attivo, filtro zero fisso in cima.

### 6. Schermate mancanti
- `contents/detail.rs` — non ancora implementata
- `people/list.rs` e `people/detail.rs` — accessibili solo dal dettaglio libro/contenuto
- Gestione entità indipendenti (tag, publisher, series, format, genre, role, language) — accessibili dal dettaglio

### 7. Sezioni relazionate nel dettaglio
`BookDetailScreen` ha sezioni relazionate (persone, tag, lingue, contenuti) navigabili ma non ancora collegate a popup per aggiungere/rimuovere. Va definita la gestione dei layer sovrapposti in `app.rs`.

### 8. Sistema ML
Esiste in ritmo2 ed è funzionante. Va importato quando si inizia a inserire dati reali — serve per la normalizzazione dei nomi, dei luoghi, delle tags, etc.

### 9. Internazionalizzazione della TUI
Tutte le label in `ritmo_tui` sono attualmente hardwired in italiano. In futuro gestite tramite `rust-i18n`, come già fatto per `Format`, `Genre`, `Role` in `ritmo_presenter`. Da fare quando la TUI è stabile.

---

## Stato attuale della TUI

Il programma compila e funziona. Le tre finestre principali si aprono e la navigazione tra esse funziona correttamente.

### Widget implementati

- `TableWidget`
- `InputWidget`
- `PopupWidget`
- `StatusBar`
- `PartialDateWidget`
- `PlaceWidget`
- `LanguageWidget` — ricerca con autocomplete
- `PersonWidget` — ricerca con autocomplete

### Schermate implementate

- `BookListScreen`
- `ContentListScreen`
- `BookDetailScreen`
- `ContentCreateScreen` — funzionante, con problemi minori da rifinire

### Navigazione TUI

**Livello 0 — Finestre principali**

Tre finestre principali: **Books**, **Contents**, **Filters**.
Ognuna mostra la lista dei relativi item, navigabile con frecce verticali o `j`/`k`.
Cambio finestra: frecce orizzontali, oppure tasti `b` (Books), `c` (Contents), `f` (Filters).

### Stato per finestra

| Finestra | Stato |
|---|---|
| **Contents** | Navigazione funzionante. Premendo `n` si apre `ContentCreateScreen`. Input, navigazione tra campi e chiusura con `Esc` funzionano. Il salvataggio (`Ctrl+S`) è strutturato ma non ancora collegato a `ritmo_core`. Ci sono problemi minori di comportamento da rifinire. |
| **Books** | Allo stato della creazione iniziale delle tre finestre. Development partito da Contents, Books è più indietro. |
| **Filters** | Da implementare. Deve mostrare la lista dei `FilterSet` salvati con spunta attivo/non attivo. Il filtro zero (titolo + autore) sempre presente in cima. |

---

## Cose minori pendenti

- `.env.example` da aggiungere al repository
- Verificare che `.env` sia nel `.gitignore`
- `entities.rs` fantasma in `ritmo_repository/src/` — file non dichiarato, da eliminare

---

## Convenzioni di lavoro

- **Copilot** riceve istruzioni tramite issue su GitHub con assegnazione a `@copilot`. I commenti nelle PR funzionano per correzioni.
- **Branch**: lo crea Copilot — il nome sarà diverso da quello indicato nel prompt. Fare sempre `git fetch origin` prima del checkout.
- **Ciclo standard**: checkout → review file → build → unisci → pulizia locale.
- **Modifiche locali**: salvare con `git stash` prima di cambiare branch.
- **Documentazione**: i file in `/doc` sono vincolanti — qualsiasi modifica al dominio o all'architettura va riflessa lì.

---

## Architettura — principi chiave

- `ritmo_presenter` è il layer stabile tra core e tutti i frontend — nessun frontend dipende direttamente da `sqlx` o `ritmo_db`.
- Le date bibliografiche usano quattro colonne nullable (`date_year`, `date_month`, `date_day`, `date_circa`), distinte dai campi tecnici Unix timestamp.
- Distribuzione portabile: cartella self-contained con launcher + eseguibile + `data/`.
- I18n al layer presenter con fallback chain.
- Tabella `s_page_fields` sostituisce le definizioni di campo hardcoded.
- Lookup table divise in system-defined (con i18n) e user-managed (senza).

---

## Convenzione nomi tabelle SQL (ARCHITECTURE.md)

Obbligatoria. Ogni tabella deve avere uno di questi prefissi:

- `d_` — tabelle di dominio: dati immessi dall'utente o da tool esterni
- `x_` — tabelle di relazione: legami molti-a-molti tra entità di dominio
- `s_` — tabelle di sistema: dati interni al funzionamento dell'applicazione

Nessuna tabella senza prefisso. Schema, seed e query in `ritmo_repository` devono essere sempre allineati.

# Ritmo3 — Contesto del progetto

## Cos'è

Applicazione Rust per la gestione di una biblioteca personale di ~12.000 EPUB.
Strutturata come workspace Cargo multi-crate. Interfaccia utente non ancora definita (vedi sotto).
Il database viene caricato integralmente in memoria all'avvio — nessuna query lazy durante la navigazione.

---

## Crate e responsabilità

| Crate | Stato | Responsabilità |
|---|---|---|
| `ritmo_errors` | ✅ completo | Tipi di errore centralizzati, `RitmoResult<T>`, trait `RitmoReporter` |
| `ritmo_domain` | ✅ allineato | Struct di dominio: `Book`, `Content`, `Person`, `Publisher`, `Series`, `Format`, `Role`, `Tag`, `Language`, `Alias`, `Place`, `PlaceType`, `PartialDate`. Strutture di filtro: `Filter`, `FilterSet`, `FilterField`, `FilterOperator`, `FilterValue`, `LogicalOperator`. La struct `Genre` è stata rimossa — i generi sono ora tag. |
| `ritmo_db` | ✅ allineato | Schema SQLite, seeding, connessione e pool. |
| `ritmo_repository` | ✅ allineato | Operazioni CRUD per tutte le entità. `genre.rs` rimosso. Query su `d_contents` e `d_tags` aggiornate. |
| `ritmo_core` | ✅ allineato | Logica applicativa, policy di delete, gestione relazioni. Casi d'uso Genre rimossi. Policy delete per lookup implementate. |
| `ritmo_presenter` | ✅ allineato | View model per tutte le entità, trait `I18nDisplayable`. Presenter Genre rimosso. |
| `ritmo_tui` | ❌ abbandonato | Interfaccia TUI con Ratatui — abbandonata. Il crate può essere rimosso dal workspace. |
| `ritmo_import` | ✅ funzionante | Importazione dati da EPUB e sorgenti esterne. Strumento di lavoro per popolare il database. |
| `ritmo_app` | ⚠️ da aggiornare | Punto di ingresso. Va aggiornato dopo la rimozione di `ritmo_tui`. |
| `ritmo_web` | ✅ attivo | Server web Axum + HTML. Liste, dettagli, form inserimento/modifica e cancellazione funzionanti per tutte le entità principali e di lookup. Widget place CRUD integrato in People e Publisher. |

---

## Interfaccia utente

`ritmo_tui` (Ratatui) è stata abbandonata. Il frontend attivo è `ritmo_web` — server Axum + HTML server-rendered nel browser.

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

`seed_lookups.sql` e `seed_page_fields.sql` allineati ai nuovi nomi di tabella.

---

## Sessione del 19 maggio 2026

### Verifica allineamento `ritmo_repository` ← schema.sql

Verificato con grep su tutti i file `.rs` di `ritmo_repository/src/`.
Nessuna query SQL usa nomi di tabella senza prefisso. Tutte le tabelle
`d_`, `s_`, `x_` sono allineate allo schema corrente, incluse le query
dinamiche in `filter_books.rs` e `filter_contents.rs`. Nessuna discrepanza.

---

## Sessione del 23 maggio 2026

### `Collection<T>` — centralizzazione del dispatch add/remove/delegate

Introdotta la struct generica `Collection<T>` in `ritmo_tui/src/widgets/collection.rs`.

**Nota:** il lavoro su `ritmo_tui` è successivamente diventato irrilevante per l'abbandono della TUI. Il crate va rimosso.

---

## Sessione del 26 maggio 2026

### Revisione del modello dati — decisioni strutturali

#### `d_contents` come entità principale

`d_contents` è l'entità centrale del database, non `d_books`. Un'opera letteraria esiste indipendentemente dalle sue edizioni fisiche. La stessa opera in lingue diverse è un `content` distinto — il traduttore appartiene al contenuto, non al libro.

`d_books` è il contenitore fisico o digitale (un'edizione). Le persone collegate a un libro sono quelle che hanno lavorato sull'oggetto fisico: cover artist, consulente editoriale, fotografo.

Questa distinzione è una convenzione d'uso documentata, non un vincolo di schema. `d_roles` è editabile dall'utente.

#### Eliminazione di `d_genres` — tag tipizzati

La tabella `d_genres` (lookup controllata con i18n) è stata eliminata. Motivazioni:

1. Una lista chiusa non copre la complessità reale — la maggior parte dei libri finiva in `other`.
2. I generi letterari non sono mutuamente esclusivi — un romanzo può essere thriller, satira sociale e dark humor contemporaneamente.
3. Una lookup controllata con traduzioni i18n non è gestibile dall'utente senza attrito eccessivo.

I generi diventano tag di tipo `genre` in `d_tags`. La colonna `tag_type` su `d_tags` distingue:
- `genre` — genere o sottogenere letterario
- `mood` — atmosfera o tono
- `setting` — ambientazione
- `personal` — annotazioni personali

`d_tags` parte vuota — nessun seeding. L'utente inserisce i tag durante l'uso. Il ML normalizza e suggerisce nel tempo, usando `tag_type` per distinguere segnali semantici diversi.

`genre_id` è stato rimosso da `d_contents`. Il genere si recupera tramite `x_contents_tags` filtrando per `tag_type = 'genre'`.

#### Ricerca per ricordo vago — strategia

Il campo `notes` su `d_contents` (già presente su `d_books`) è il punto di ingresso per metadati non strutturati inseriti al momento dell'immissione. La ricerca full-text tramite FTS5 su titoli, note e nomi delle persone è la soluzione prevista — da implementare.

#### Revisione del seeding di d_types
I valori di d_types non riflettevano bene lo schema che ho in mente. Devono appartenere esclusivamente a contents, per cui alcuni valori non erano piu necessari. Altri valori invece erano mancanti, altri duplicati. 
Lo schema finale di seeding ha 11 valori, sempre con le relative traduzioni. ✅ fatto


### File aggiornati in questa sessione

- `schema.sql` — rimossa `d_genres` e `s_genre_translations`; aggiunto `tag_type` a `d_tags`; rimosso `genre_id` da `d_contents`; aggiornati indici
- `seed_lookups.sql` — rimossa sezione `d_genres`; `d_tags` non viene pre-popolata
- `RITMO_DB.md` — aggiornato a riflettere tutte le decisioni sopra

### Crate da allineare allo schema

I seguenti crate fanno ancora riferimento a `Genre` e `genre_id` e vanno aggiornati:
- `ritmo_domain` — rimuovere struct `Genre` ✅ fatto
- `ritmo_repository` — rimuovere `genre.rs`, aggiornare query su `d_contents` e `d_tags` ✅ fatto
- `ritmo_core` — rimuovere casi d'uso Genre ✅ fatto
- `ritmo_presenter` — rimuovere view model Genre e i18n correlato ✅ fatto

---

## Sessione del 26 maggio 2026 — parte 2
### Creazione del crate ritmo_web
Creato il crate ritmo_web basato su Axum + HTML server-rendered. Aggiunto al workspace.
Pattern di inizializzazione adottato — identico a ritmo_import:

ritmo_db::create_sqlite_pool chiamato in main.rs (unico punto)
Il pool viene wrappato in RepositoryContext di ritmo_repository
AppState contiene RepositoryContext + AppConfig — nessun SqlitePool o sqlx esposto

### Struttura del crate:

src/main.rs — entrypoint, init DB, bind address da env, avvio server
src/state.rs — AppState con RepositoryContext e AppConfig
src/router.rs — route per books, contents, people, lookups
src/error.rs — mapping RitmoErr → HTTP response via IntoResponse
src/handlers/{books,contents,people,lookups}.rs — handler placeholder
src/templates/ — template HTML placeholder per tutte le entità

Dipendenze di ritmo_web: axum, tokio, dotenvy, serde_json, ritmo_db, ritmo_errors, ritmo_repository. Nessun sqlx diretto.
Passi successivi

Implementare gli handler reali collegando ritmo_core e ritmo_presenter
Definire i view model in ritmo_presenter per le liste (books, contents, people)
Costruire i template HTML reali a partire dai placeholder

---

## Sessione del 26 maggio 2026 — parte 3

### ritmo_web — primo handler reale + sistema di templating

#### Tera integrato come motore di template
Aggiunta dipendenza `tera = "1"` a `ritmo_web/Cargo.toml`.
I template vivono in `ritmo_web/templates/` (non in `src/templates/`).
`AppState` ora include un campo `tera: Tera` inizializzato in `main.rs` con glob `ritmo_web/templates/**/*.html`.
La working directory attesa è la root del workspace (`cargo run` da `~/Projects/ritmo3`).

#### ritmo_presenter — BookListItem serializzabile
Aggiunta dipendenza `serde` con feature `derive` a `ritmo_presenter/Cargo.toml`.
`BookListItem` deriva ora `Serialize` — necessario per inserirlo nel contesto Tera.
Aggiunta funzione `build_book_list_items` che costruisce `Vec<BookListItem>` dalle tuple restituite dal repository.

#### ritmo_repository — query JOIN per la lista libri
Aggiunto metodo `list_all_with_authors` a `BookRepository`.
Esegue un'unica query con LEFT JOIN su `x_books_people_roles`, `d_roles`, `d_people`, `d_formats`, `d_series`.
Usa `GROUP_CONCAT` con separatore `||` per aggregare gli autori per libro.
Restituisce `Vec<(i64, String, Vec<String>, Option<String>, Option<String>)>` — (id, titolo, autori, formato, serie).
La query filtra per `r.key = 'author'` tramite `CASE WHEN` dentro `GROUP_CONCAT`.

#### Handler books.rs — list implementato
`books::list` ora chiama `BookRepository::list_all_with_authors`, costruisce i view model con `build_book_list_items`, li passa al template Tera.
`books::detail` e `books::form` passano al template Tera ma restituiscono ancora pagine placeholder.

#### Template books
- `templates/base.html` — layout base con nav (Libri / Contenuti / Persone)
- `templates/books/list.html` — tabella con titolo, autori, formato, serie; messaggio se vuota
- `templates/books/detail.html` — placeholder
- `templates/books/form.html` — placeholder

#### Configurazione ambiente
Docker occupa la porta 3000. Aggiunto `.env` nella root del workspace con:
`DATABASE_URL=data/ritmo.db`
`WEB_BIND=127.0.0.1:3001`

### Stato attuale
- Lista libri: ✅ funzionante
- Dettaglio libro: ❌ placeholder
- Lista contenuti: ❌ placeholder
- Lista persone: ❌ placeholder
- Tutte le form: ❌ placeholder

### Prossimi passi
1. Lista contenuti — stesso pattern della lista libri
2. Lista persone
3. Pagine di dettaglio

---

## Sessione del 26 maggio 2026 — parte 4

### ritmo_web — liste contenuti e persone

Implementate e mergiate le pagine lista per le tre entità principali.
Tutte e tre funzionanti e visibili nel browser.

**File aggiunti/modificati:**
- `ritmo_repository/src/content.rs` — aggiunto `list_all_with_people`
- `ritmo_repository/src/person.rs` — aggiunto `list_all_for_display`
- `ritmo_presenter/src/content.rs` — `ContentListItem` + `build_content_list_items`
- `ritmo_presenter/src/person.rs` — `PersonListItem` + `build_person_list_items`
- `ritmo_web/src/handlers/contents.rs` — handler `list` implementato
- `ritmo_web/src/handlers/people.rs` — handler `list` implementato
- `ritmo_web/templates/contents/list.html` — template lista contenuti
- `ritmo_web/templates/people/list.html` — template lista persone

### Stato attuale delle pagine web

| Pagina | Stato |
|---|---|
| `GET /books` | ✅ funzionante |
| `GET /contents` | ✅ funzionante |
| `GET /people` | ✅ funzionante |
| `GET /books/:id` | ❌ placeholder |
| `GET /contents/:id` | ❌ placeholder |
| `GET /people/:id` | ❌ placeholder |
| Form di inserimento/modifica | ❌ placeholder |

### Note aperte
- La colonna `genre_id` su `d_contents` è ancora presente nello schema fisico
  (la migrazione di rimozione non è ancora stata applicata). Non ha effetti
  sul funzionamento attuale — da verificare e risolvere in seguito.

### Prossimi passi
1. Pagine di dettaglio — books, contents, people
2. Form di inserimento/modifica

---

## Sessione del 26 maggio 2026 — parte 5

### ritmo_web — pagine di dettaglio

Preparata issue per Copilot con specifica completa delle pagine di dettaglio per le tre entità principali.

**Lavoro assegnato a Copilot:**

- `ritmo_repository` — aggiungere `get_detail(id)` per `Book`, `Content`, `Person` con tutti i JOIN necessari (relazioni, persone, tag, lingue, luoghi, alias)
- `ritmo_presenter` — aggiungere `BookDetail`, `ContentDetail`, `PersonDetail` con struct di supporto condivise (`LinkedItem`, `LinkedItemWithRole`, `PersonWithRole`, `TagItem`, `LanguageItem`, `PlaceItem`); tutti `Serialize`; formattazione `PartialDate` in stringa con prefisso `~` se circa
- `ritmo_web` — implementare handler `detail` per books, contents, people con gestione 404
- Template Tera — `books/detail.html`, `contents/detail.html`, `people/detail.html` coerenti con lo stile delle liste

### Stato attuale delle pagine web

| Pagina | Stato |
|---|---|
| `GET /books` | ✅ funzionante |
| `GET /contents` | ✅ funzionante |
| `GET /people` | ✅ funzionante |
| `GET /books/:id` | ⏳ assegnato a Copilot |
| `GET /contents/:id` | ⏳ assegnato a Copilot |
| `GET /people/:id` | ⏳ assegnato a Copilot |
| Form di inserimento/modifica | ❌ placeholder |

---

---

## Sessione del 27 maggio 2026

### ritmo_web — pagine di dettaglio completate

Le pagine di dettaglio per le tre entità principali sono state implementate da Copilot e sono funzionanti.

### ritmo_web — form di inserimento/modifica unificati

Decisione architetturale: la pagina di dettaglio e la pagina di inserimento sono la stessa pagina. Nessuna rotta separata per il form.

**Pattern adottato:**
- `GET /books/:id` e `GET /books/new` → stesso template, dati pre-popolati o vuoti
- `POST /books/:id` → salva modifiche a record esistente
- `POST /books` → crea nuovo record
- Stesso pattern per `/contents` e `/people`

**Comportamento UI:**
- Campi sempre editabili, nessun toggle modifica/visualizzazione
- Bottone **Salva** — POST al server, in caso di successo redirect a `/entity/:id`
- Bottone **Nuovo** — naviga a `GET /entity/new`, svuota tutti i campi
- Bottone **Annulla** — torna alla lista senza salvare
- Navigazione da tastiera: Tab/Shift+Tab tra campi, Enter su campo singola riga avanza al successivo
- Campi relazionali complessi (persone+ruoli, tag, lingue) — sola lettura in questa iterazione; autocomplete ML da implementare in seguito

Preparata issue per Copilot con specifica completa.

### Bug fix — deserializzazione campi numerici opzionali

I campi numerici opzionali (`_year`, `_month`, `_day`, FK id) arrivano dal browser come stringa vuota `""` quando non compilati. `serde` non sa convertire `""` in `Option<i32>`.

**Fix:** aggiungere deserializzatore custom `deserialize_optional_i32` in `ritmo_presenter` e annotare con `#[serde(deserialize_with = "...")]` tutti i campi numerici opzionali nelle struct `BookFormData`, `ContentFormData`, `PersonFormData`.

I checkbox (`has_cover`, `has_paper`, `verified`, `*_circa`) richiedono `#[serde(default)]` — i checkbox non selezionati non vengono inviati dal browser.

Preparata issue per Copilot con lista completa dei campi da annotare per tutte e tre le struct.

### Stato attuale delle pagine web

| Pagina | Stato |
|---|---|
| `GET /books` | ✅ funzionante |
| `GET /contents` | ✅ funzionante |
| `GET /people` | ✅ funzionante |
| `GET /books/:id` | ✅ funzionante |
| `GET /contents/:id` | ✅ funzionante |
| `GET /people/:id` | ✅ funzionante |
| `GET /books/new` + `POST /books` | ⏳ assegnato a Copilot |
| `GET /contents/new` + `POST /contents` | ⏳ assegnato a Copilot |
| `GET /people/new` + `POST /people` | ⏳ assegnato a Copilot |
| `POST /books/:id` (save) | ⏳ assegnato a Copilot |
| `POST /contents/:id` (save) | ⏳ assegnato a Copilot |
| `POST /people/:id` (save) | ⏳ assegnato a Copilot |

---

## Sessione del 27 maggio 2026 — parte 2

### ritmo_web — cancellazione entità

Implementata la cancellazione per tutte le entità principali e di lookup.

#### Policy adottate

**Entità principali (Book, Content, Person):** cancellazione sempre permessa, con dialog di conferma JavaScript. Il cascade del database gestisce i legami dipendenti. Avvisi non bloccanti nella pagina di dettaglio:
- Book senza contenuti collegati → banner giallo: "Questo libro non ha contenuti collegati."
- Content senza autore → banner giallo: "Questo contenuto non ha un autore."

**Entità di lookup (Publisher, Series, Format, Role, Language):** cancellazione bloccata se referenziata. In caso di blocco, messaggio: "Impossibile eliminare: è utilizzata da N record."

**Tag:** sempre cancellabile, cascade sulle junction.

#### Rotte aggiunte

```
DELETE /books/:id
DELETE /contents/:id
DELETE /people/:id
DELETE /publishers/:id
DELETE /series/:id
DELETE /formats/:id
DELETE /roles/:id
DELETE /languages/:id
DELETE /tags/:id
```

Implementate via fetch JavaScript (HTML non supporta DELETE nativo nei form).

#### Crate modificati

- `ritmo_core` — aggiunto caso d'uso `delete` con policy per Publisher, Series, Format, Role, Language; delete diretto per Book, Content, Person; aggiunte funzioni `has_contents(book_id)` e `has_author(content_id)`
- `ritmo_repository` — aggiunto `is_referenced(id)` per Publisher, Series, Format, Role, Language
- `ritmo_presenter` — aggiunti flag `has_contents: bool` su `BookDetail` e `has_author: bool` su `ContentDetail`
- `ritmo_web` — handler delete per tutte le entità; banner condizionali nei template di dettaglio; pagine lista minimali per le entità di lookup con bottone Elimina

### Stato attuale delle pagine web

| Pagina | Stato |
|---|---|
| `GET /books` | ✅ funzionante |
| `GET /contents` | ✅ funzionante |
| `GET /people` | ✅ funzionante |
| `GET /books/:id` | ✅ funzionante |
| `GET /contents/:id` | ✅ funzionante |
| `GET /people/:id` | ✅ funzionante |
| `GET /books/new` + `POST /books` | ✅ funzionante |
| `GET /contents/new` + `POST /contents` | ✅ funzionante |
| `GET /people/new` + `POST /people` | ✅ funzionante |
| `POST /books/:id` (save) | ✅ funzionante |
| `POST /contents/:id` (save) | ✅ funzionante |
| `POST /people/:id` (save) | ✅ funzionante |
| `DELETE /books/:id` | ✅ funzionante |
| `DELETE /contents/:id` | ✅ funzionante |
| `DELETE /people/:id` | ✅ funzionante |
| `DELETE /publishers/:id` | ✅ funzionante |
| `DELETE /series/:id` | ✅ funzionante |
| `DELETE /formats/:id` | ✅ funzionante |
| `DELETE /roles/:id` | ✅ funzionante |
| `DELETE /languages/:id` | ✅ funzionante |
| `DELETE /tags/:id` | ✅ funzionante |
| Campi relazionali complessi (persone+ruoli, tag, lingue) nelle form | ❌ sola lettura |

---

## Sessione del 11 giugno 2026

### ritmo_web — widget Place (CRUD + unlink)

Implementato widget Tera riutilizzabile per la gestione dei luoghi associati a un'entità. Il widget è un frammento di template includibile in qualsiasi pagina.

#### Decisioni adottate

- Nessuna pagina `/places` autonoma — il widget è l'unico punto di accesso ai luoghi
- Il widget gestisce CRUD completo + unlink inline nella pagina ospite
- Aggiunta luogo tramite popup con ricerca semplice su `country`/`city` (LIKE, case-insensitive) — ML da collegare in seguito
- `d_publishers.country` rimosso da `PublisherDetail` e dal template (il campo resta nel DB senza migration)
- Policy delete luogo: cancella `d_places` solo se non referenziato da altre entità oltre a quella corrente

#### Crate modificati

- `ritmo_repository/src/place.rs` — `PlaceRepository` con save, get, update, delete, search, list_by_person, list_by_publisher, link/unlink per person e publisher, list_place_types
- `ritmo_presenter/src/place.rs` — `PlaceItem`, `PlaceFormData`, `PlaceUpdateData`, `build_place_items`
- `ritmo_presenter` — `PersonDetail` e `PublisherDetail` aggiornate con `places: Vec<PlaceItem>`; `PublisherDetail.country` rimosso
- `ritmo_core/src/place.rs` — `PlaceService` con create_and_link, update, delete (con policy), unlink, search, link
- `ritmo_web/src/handlers/places.rs` — handler search, create, update, delete, link, unlink
- `ritmo_web/src/router.rs` — rotte places aggiunte
- `templates/widgets/place_list.html` — widget riga luogo con edit inline, scollega, elimina
- `templates/widgets/place_popup.html` — popup ricerca/creazione luogo
- `templates/people/detail.html` — sezione luoghi integrata
- `templates/publishers/detail.html` — creato con sezione luoghi integrata

#### Rotte aggiunte

```
POST   /places
PUT    /places/:place_id
DELETE /places/:place_id
POST   /people/:id/places
DELETE /people/:id/places/:place_id
POST   /publishers/:id/places
DELETE /publishers/:id/places/:place_id
GET    /places/search?q=...
```

### Stato attuale delle pagine web

| Pagina | Stato |
|---|---|
| `GET /books` | ✅ funzionante |
| `GET /contents` | ✅ funzionante |
| `GET /people` | ✅ funzionante |
| `GET /publishers` | ✅ funzionante |
| `GET /books/:id` | ✅ funzionante |
| `GET /contents/:id` | ✅ funzionante |
| `GET /people/:id` | ✅ funzionante (con widget places) |
| `GET /publishers/:id` | ✅ funzionante (con widget places) |
| `GET /books/new` + `POST /books` | ✅ funzionante |
| `GET /contents/new` + `POST /contents` | ✅ funzionante |
| `GET /people/new` + `POST /people` | ✅ funzionante |
| `POST /books/:id` (save) | ✅ funzionante |
| `POST /contents/:id` (save) | ✅ funzionante |
| `POST /people/:id` (save) | ✅ funzionante |
| `DELETE /books/:id` | ✅ funzionante |
| `DELETE /contents/:id` | ✅ funzionante |
| `DELETE /people/:id` | ✅ funzionante |
| `DELETE /publishers/:id` | ✅ funzionante |
| `DELETE /series/:id` | ✅ funzionante |
| `DELETE /formats/:id` | ✅ funzionante |
| `DELETE /roles/:id` | ✅ funzionante |
| `DELETE /languages/:id` | ✅ funzionante |
| `DELETE /tags/:id` | ✅ funzionante |
| Link dettaglio dalle pagine lista (books, contents, people, publishers) | ⏳ assegnato a Copilot |
| Campi relazionali complessi (persone+ruoli, tag, lingue) nelle form | ❌ sola lettura |

### Prossimi passi

1. Merge issue "link dettaglio nelle pagine lista" (assegnata a Copilot)
2. Campi relazionali nelle form — persone+ruoli, tag, lingue
3. Estetica — wireframe già prodotto, applicazione rimandata a superficie stabile
4. FTS5

---

## Azioni future

### 1. Implementare FTS5
Virtual table FTS5 su titoli (`d_books.name`, `d_contents.name`), note, nomi persone. Collegare alla ricerca principale.

### 2. Sistema ML
Esiste in ritmo2 ed è funzionante. Va importato quando si inizia a inserire dati reali — serve per normalizzazione nomi, luoghi, tag. Il popup place è il primo punto di integrazione previsto.

---

## Architettura — principi chiave

- `ritmo_presenter` è il layer stabile tra core e tutti i frontend — nessun frontend dipende direttamente da `sqlx` o `ritmo_db`.
- Le date bibliografiche usano quattro colonne nullable (`date_year`, `date_month`, `date_day`, `date_circa`), distinte dai campi tecnici Unix timestamp.
- Distribuzione portabile: cartella self-contained con launcher + eseguibile + `data/`.
- I18n al layer presenter con fallback chain — solo per lookup di sistema, non per dati utente.
- Tabella `s_page_fields` sostituisce le definizioni di campo hardcoded.
- Lookup di sistema (con i18n): `d_roles`, `d_types`, `d_formats`, `d_languages`, `s_person_language_roles`, `s_place_types`, `s_content_language_roles`, `s_book_language_roles`.
- Dati utente (`d_tags`, `d_publishers`, `d_series`, ecc.): nessun seeding, nessuna i18n.

---

## Convenzioni di lavoro

- **Copilot** riceve istruzioni tramite issue su GitHub con assegnazione a `@copilot`. I commenti nelle PR funzionano per correzioni.
- **Branch**: lo crea Copilot — il nome sarà diverso da quello indicato nel prompt. Fare sempre `git fetch origin` prima del checkout.
- **Ciclo standard**: checkout → review file → build → unisci → pulizia locale.
- **Modifiche locali**: salvare con `git stash` prima di cambiare branch.
- **Documentazione**: i file in `/doc` sono vincolanti — qualsiasi modifica al dominio o all'architettura va riflessa lì.

---

## Convenzione nomi tabelle SQL (ARCHITECTURE.md)

Obbligatoria. Ogni tabella deve avere uno di questi prefissi:

- `d_` — tabelle di dominio: dati immessi dall'utente o da tool esterni
- `x_` — tabelle di relazione: legami molti-a-molti tra entità di dominio
- `s_` — tabelle di sistema: dati interni al funzionamento dell'applicazione

Nessuna tabella senza prefisso. Schema, seed e query in `ritmo_repository` devono essere sempre allineati.


---

## Sessione del 15 giugno 2026 — Revisione dell'interfaccia

### Decisione: rifacimento completo di `ritmo_web`

L'interfaccia web esistente è stata valutata insufficiente. Il problema non è tecnico ma concettuale: il modello "pagine separate per ogni entità" non corrisponde al modello mentale dell'applicazione. Il frontend va rifatto da zero con una direzione chiara.

**Tutto il resto del progetto è intoccabile.** `ritmo_repository`, `ritmo_core`, `ritmo_presenter`, lo schema SQL, il seeding — nessuna modifica. Il rifacimento riguarda esclusivamente `ritmo_web`.

---

### Modello utente chiarito

**Modalità di uso primarie:**

1. **Navigazione esplorativa** — l'utente cerca un libro con ricordo vago (autore incerto, titolo parziale, ricorda che era un racconto in un'antologia). Il percorso è: cerco → filtro → trovo → approfondisco → confronto.

2. **Data entry intensivo** — l'utente ha un batch di EPUB da catalogare. Il flusso è ripetitivo: apro un EPUB → estraggo quello che posso → completo a mano → salvo → prossimo.

**Distribuzione del tempo stimata:**
- Liste: 40%
- Vista filtri: 20%
- Vista libri: 15%
- Vista contenuti: 15%
- Altre visualizzazioni (autori, persone, luoghi, ecc.): 10%

**Deployment:** server web locale (`localhost`), accessibile da più browser sulla stessa rete locale (es. utente principale + moglie). Nessun server remoto.

**Sessioni utente:** identificazione tramite cookie di sessione, senza login esplicito. Ogni browser ha la propria sessione. I filtri salvati e le preferenze sono per sessione, memorizzati nel database (`s_filter_sets`, `s_filter_conditions` già presenti nello schema). Nessuna gestione password.

---

### Architettura del nuovo frontend

**Stack tecnologico:**
- **Axum** — server web locale (invariato)
- **Tera** — template engine server-rendered (invariato)
- **HTMX** — reattività senza SPA; il server restituisce frammenti HTML, HTMX aggiorna il DOM in place
- **FTS5** — ricerca full-text SQLite per autocomplete e ricerca fuzzy
- **CSS** — layout applicativo secondo il wireframe (IBM Plex Sans/Mono, dark theme, accenti gold/amber)

**Modello di layout (dal wireframe `rwf.html`):**
┌─────────────────────────────────────────────────┐
│ TOPBAR: logo · toggle vista · search · azioni   │
├──────────────┬──────────────────────────────────┤
│              │                                  │
│   SIDEBAR    │         LISTA / CONTENUTO        │
│   (filtri)   │                                  │
│   240px      │                                  │
│              │                                  │
├──────────────┴──────────────────────────────────┤
│ STATUSBAR: contatori · filtri attivi · Kobo     │
└─────────────────────────────────────────────────┘
- Layout fisso a quattro zone: topbar, sidebar, contenuto, statusbar
- Una sola superficie applicativa — nessuna navigazione tra pagine
- Il toggle in topbar cambia la vista attiva (Libri / Contenuti / Persone / ecc.)
- I filtri nella sidebar si accumulano e restano visibili e attivi
- La lista si aggiorna via HTMX senza reload
- Le viste di dettaglio e i form si aprono come **popup/modale** sovrapposto alla lista — la lista rimane sotto, chiudendo il popup si torna esattamente dove si era

**Autocomplete — elemento centrale dell'interfaccia:**

L'autocomplete è la funzione più importante del progetto. È il meccanismo principale per navigare, filtrare e collegare entità. Funzionamento:

- L'utente digita nel campo di ricerca/filtro
- Si apre un popup con lista di items che matchano quanto digitato
- La lista si riduce man mano che si digita
- La lista è cliccabile
- Implementazione: `hx-trigger="keyup changed delay:200ms"` → GET al server → server interroga FTS5 → restituisce frammento HTML lista → HTMX sostituisce in place
- Per l'autocomplete con selezione multipla (filtri) può essere necessaria una piccola libreria JS dedicata (es. Tom Select)

---

### Stato di `ritmo_web` dopo questa sessione

Il crate `ritmo_web` va svuotato e ricostruito. Si conservano:
- La struttura del crate e il suo posto nel workspace
- Le dipendenze: `axum`, `tokio`, `tera`, `dotenvy`, `serde`, `serde_json`
- Il file `.env` con `DATABASE_URL` e `WEB_BIND`
- Il pattern di inizializzazione: `ritmo_db::create_sqlite_pool` → `RepositoryContext` → `AppState`

Si aggiunge:
- `htmx` (CDN, nessuna dipendenza Rust)
- Font IBM Plex Sans e IBM Plex Mono (Google Fonts o locale)

Si butta:
- Tutti i template esistenti
- Tutti gli handler esistenti
- Il router esistente
- Tutta la logica di form e dettaglio basata su pagine separate

Il primo passo della ricostruzione è il layout shell — la struttura HTML fissa con topbar, sidebar, area contenuto e statusbar, senza ancora contenuto reale. Da lì si costruisce incrementalmente.

## `ritmo_repository`

### Responsabilità

Unico punto di accesso al database. Esegue tutte le operazioni di lettura e scrittura su SQLite. Conosce sia le struct di dominio (`ritmo_domain`) che la struttura del database (`ritmo_db`). Non interpreta il significato dei dati, non contiene logica applicativa.

### Contenuto

#### Operazioni sulle entità di dominio

Per ciascuna entità — `Alias`, `Book`, `Content`, `Format`, `Genre`, `Language`, `Person`, `Publisher`, `Role`, `Series`, `Tag` — il repository espone:

-   `save` — inserimento di un nuovo record, restituisce l'id assegnato
-   `get` — lettura per id
-   `get_by_key` — lettura per chiave naturale (nome, codice, ecc.) dove applicabile
-   `update` — aggiornamento di un record esistente
-   `delete` — cancellazione per id
-   `list_all` — lista completa ordinata
-   `search` — ricerca per pattern su campi testuali
-   `get_or_create` — ricerca per chiave naturale, creazione se non esiste

#### Operazioni sulle tabelle di relazione

Per ciascuna tabella di relazione — `x_books_contents`, `x_books_people_roles`, `x_books_tags`, `x_contents_people_roles`, `x_contents_tags`, `x_content_languages`, `x_book_languages`, `x_person_languages` — il repository espone:

-   `create` — creazione del legame
-   `delete` — rimozione del legame
-   `list_by_*` — lista per uno dei due lati della relazione

#### Mapping

Traduce righe SQLite in struct di dominio e viceversa. È l'unico crate che conosce entrambe le rappresentazioni.

### Dipendenze esterne

-   `sqlx` — query e mapping
-   `tokio` — runtime async

### Dipendenze interne

-   `ritmo_domain` — struct di dominio
-   `ritmo_db` — connessione e pool
-   `ritmo_errors` — errori di accesso al database

### Regole

-   Solo questo crate legge e scrive il database.
-   Nessuna logica applicativa — nessuna decisione su cosa fare con i dati.
-   Nessuna logica di presentazione.
-   Le policy di delete (es. blocco se referenziato) non appartengono qui — appartengono a `ritmo_core`.
-   Le operazioni sulle tabelle di relazione sono sempre guidate dal contesto in `ritmo_core`.

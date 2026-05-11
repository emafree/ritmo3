### Ritmo — Documento Architetturale

#### Struttura dei crate

```
ritmo_errors      →  tipi di errore, RitmoResult, RitmoReporter
ritmo_domain      →  struct di dominio, tipi di supporto
ritmo_db          →  schema, migrations, seeding, connessione
ritmo_repository  →  CRUD, mapping SQL↔dominio
ritmo_core        →  logica applicativa, orchestrazione, policy
ritmo_presenter   →  view model, i18n
ritmo_tui         →  interfaccia utente testuale
```

* * *

##### `ritmo_errors`

Definisce i tipi di errore del progetto e le strutture per il reporting. Non dipende da nessun altro crate del progetto.

**Non può:** contenere logica applicativa, accedere al DB, contenere logica di presentazione.

* * *

##### `ritmo_domain`

Contiene le struct di dominio del progetto. Non dipende da nessun altro crate del progetto. È il fondamento comune che tutti gli altri crate importano.

**Entità di dominio autonome:** `Book`, `Content`, `Person`, `Publisher`, `Series`, `Format`, `Genre`, `Role`, `Tag`, `Language`.

**Strutture di supporto:** `Alias` (attributo di `Person`), `Place` (attributo di `Person`), `PartialDate` (tipo di supporto per le date).

**Non può:** accedere al DB, contenere logica applicativa, contenere logica di presentazione, definire trait CRUD o di accesso ai dati.

* * *

##### `ritmo_db`

Contiene la connessione al database, le migrations, e lo schema SQLite. Non definisce struct di dominio.

**Non può:** contenere logica applicativa, contenere logica di presentazione, definire struct di dominio.

* * *

##### `ritmo_repository`

Contiene l'implementazione concreta delle operazioni CRUD su SQLite. È l'unico crate che conosce sia le struct di dominio (`ritmo_domain`) che le strutture del database (`ritmo_db`). Mappa righe SQL su struct di dominio e viceversa. Non interpreta il significato dei dati.

**Non può:** contenere logica applicativa, contenere logica di presentazione, eseguire SQL al di fuori del mapping diretto.

* * *

##### `ritmo_core`

Contiene la logica applicativa e l'orchestrazione. Sa cosa significano i dati e cosa fare con essi. Valida, coordina operazioni tra entità, gestisce i casi d'uso. Non accede direttamente al database.

**Non può:** accedere direttamente al DB, contenere logica di presentazione.

* * *

##### `ritmo_presenter`

Costruisce view model pronti per qualunque interfaccia utente. Riceve dati da `ritmo_core` e restituisce struct formattate e pronte per il rendering. Non sa come i dati vengono visualizzati né come vengono prodotti.

**Non può:** accedere al DB, scrivere dati, contenere logica applicativa, importare `ritmo_repository` o `ritmo_db`.

* * *

##### `ritmo_tui` / future interfacce

Responsabili esclusivamente del rendering e dell'input utente. Chiamano `ritmo_presenter` per i dati da mostrare e `ritmo_core` per le azioni.

**Non possono:** accedere al DB, importare `ritmo_repository`, contenere logica applicativa.

* * *

#### Dipendenze tra crate

```
ritmo_tui → ritmo_presenter → ritmo_core → ritmo_repository → ritmo_db
                                    |                |
                                    +--→ ritmo_domain ←--+

ritmo_errors   →  importato da tutti i crate
ritmo_domain   →  importato da tutti tranne ritmo_errors e ritmo_db
```

Nessun crate salta livelli. La direzione delle dipendenze è sempre verso il basso. Nessun crate dei livelli inferiori conosce i livelli superiori.

* * *

#### Convenzioni di nomenclatura del database

Le tabelle SQLite seguono una convenzione di prefisso obbligatoria:

-   `d_` — tabelle di dominio: dati immessi dall'utente o da tool esterni (`d_books`, `d_contents`, `d_people`, ecc.)
-   `x_` — tabelle di relazione: legami molti-a-molti tra entità di dominio (`x_books_contents`, `x_books_people_roles`, ecc.)
-   `s_` — tabelle di sistema: dati interni al funzionamento dell'applicazione (`s_audit_log`, `s_page_fields`, ecc.)

Qualunque nuova tabella deve rispettare questa convenzione. Nessuna tabella senza prefisso.

* * *

#### Regole che non si violano

1.  Solo `ritmo_repository` scrive e legge dal database.
2.  Solo `ritmo_repository` conosce sia le struct di dominio che le strutture DB.
3.  `ritmo_presenter` non importa mai `ritmo_repository` né `ritmo_db`.
4.  Le struct di dominio sono definite solo in `ritmo_domain`.
5.  `ritmo_domain` non contiene trait CRUD, trait di accesso ai dati, o dipendenze da `sqlx`, `tokio`, `rust-i18n`.
6.  Nessuna interfaccia utente contiene logica applicativa.
7.  La documentazione architetturale precede sempre il codice. Non viene delegata.
8.  Nessun documento contenuto in `/doc` può essere trascurato o disatteso.

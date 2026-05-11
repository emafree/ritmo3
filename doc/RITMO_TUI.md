## `ritmo_tui`

### Responsabilità

Interfaccia utente testuale basata su Ratatui. Gestisce il rendering e l'input utente. Chiama `ritmo_presenter` per i dati da mostrare e `ritmo_core` per le azioni. Non contiene logica applicativa né logica di accesso al database.

### Scopo primario

Permettere la creazione e gestione rapida del database con contenuti reali, come strumento di lavoro durante lo sviluppo del progetto.

### Contenuto

#### Schermate previste

-   Lista libri — navigazione, ricerca, filtri
-   Dettaglio libro — visualizzazione completa, editing
-   Lista contenuti — navigazione, ricerca, filtri
-   Dettaglio contenuto — visualizzazione completa, editing
-   Lista persone — navigazione, ricerca
-   Dettaglio persona — visualizzazione completa, editing
-   Schermate di gestione entità indipendenti — tag, publisher, series, format, genre, role, language

#### Navigazione

Tastiera come unico mezzo di interazione.

### Dipendenze esterne

-   `ratatui` — rendering TUI
-   `crossterm` — gestione input e terminale

### Dipendenze interne

-   `ritmo_presenter` — view model
-   `ritmo_core` — azioni applicative
-   `ritmo_errors` — gestione errori

### Regole

-   Nessun accesso al database, diretto o indiretto.
-   Non importa mai `ritmo_repository` né `ritmo_db`.
-   Nessuna logica applicativa.
-   Nessuna logica di trasformazione dati — tutto passa per `ritmo_presenter`.

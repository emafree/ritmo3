## `ritmo_core`

### Responsabilità

Contiene la logica applicativa e l'orchestrazione. È l'unico crate che sa cosa significano i dati e cosa fare con essi. Valida, coordina operazioni tra entità, applica le policy di dominio. Non accede direttamente al database — delega sempre a `ritmo_repository`.

### Casi d'uso

#### Entità indipendenti

Entità che esistono da sole, senza relazioni obbligatorie. Le operazioni sono create, update, delete con policy.

-   `Tag`
-   `Publisher`
-   `Series`
-   `Format`
-   `Genre`
-   `Role`
-   `Language`

#### Entità autonome ma relazionabili

Entità che esistono indipendentemente ma possono essere collegate ad altre. Le operazioni sono create, update, delete con policy. `Alias` e `Place` sono attributi di `Person` e vengono gestiti nel suo contesto.

-   `Person` — include la gestione di alias e luoghi
-   `Book` — include la gestione di publisher, format, series
-   `Content` — include la gestione di type, genre

#### Operazioni contestuali

Collegamento e scollegamento tra entità già esistenti.

-   Persona + ruolo → libro
-   Persona + ruolo → contenuto
-   Tag → libro
-   Tag → contenuto
-   Contenuto → libro
-   Lingua → contenuto
-   Lingua → libro
-   Lingua → persona

#### Sync metadata

Allineamento dei metadati tra il filesystem e il database.

### Policy di dominio

-   Delete bloccata se l'entità è referenziata, per le entità che lo richiedono.
-   Le policy sono definite qui e solo qui — `ritmo_repository` cancella senza verifiche.

### Dipendenze esterne

-   `tokio` — runtime async

### Dipendenze interne

-   `ritmo_domain` — struct di dominio
-   `ritmo_repository` — accesso al database
-   `ritmo_errors` — errori applicativi

### Regole

-   Nessun accesso diretto al database — sempre tramite `ritmo_repository`.
-   Nessuna logica di presentazione.
-   Le policy di delete vivono solo qui.
-   Qualunque sorgente di dati (utente, EPUB importer, batch) chiama `ritmo_core` — `ritmo_core` non conosce la sorgente.

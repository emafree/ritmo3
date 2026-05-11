## `ritmo_domain`

### Responsabilità

Definisce le struct di dominio del progetto. È il crate fondante da cui tutti gli altri dipendono. Non conosce il database, non conosce la presentazione, non conosce la logica applicativa.

### Contenuto

#### Entità di dominio autonome

Entità con vita propria, gestibili indipendentemente dalle altre:

-   `Book` — contenitore fisico o digitale di uno o più contenuti
-   `Content` — opera letteraria indipendente dalla sua veste editoriale
-   `Person` — autore, traduttore, curatore o altra figura associata a un contenuto od ad un libro
-   `Publisher` — editore associato a un libro
-   `Series` — collana editoriale a cui appartiene un libro
-   `Format` — formato fisico o digitale del libro (chiave i18n)
-   `Genre` — genere letterario del contenuto (chiave i18n)
-   `Role` — ruolo di una persona rispetto a un contenuto (chiave i18n)
-   `Tag` — etichetta libera associata a un libro o contenuto
-   `Language` — lingua, associabile ad un contenuto, ad un libro, a persone

#### Strutture di supporto

Strutture che esistono solo in relazione a una entità autonoma:

-   `Alias` — nome alternativo di una `Person`
-   `Place` — luogo associato a una `Person` (nascita, residenza, ecc.)
-   `PartialDate` — data parziale o approssimata, usata per date di pubblicazione, nascita, morte. Supporta anno/mese/giorno opzionali e flag `circa`. Gestisce date storiche con anno negativo.

#### Cosa non contiene

-   Nessun metodo che accede al database
-   Nessun trait `CrudModel` o `GetOrCreateModel` — la logica CRUD appartiene a `ritmo_repository`
-   Nessun trait `I18nDisplayable` — la traduzione delle chiavi appartiene a `ritmo_presenter`
-   Nessun artefatto interno (`PageFieldRow`, `pending_sync`)

### Dipendenze esterne

-   `serde` — serializzazione/deserializzazione delle struct

### Dipendenze interne

Nessuna. Questo crate non dipende da nessun altro crate del progetto.

### Regole

-   Le struct di dominio sono definite solo qui e solo qui.
-   Nessun metodo async, nessun accesso al database.
-   Nessuna dipendenza da `sqlx`, `tokio`, `rust-i18n`.
-   Aggiungere una struct richiede aggiornamento di questo documento.
-   Una struct in questo crate non implica necessariamente una entità autonoma — alcune sono attributi o tipi di supporto.

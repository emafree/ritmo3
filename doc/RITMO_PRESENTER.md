## `ritmo_presenter`

### Responsabilità

Trasforma i dati di dominio in view model pronti per qualunque interfaccia utente. È l'unico punto di trasformazione tra il dominio e la presentazione. Non sa come i dati vengono visualizzati, non sa come vengono prodotti.

### Contenuto

#### View model

Struct pronte per il rendering, costruite a partire dalle struct di dominio. Esempi:

-   Lista di libri — id, titolo, autori, formato, serie
-   Dettaglio libro — tutti i campi, contenuti collegati, persone con ruoli, tag
-   Lista di contenuti — id, titolo, autori, genere, tipo
-   Dettaglio contenuto — tutti i campi, libri collegati, persone con ruoli, tag, lingue
-   Lista di persone — id, nome, date
-   Dettaglio persona — tutti i campi, alias, luoghi, libri e contenuti collegati

#### Traduzione chiavi i18n

`I18nDisplayable` — trait per la traduzione delle chiavi canoniche di `Format`, `Genre`, `Role`, `Type` nella lingua corrente dell'interfaccia.

### Dipendenze esterne

-   `rust-i18n` — traduzione delle chiavi i18n

### Dipendenze interne

-   `ritmo_domain` — struct di dominio
-   `ritmo_errors` — errori di presentazione

### Regole

-   Nessun accesso al database, diretto o indiretto.
-   Nessuna scrittura di dati.
-   Nessuna logica applicativa.
-   Non importa mai `ritmo_repository` né `ritmo_db`.
-   Riceve solo struct di dominio da `ritmo_domain` — non riceve risultati SQL grezzi.

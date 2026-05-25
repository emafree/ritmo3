# Ritmo Import — Specifica del formato TOML

## Panoramica

I dati vengono importati tramite due tipi di file TOML:

- `people.toml` — persone con dati biografici completi
- `books.toml` — libri con tutte le relazioni inline

**Ordine obbligatorio:** importare sempre `people.toml` prima di `books.toml`.
Le persone definite in `people.toml` vengono trovate per nome quando compaiono in `books.toml`.

---

## Risoluzione per nome

L'importatore risolve tutte le entità per nome, non per id.
Quando incontra `publisher = "Nord"`, cerca `Nord` in `d_publishers`.
Se non esiste, lo crea. Se esiste, lo collega.

Lo stesso vale per: publisher, series, tag, format, genre, role, language, person.

Le persone vengono cercate per corrispondenza esatta del campo `name`.
Se una persona compare in `books.toml` ma non è stata definita in `people.toml`,
viene creata con il solo campo `name` — senza dati biografici.

---

## File: `people.toml`

### Struttura

```toml
[[person]]
# campi della persona

[[person.language]]
# lingua associata

[[person.place]]
# luogo associato
```

### Campi `[[person]]`

| Campo | Tipo | Obbligatorio | Note |
|---|---|---|---|
| `name` | stringa | **sì** | chiave di ricerca — deve essere unico e coerente tra i file |
| `display_name` | stringa | no | nome da mostrare in UI se diverso da `name` |
| `given_name` | stringa | no | nome di battesimo |
| `surname` | stringa | no | cognome |
| `middle_names` | stringa | no | nomi intermedi |
| `title` | stringa | no | titolo onorifico (Dr., Prof., Sir…) |
| `suffix` | stringa | no | suffisso (Jr., III…) |
| `birth_date` | data parziale | no | vedi formato date |
| `death_date` | data parziale | no | vedi formato date |
| `biography` | stringa | no | testo libero, supporta multiriga con `"""` |
| `aliases` | lista di stringhe | no | nomi alternativi, pseudonimi, traslitterazioni |

### Campi `[[person.language]]`

| Campo | Tipo | Obbligatorio | Valori validi |
|---|---|---|---|
| `language` | stringa | **sì** | codice ISO 639-1 (2 char) o ISO 639-3 (3 char) |
| `role` | stringa | **sì** | `native` `writing` `fluent` `reading` `other` |

### Campi `[[person.place]]`

| Campo | Tipo | Obbligatorio | Valori validi |
|---|---|---|---|
| `type` | stringa | **sì** | `birth` `death` `residence` `activity` `other` |
| `continent` | stringa | no | testo libero |
| `country` | stringa | no | testo libero |
| `city` | stringa | no | testo libero |
| `circa` | bool | no | default `false` — luogo incerto |
| `disputed` | bool | no | default `false` — appartenenza storicamente contestata |

---

## File: `books.toml`

### Struttura

```toml
[[book]]
# campi del libro
tags = [...]

[[book.language]]
# lingua del libro (testo fisico dell'edizione)

[[book.person]]
# persona associata al libro

[[book.content]]
# opera letteraria contenuta nel libro

[[book.content.language]]
# lingua del contenuto

[[book.content.person]]
# persona associata al contenuto (se diversa da quella del libro)
```

### Campi `[[book]]`

| Campo | Tipo | Obbligatorio | Note |
|---|---|---|---|
| `name` | stringa | **sì** | titolo dell'edizione |
| `original_title` | stringa | no | titolo originale se diverso |
| `format` | stringa | no | vedi valori validi sotto |
| `publisher` | stringa | no | nome testuale — cercato/creato |
| `series` | stringa | no | nome testuale — cercata/creata |
| `series_index` | intero | no | posizione nella collana (> 0) |
| `publication_date` | data parziale | no | data di questa edizione |
| `isbn` | stringa | no | |
| `notes` | stringa | no | testo libero |
| `has_cover` | bool | no | default `false` |
| `has_paper` | bool | no | default `false` |
| `file_link` | stringa | no | percorso al file digitale |
| `tags` | lista di stringhe | no | etichette libere — cercate/create |

**Valori validi per `format`:**
`epub` `pdf` `mobi` `azw3` `djvu` `cbz` `cbr` `txt` `other`

### Campi `[[book.language]]`

| Campo | Tipo | Obbligatorio | Valori validi |
|---|---|---|---|
| `iso2` | stringa | condizionale | codice ISO 639-1 (2 caratteri) |
| `iso3` | stringa | condizionale | codice ISO 639-3 (3 caratteri) |
| `name` | stringa | condizionale | nome ufficiale lingua |
| `role` | stringa | **sì** | `actual` `other` |

È obbligatorio valorizzare almeno uno tra `iso2`, `iso3`, `name`.

### Campi `[[book.person]]`

| Campo | Tipo | Obbligatorio | Note |
|---|---|---|---|
| `name` | stringa | **sì** | deve corrispondere al campo `name` di una persona |
| `role` | stringa | **sì** | vedi valori validi sotto |

**Valori validi per `role`:**
`author` `translator` `editor` `cover_artist` `illustrator`
`editorial_consultant` `photographer` `preface_writer`
`afterword_writer` `commentator` `other`

### Campi `[[book.content]]`

| Campo | Tipo | Obbligatorio | Note |
|---|---|---|---|
| `name` | stringa | **sì** | titolo dell'opera |
| `original_title` | stringa | no | titolo originale se diverso |
| `type` | stringa | no | vedi valori validi sotto |
| `genre` | stringa | no | vedi valori validi sotto |
| `publication_date` | data parziale | no | data di prima pubblicazione dell'opera |
| `notes` | stringa | no | testo libero |

**Valori validi per `type`:**
`novel` `essay` `short_story` `short_story_collection`
`biography` `autobiography` `manual` `comic` `poetry` `theatre` `other`

**Valori validi per `genre`:**
`adventure` `biography` `crime` `dystopia` `fantasy` `historical`
`horror` `humor` `mystery` `philosophy` `romance` `science_fiction`
`thriller` `travel` `other`

### Campi `[[book.content.language]]`

| Campo | Tipo | Obbligatorio | Valori validi |
|---|---|---|---|
| `iso2` | stringa | condizionale | codice ISO 639-1 (2 caratteri) |
| `iso3` | stringa | condizionale | codice ISO 639-3 (3 caratteri) |
| `name` | stringa | condizionale | nome ufficiale lingua |
| `role` | stringa | **sì** | `original` `source` `actual` `other` |

È obbligatorio valorizzare almeno uno tra `iso2`, `iso3`, `name`.

### Campi `[[book.content.person]]`

Stessi campi di `[[book.person]]`. Da usare solo quando le persone associate
al contenuto differiscono da quelle del libro — tipicamente nelle antologie.

---

## Formato delle date parziali

Le date supportano anno, mese e giorno opzionali, più un flag `circa`.

```toml
# Solo anno
publication_date = { year = 1969 }

# Anno e mese
birth_date = { year = 1929, month = 10 }

# Data completa
death_date = { year = 2018, month = 1, day = 22 }

# Data approssimata
birth_date = { year = 1200, circa = true }

# Data antica (a.C.) — anno negativo
publication_date = { year = -750, circa = true }
```

Il campo `circa` è `false` per default — va indicato solo quando è `true`.

---

## Codici lingua

Utilizzare i codici ISO 639-1 (2 caratteri) quando disponibili,
ISO 639-3 (3 caratteri) per le lingue antiche o senza codice a 2 caratteri.

Lingue pre-seeded nel database:

| Codice | Lingua |
|---|---|
| `it` | Italiano |
| `en` | Inglese |
| `fr` | Francese |
| `es` | Spagnolo |
| `de` | Tedesco |
| `zh` | Cinese |
| `ja` | Giapponese |
| `ko` | Coreano |
| `la` | Latino |
| `grc` | Greco antico |

Altre lingue vengono create automaticamente se non presenti nel database,
purché il codice ISO sia valido.

---

## Comportamento in caso di duplicati

| Entità | Comportamento |
|---|---|
| `Person` | Ricerca per `name` esatto. Se trovata: aggiorna i campi vuoti con i nuovi dati, non sovrascrive quelli già presenti. Se non trovata: crea. |
| `Publisher` | Ricerca per nome (case-insensitive, trim). Se non trovato: crea. |
| `Series` | Ricerca per nome (case-insensitive, trim). Se non trovata: crea. |
| `Tag` | Ricerca per nome (case-insensitive, trim). Se non trovato: crea. |
| `Book` | Ricerca per `name` + `publisher` + `publication_date.year`. Se trovato: aggiorna. Se non trovato: crea. |
| `Content` | Ricerca per `name`. Se trovato: collega senza duplicare. Se non trovato: crea. |
| Relazioni (`x_*`) | Insert idempotente — se la relazione esiste già viene ignorata. |

---

## Ordine di importazione consigliato

1. `people.toml` — persone con dati biografici ricchi
2. `books.toml` — libri con relazioni inline

È possibile avere più file per tipo (es. `people-classici.toml`, `people-contemporanei.toml`,
`books-fantascienza.toml`, `books-saggistica.toml`). L'importatore li processa nell'ordine
in cui vengono passati da riga di comando.

---

## Esempio di invocazione (da definire)

```bash
ritmo_import people.toml people-extra.toml
ritmo_import books.toml books-antologie.toml
```

o tutto in una volta con risoluzione automatica dell'ordine:

```bash
ritmo_import --people people.toml --books books.toml
```

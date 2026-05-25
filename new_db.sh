#!/usr/bin/env bash
set -euo pipefail

SCHEMA_SQL="/home/ema/Projects/ritmo3/ritmo_db/schema/schema.sql"
SEED_LOOKUPS_SQL="/home/ema/Projects/ritmo3/ritmo_db/schema/seed_lookups.sql"
SEED_PAGE_FIELDS_SQL="/home/ema/Projects/ritmo3/ritmo_db/schema/seed_page_fields.sql"

usage() {
    echo "Usage: $0 <database_file>"
    echo "  Example: $0 /home/ema/ritmo/template.db"
    exit 1
}

# --- argomento obbligatorio ---
if [[ $# -ne 1 ]]; then
    usage
fi

DB_FILE="$1"

# --- il db non deve già esistere ---
if [[ -e "$DB_FILE" ]]; then
    echo "Errore: il file '$DB_FILE' esiste già. Scegli un altro nome o cancellalo prima." >&2
    exit 1
fi

# --- verifica che i file SQL esistano ---
for f in "$SCHEMA_SQL" "$SEED_LOOKUPS_SQL" "$SEED_PAGE_FIELDS_SQL"; do
    if [[ ! -f "$f" ]]; then
        echo "Errore: file SQL non trovato: $f" >&2
        exit 1
    fi
done

# --- verifica che sqlite3 sia disponibile ---
if ! command -v sqlite3 &>/dev/null; then
    echo "Errore: sqlite3 non trovato nel PATH." >&2
    exit 1
fi

# --- crea la directory di destinazione se non esiste ---
DB_DIR="$(dirname "$DB_FILE")"
if [[ ! -d "$DB_DIR" ]]; then
    mkdir -p "$DB_DIR"
    echo "Directory creata: $DB_DIR"
fi

echo "Creazione del database: $DB_FILE"

sqlite3 "$DB_FILE" <<EOF
.bail on
.read $SCHEMA_SQL
.read $SEED_LOOKUPS_SQL
.read $SEED_PAGE_FIELDS_SQL
EOF

echo "Database creato con successo: $DB_FILE"

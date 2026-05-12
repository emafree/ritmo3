BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "system_config" (
	"key"	TEXT,
	"value"	TEXT,
	"description"	TEXT,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("key")
);
CREATE TABLE IF NOT EXISTS "audit_log" (
	"id"	INTEGER,
	"table_name"	TEXT NOT NULL,
	"record_id"	INTEGER NOT NULL,
	"operation"	TEXT NOT NULL CHECK("operation" IN ('INSERT', 'UPDATE', 'DELETE')),
	"old_values"	TEXT,
	"new_values"	TEXT,
	"timestamp"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"user_id"	TEXT,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "stats_cache" (
	"id"	INTEGER,
	"cache_key"	TEXT NOT NULL UNIQUE,
	"cache_value"	TEXT NOT NULL,
	"expires_at"	INTEGER NOT NULL,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "formats" (
	"id"	INTEGER,
	"key"	TEXT NOT NULL UNIQUE,
	"created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "publishers" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL,
	"country"	TEXT,
	"website"	TEXT,
	"notes"	TEXT,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "series" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL UNIQUE,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "roles" (
	"id"	INTEGER,
	"key"	TEXT NOT NULL UNIQUE,
	"created_at"    INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "role_translations" (
	"id" INTEGER PRIMARY KEY AUTOINCREMENT,
	"role_id" INTEGER NOT NULL REFERENCES "roles"("id") ON DELETE CASCADE,
	"language_code" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	UNIQUE("role_id", "language_code")
);
CREATE TABLE IF NOT EXISTS "format_translations" (
	"id" INTEGER PRIMARY KEY AUTOINCREMENT,
	"format_id" INTEGER NOT NULL REFERENCES "formats"("id") ON DELETE CASCADE,
	"language_code" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	UNIQUE("format_id", "language_code")
);
CREATE TABLE IF NOT EXISTS "tags" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL UNIQUE,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "genres" (
	"id"	INTEGER,
	"key"	TEXT NOT NULL UNIQUE,
	"created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "genre_translations" (
	"id" INTEGER PRIMARY KEY AUTOINCREMENT,
	"genre_id" INTEGER NOT NULL REFERENCES "genres"("id") ON DELETE CASCADE,
	"language_code" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	UNIQUE("genre_id", "language_code")
);
CREATE TABLE IF NOT EXISTS "types" (
	"id"	INTEGER,
	"key"	TEXT NOT NULL UNIQUE,
	"created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "type_translations" (
	"id" INTEGER PRIMARY KEY AUTOINCREMENT,
	"type_id" INTEGER NOT NULL REFERENCES "types"("id") ON DELETE CASCADE,
	"language_code" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	UNIQUE("type_id", "language_code")
);

CREATE TABLE IF NOT EXISTS "languages" (
    id            INTEGER,
    iso_code_2char TEXT,
    iso_code_3char TEXT,
    official_name  TEXT NOT NULL,
    native_name    TEXT,
    created_at     INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at     INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    PRIMARY KEY (id AUTOINCREMENT),
    CHECK (iso_code_2char IS NOT NULL OR iso_code_3char IS NOT NULL),
    UNIQUE (iso_code_2char, iso_code_3char)
);

CREATE TABLE IF NOT EXISTS "people" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL,
	"display_name"	TEXT,
	"given_name"	TEXT,
	"surname"	TEXT,
	"middle_names"	TEXT,
	"title"	TEXT,
	"suffix"	TEXT,
	"birth_date_year"	INTEGER,
	"birth_date_month"	INTEGER CHECK("birth_date_month" IS NULL OR ("birth_date_month" >= 1 AND "birth_date_month" <= 12)),
	"birth_date_day"	INTEGER CHECK("birth_date_day" IS NULL OR ("birth_date_day" >= 1 AND "birth_date_day" <= 31)),
	"birth_date_circa"	INTEGER NOT NULL DEFAULT 0 CHECK("birth_date_circa" IN (0, 1)),
	"death_date_year"	INTEGER,
	"death_date_month"	INTEGER CHECK("death_date_month" IS NULL OR ("death_date_month" >= 1 AND "death_date_month" <= 12)),
	"death_date_day"	INTEGER CHECK("death_date_day" IS NULL OR ("death_date_day" >= 1 AND "death_date_day" <= 31)),
	"death_date_circa"	INTEGER NOT NULL DEFAULT 0 CHECK("death_date_circa" IN (0, 1)),
	"biography"	TEXT,
	"normalized_key"	TEXT,
	"confidence"	REAL NOT NULL DEFAULT 1.0 CHECK("confidence" >= 0.0 AND "confidence" <= 1.0),
	"source"	TEXT NOT NULL DEFAULT 'biblioteca',
	"verified"	INTEGER NOT NULL DEFAULT 0 CHECK("verified" IN (0, 1)),
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE person_place_types (
    "id"         INTEGER PRIMARY KEY AUTOINCREMENT,
    "key"        TEXT NOT NULL UNIQUE,
    "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE person_place_type_translations (
    place_type_id INTEGER NOT NULL REFERENCES person_place_types(id),
    language_code TEXT NOT NULL,
    label         TEXT NOT NULL,
    PRIMARY KEY (place_type_id, language_code)
);

CREATE TABLE person_places (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    person_id       INTEGER NOT NULL REFERENCES people(id),
    place_type_id   INTEGER NOT NULL REFERENCES person_place_types(id),
    place_name      TEXT NOT NULL,
    region          TEXT,
    country         TEXT,
    country_historical TEXT,
    notes           TEXT
);

CREATE TABLE IF NOT EXISTS person_language_roles (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    code       TEXT NOT NULL UNIQUE,  -- 'native', 'writing', 'fluent', 'reading'
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS person_language_role_translations (
    role_id     INTEGER NOT NULL REFERENCES person_language_roles(id),
    language_code TEXT NOT NULL,
    label       TEXT NOT NULL,
    PRIMARY KEY (role_id, language_code)
);

CREATE TABLE IF NOT EXISTS content_language_roles (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    code       TEXT NOT NULL UNIQUE,  -- 'native', 'writing', 'fluent', 'reading'
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS content_language_role_translations (
    role_id     INTEGER NOT NULL REFERENCES content_language_roles(id),
    language_code TEXT NOT NULL,
    label       TEXT NOT NULL,
    PRIMARY KEY (role_id, language_code)
);

CREATE TABLE IF NOT EXISTS book_language_roles (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    code       TEXT NOT NULL UNIQUE,  -- 'native', 'writing', 'fluent', 'reading'
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS book_language_role_translations (
    role_id     INTEGER NOT NULL REFERENCES book_language_roles(id),
    language_code TEXT NOT NULL,
    label       TEXT NOT NULL,
    PRIMARY KEY (role_id, language_code)
);

CREATE TABLE IF NOT EXISTS person_languages (
    person_id   INTEGER NOT NULL REFERENCES people(id),
    language_id INTEGER NOT NULL REFERENCES languages(id),
    role_id     INTEGER NOT NULL REFERENCES person_language_roles(id),
    PRIMARY KEY (person_id, language_id, role_id)
);

CREATE TABLE IF NOT EXISTS content_languages (
    content_id   INTEGER NOT NULL REFERENCES contents(id),
    language_id INTEGER NOT NULL REFERENCES languages(id),
    role_id     INTEGER NOT NULL REFERENCES content_language_roles(id),
    PRIMARY KEY (content_id, language_id, role_id)
);

CREATE TABLE IF NOT EXISTS book_languages (
    book_id   INTEGER NOT NULL REFERENCES books(id),
    language_id INTEGER NOT NULL REFERENCES languages(id),
    role_id     INTEGER NOT NULL REFERENCES book_language_roles(id),
    PRIMARY KEY (book_id, language_id, role_id)
);

CREATE TABLE IF NOT EXISTS "aliases" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL,
	"person_id"	INTEGER NOT NULL,
	"alias_normalized"	TEXT,
	"confidence"	REAL NOT NULL DEFAULT 0.9 CHECK("confidence" >= 0.0 AND "confidence" <= 1.0),
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("person_id") REFERENCES "people"("id") ON DELETE CASCADE,
	UNIQUE("person_id","name")
);
CREATE TABLE IF NOT EXISTS "ml_data" (
	"id"	INTEGER,
	"data_type"	TEXT NOT NULL UNIQUE,
	"data_json"	TEXT NOT NULL,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS "books" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL,
	"original_title"	TEXT,
	"publisher_id"	INTEGER,
	"format_id"	INTEGER,
	"series_id"	INTEGER,
	"series_index"	INTEGER CHECK("series_index" > 0),
	"publication_date_year"	INTEGER,
	"publication_date_month"	INTEGER CHECK("publication_date_month" IS NULL OR ("publication_date_month" >= 1 AND "publication_date_month" <= 12)),
	"publication_date_day"	INTEGER CHECK("publication_date_day" IS NULL OR ("publication_date_day" >= 1 AND "publication_date_day" <= 31)),
	"publication_date_circa"	INTEGER NOT NULL DEFAULT 0 CHECK("publication_date_circa" IN (0, 1)),
	"last_modified_date"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"isbn"	TEXT,
	"notes"	TEXT,
	"has_cover"	INTEGER NOT NULL DEFAULT 0 CHECK("has_cover" IN (0, 1)),
	"has_paper"	INTEGER NOT NULL DEFAULT 0 CHECK("has_paper" IN (0, 1)),
	"file_link"	TEXT UNIQUE,
	"file_size"	INTEGER,
	"file_hash"	TEXT,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("format_id") REFERENCES "formats"("id") ON DELETE SET NULL,
	FOREIGN KEY("publisher_id") REFERENCES "publishers"("id") ON DELETE SET NULL,
	FOREIGN KEY("series_id") REFERENCES "series"("id") ON DELETE SET NULL
);
CREATE TABLE IF NOT EXISTS "contents" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL,
	"original_title"	TEXT,
	"type_id"	INTEGER,
	"genre_id"	INTEGER,
	"publication_date_year"	INTEGER,
	"publication_date_month"	INTEGER CHECK("publication_date_month" IS NULL OR ("publication_date_month" >= 1 AND "publication_date_month" <= 12)),
	"publication_date_day"	INTEGER CHECK("publication_date_day" IS NULL OR ("publication_date_day" >= 1 AND "publication_date_day" <= 31)),
	"publication_date_circa"	INTEGER NOT NULL DEFAULT 0 CHECK("publication_date_circa" IN (0, 1)),
	"notes"	TEXT,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("type_id") REFERENCES "types"("id") ON DELETE SET NULL,
	FOREIGN KEY("genre_id") REFERENCES "genres"("id") ON DELETE SET NULL
);
CREATE TABLE IF NOT EXISTS "x_books_contents" (
	"book_id"	INTEGER NOT NULL,
	"content_id"	INTEGER NOT NULL,
	PRIMARY KEY("book_id","content_id"),
	FOREIGN KEY("content_id") REFERENCES "contents"("id") ON DELETE CASCADE,
	FOREIGN KEY("book_id") REFERENCES "books"("id") ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS "x_books_people_roles" (
	"book_id"	INTEGER NOT NULL,
	"person_id"	INTEGER NOT NULL,
	"role_id"	INTEGER NOT NULL,
	PRIMARY KEY("book_id","person_id","role_id"),
	FOREIGN KEY("book_id") REFERENCES "books"("id") ON DELETE CASCADE,
	FOREIGN KEY("person_id") REFERENCES "people"("id") ON DELETE CASCADE,
	FOREIGN KEY("role_id") REFERENCES "roles"("id") ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS "x_books_tags" (
	"book_id"	INTEGER NOT NULL,
	"tag_id"	INTEGER NOT NULL,
	PRIMARY KEY("book_id","tag_id"),
	FOREIGN KEY("book_id") REFERENCES "books"("id") ON DELETE CASCADE,
	FOREIGN KEY("tag_id") REFERENCES "tags"("id") ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS "x_contents_people_roles" (
	"content_id"	INTEGER NOT NULL,
	"person_id"	INTEGER NOT NULL,
	"role_id"	INTEGER NOT NULL,
	PRIMARY KEY("content_id","person_id","role_id"),
	FOREIGN KEY("content_id") REFERENCES "contents"("id") ON DELETE CASCADE,
	FOREIGN KEY("role_id") REFERENCES "roles"("id") ON DELETE CASCADE,
	FOREIGN KEY("person_id") REFERENCES "people"("id") ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS "x_contents_tags" (
	"content_id"	INTEGER NOT NULL,
	"tag_id"	INTEGER NOT NULL,
	PRIMARY KEY("content_id","tag_id"),
	FOREIGN KEY("content_id") REFERENCES "contents"("id") ON DELETE CASCADE,
	FOREIGN KEY("tag_id") REFERENCES "tags"("id") ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS "pending_metadata_sync" (
	"id"	INTEGER,
	"book_id"	INTEGER NOT NULL,
	"reason"	TEXT NOT NULL,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("book_id") REFERENCES "books"("id") ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS "metadata" (
	"version"		TEXT NOT NULL,
	"updated_at"  	INTEGER NOT NULL,
	"created_at"	INTEGER NOT NULL,
	PRIMARY KEY("version")
);
CREATE INDEX IF NOT EXISTS "idx_people_name_search" ON "people" (
	"name" COLLATE NOCASE
);
CREATE INDEX IF NOT EXISTS "idx_series_name_search" ON "series" (
	"name" COLLATE NOCASE
);
CREATE INDEX IF NOT EXISTS "idx_publishers_name_search" ON "publishers" (
	"name" COLLATE NOCASE
);
CREATE INDEX IF NOT EXISTS "idx_tags_name_search" ON "tags" (
	"name" COLLATE NOCASE
);
CREATE INDEX IF NOT EXISTS "idx_genres_name_search" ON "genres" (
	"key" COLLATE NOCASE
);
CREATE INDEX IF NOT EXISTS "idx_people_dates" ON "people" (
	"birth_date_year",
	"death_date_year"
);
CREATE INDEX IF NOT EXISTS "idx_people_normalized_search" ON "people" (
	"normalized_key" COLLATE NOCASE
) WHERE "normalized_key" IS NOT NULL;
CREATE INDEX IF NOT EXISTS "idx_aliases_normalized_search" ON "aliases" (
	"alias_normalized" COLLATE NOCASE
) WHERE "alias_normalized" IS NOT NULL;
CREATE INDEX IF NOT EXISTS "idx_aliases_person_lookup" ON "aliases" (
	"person_id",
	"name"
);
CREATE INDEX IF NOT EXISTS "idx_audit_log_lookup" ON "audit_log" (
	"table_name",
	"record_id",
	"timestamp"
);
CREATE INDEX IF NOT EXISTS "idx_audit_log_timestamp" ON "audit_log" (
	"timestamp"
);
CREATE INDEX IF NOT EXISTS "idx_stats_cache_key" ON "stats_cache" (
	"cache_key"
);
CREATE INDEX IF NOT EXISTS "idx_stats_cache_expires" ON "stats_cache" (
	"expires_at"
);
CREATE INDEX IF NOT EXISTS "idx_languages_codes" ON "languages" (
	"iso_code_2char",
	"iso_code_3char"
);
CREATE INDEX IF NOT EXISTS "idx_books_name_search" ON "books" (
	"name" COLLATE NOCASE
);
CREATE INDEX IF NOT EXISTS "idx_contents_name_search" ON "contents" (
	"name" COLLATE NOCASE
);
CREATE INDEX IF NOT EXISTS "idx_books_search_optimized" ON "books" (
	"name",
	"publication_date_year",
	"series_id"
);
CREATE INDEX IF NOT EXISTS "idx_contents_search_optimized" ON "contents" (
	"name",
	"type_id",
	"publication_date_year"
);
CREATE INDEX IF NOT EXISTS "idx_contents_genre_lookup" ON "contents" (
	"genre_id"
);
CREATE INDEX IF NOT EXISTS "idx_books_series_lookup" ON "books" (
	"series_id",
	"series_index"
);
CREATE INDEX IF NOT EXISTS "idx_books_metadata" ON "books" (
	"publisher_id",
	"format_id",
	"series_id"
);
CREATE INDEX IF NOT EXISTS "idx_books_file_info" ON "books" (
	"file_link",
	"file_size",
	"file_hash"
) WHERE "file_link" IS NOT NULL;
CREATE INDEX IF NOT EXISTS "idx_books_dates_combined" ON "books" (
	"publication_date_year",
	"created_at",
	"last_modified_date"
);
CREATE INDEX IF NOT EXISTS "idx_contents_dates" ON "contents" (
	"publication_date_year",
	"created_at"
);
CREATE INDEX IF NOT EXISTS "idx_books_people_roles_person_role" ON "x_books_people_roles" (
	"person_id",
	"role_id"
);
CREATE INDEX IF NOT EXISTS "idx_books_people_roles_book_lookup" ON "x_books_people_roles" (
	"book_id",
	"person_id"
);
CREATE INDEX IF NOT EXISTS "idx_contents_people_roles_person_role" ON "x_contents_people_roles" (
	"person_id",
	"role_id"
);
CREATE INDEX IF NOT EXISTS "idx_contents_people_roles_content_lookup" ON "x_contents_people_roles" (
	"content_id",
	"person_id"
);
CREATE INDEX IF NOT EXISTS "idx_books_contents_junction" ON "x_books_contents" (
	"book_id",
	"content_id"
);
CREATE INDEX IF NOT EXISTS "idx_books_tags_lookup" ON "x_books_tags" (
	"book_id",
	"tag_id"
);
CREATE INDEX IF NOT EXISTS "idx_contents_tags_lookup" ON "x_contents_tags" (
	"content_id",
	"tag_id"
);
CREATE INDEX IF NOT EXISTS "idx_pending_sync_book_lookup" ON "pending_metadata_sync" (
	"book_id"
);
CREATE UNIQUE INDEX IF NOT EXISTS "idx_books_file_hash_unique" ON "books" (
	"file_hash"
) WHERE "file_hash" IS NOT NULL;
CREATE INDEX IF NOT EXISTS "idx_books_isbn" ON "books" (
	"isbn"
) WHERE "isbn" IS NOT NULL;
CREATE UNIQUE INDEX IF NOT EXISTS "idx_publishers_name_unique" ON "publishers" (
	LOWER(TRIM("name"))
);
CREATE UNIQUE INDEX IF NOT EXISTS "idx_series_name_unique" ON "series" (
	LOWER(TRIM("name"))
);
CREATE UNIQUE INDEX IF NOT EXISTS "idx_tags_name_unique" ON "tags" (
	LOWER(TRIM("name"))
);
CREATE INDEX IF NOT EXISTS "idx_books_bulk_filters_covering" ON "books" (
	"publisher_id",
	"format_id",
	"publication_date_year",
	"series_id",
	"name",
	"id",
	"created_at"
);
CREATE INDEX IF NOT EXISTS "idx_books_people_composite" ON "x_books_people_roles" (
	"person_id",
	"role_id",
	"book_id"
);
CREATE INDEX IF NOT EXISTS "idx_contents_type_lookup" ON "contents" (
	"type_id",
	"publication_date_year",
	"id"
);
CREATE INDEX IF NOT EXISTS "idx_pending_sync_composite" ON "pending_metadata_sync" (
	"book_id",
	"created_at",
	"reason"
);



-- 1. Normalizzazione nome persona (nessuna ricorsione, ma aggiungo WHEN più robusto)
CREATE TRIGGER normalize_person_name
    AFTER INSERT ON people
    FOR EACH ROW
    WHEN NEW.normalized_key IS NULL
BEGIN
    UPDATE people
    SET normalized_key = LOWER(TRIM(NEW.name))
    WHERE id = NEW.id;
END;

-- 2. Timestamp people  ← CORRETTO (aggiunta guardia WHEN)
CREATE TRIGGER update_people_timestamp
    AFTER UPDATE ON people
    FOR EACH ROW
    WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE people
    SET updated_at = strftime('%s', 'now')
    WHERE id = NEW.id;
END;

-- 3. Timestamp publishers  ← CORRETTO
CREATE TRIGGER update_publishers_timestamp
    AFTER UPDATE ON publishers
    FOR EACH ROW
    WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE publishers
    SET updated_at = strftime('%s', 'now')
    WHERE id = NEW.id;
END;

-- 4. Normalizzazione alias (nessuna ricorsione)
CREATE TRIGGER normalize_alias_name
    AFTER INSERT ON aliases
    FOR EACH ROW
    WHEN NEW.alias_normalized IS NULL
BEGIN
    UPDATE aliases
    SET alias_normalized = LOWER(TRIM(NEW.name))
    WHERE id = NEW.id;
END;

-- 5. Timestamp system_config  ← CORRETTO (PK è key, non id)
CREATE TRIGGER update_config_timestamp
    AFTER UPDATE ON system_config
    FOR EACH ROW
    WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE system_config
    SET updated_at = strftime('%s', 'now')
    WHERE key = NEW.key;
END;

-- 6. Audit insert (nessuna ricorsione, invariato)
CREATE TRIGGER audit_people_insert
    AFTER INSERT ON people
    FOR EACH ROW
BEGIN
    INSERT INTO audit_log (table_name, record_id, operation, new_values)
    VALUES ('people', NEW.id, 'INSERT',
            json_object('name', NEW.name, 'verified', NEW.verified));
END;

-- 7. Audit update (nessuna ricorsione, invariato)
CREATE TRIGGER audit_people_update
    AFTER UPDATE ON people
    FOR EACH ROW
BEGIN
    INSERT INTO audit_log (table_name, record_id, operation, old_values, new_values)
    VALUES ('people', NEW.id, 'UPDATE',
            json_object('name', OLD.name, 'verified', OLD.verified),
            json_object('name', NEW.name, 'verified', NEW.verified));
END;

-- 8. Audit delete (nessuna ricorsione, invariato)
CREATE TRIGGER audit_people_delete
    AFTER DELETE ON people
    FOR EACH ROW
BEGIN
    INSERT INTO audit_log (table_name, record_id, operation, old_values)
    VALUES ('people', OLD.id, 'DELETE',
            json_object('name', OLD.name, 'verified', OLD.verified));
END;

-- 10. Timestamp series  ← CORRETTO
CREATE TRIGGER update_series_timestamp
    AFTER UPDATE ON series
    FOR EACH ROW
    WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE series
    SET updated_at = strftime('%s', 'now')
    WHERE id = NEW.id;
END;

-- 11. Timestamp tags  ← CORRETTO
CREATE TRIGGER update_tags_timestamp
    AFTER UPDATE ON tags
    FOR EACH ROW
    WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE tags
    SET updated_at = strftime('%s', 'now')
    WHERE id = NEW.id;
END;

-- 12. Timestamp aliases  ← CORRETTO
CREATE TRIGGER update_aliases_timestamp
    AFTER UPDATE ON aliases
    FOR EACH ROW
    WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE aliases
    SET updated_at = strftime('%s', 'now')
    WHERE id = NEW.id;
END;

-- 13. last_modified_date books (già corretto nell'originale, invariato)
CREATE TRIGGER update_books_modified_date
    AFTER UPDATE ON books
    FOR EACH ROW
    WHEN NEW.last_modified_date = OLD.last_modified_date
BEGIN
    UPDATE books
    SET last_modified_date = strftime('%s', 'now')
    WHERE id = NEW.id;
END;

-- 14. Cleanup audit_log (nessuna ricorsione, invariato)
CREATE TRIGGER cleanup_old_audit_logs
    AFTER INSERT ON audit_log
    WHEN (SELECT COUNT(*) FROM audit_log) > 10000
BEGIN
    DELETE FROM audit_log
    WHERE timestamp < strftime('%s', 'now', '-90 days')
    AND id NOT IN (
        SELECT id FROM audit_log
        ORDER BY timestamp DESC
        LIMIT 10000
    );
END;

-- 15. Cleanup stats_cache (nessuna ricorsione, invariato)
CREATE TRIGGER cleanup_expired_cache
    AFTER INSERT ON stats_cache
BEGIN
    DELETE FROM stats_cache WHERE expires_at < strftime('%s', 'now');
END;

-- languages (sostituisce update_languages_timestamp, già droppato al step 16)
CREATE TRIGGER update_languages_timestamp
    AFTER UPDATE ON languages
    FOR EACH ROW
    WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE languages SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

-- person_language_roles
CREATE TRIGGER update_person_language_roles_timestamp
    AFTER UPDATE ON person_language_roles
    FOR EACH ROW
    WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE person_language_roles SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

-- person_place_types
CREATE TRIGGER update_person_place_types_timestamp
    AFTER UPDATE ON person_place_types
    FOR EACH ROW
    WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE person_place_types SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

-- content_language_roles
CREATE TRIGGER update_content_language_roles_timestamp
    AFTER UPDATE ON content_language_roles
    FOR EACH ROW
    WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE content_language_roles SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

-- book_language_roles
CREATE TRIGGER update_book_language_roles_timestamp
    AFTER UPDATE ON book_language_roles
    FOR EACH ROW
    WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE book_language_roles SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

-- person_places
CREATE TRIGGER update_person_places_timestamp
    AFTER UPDATE ON person_places
    FOR EACH ROW
    WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE person_places SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

CREATE TABLE IF NOT EXISTS "page_fields" (
    "id"            INTEGER PRIMARY KEY AUTOINCREMENT,
    "page"          TEXT NOT NULL CHECK("page" IN ('book_page', 'content_page', 'people_page')),
    "field_key"     TEXT NOT NULL,
    "data_kind"     TEXT NOT NULL CHECK("data_kind" IN ('string', 'quantity', 'date', 'enum', 'person')),
    "sort_order"    INTEGER NOT NULL DEFAULT 0,
    "enum_values"   TEXT,
    "relation_type" TEXT NOT NULL DEFAULT 'direct' CHECK("relation_type" IN ('direct', 'fk', 'junction')),
    "target_table"  TEXT,
    "target_field"  TEXT,
    "created_at"    INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    "updated_at"      INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    UNIQUE("page", "field_key")
);
COMMIT;

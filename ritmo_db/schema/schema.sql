BEGIN TRANSACTION;

-- ============================================================
-- TABELLE DI SISTEMA (s_)
-- ============================================================

CREATE TABLE IF NOT EXISTS "s_system_config" (
	"key"	TEXT,
	"value"	TEXT,
	"description"	TEXT,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("key")
);

CREATE TABLE IF NOT EXISTS "s_audit_log" (
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

CREATE TABLE IF NOT EXISTS "s_stats_cache" (
	"id"	INTEGER,
	"cache_key"	TEXT NOT NULL UNIQUE,
	"cache_value"	TEXT NOT NULL,
	"expires_at"	INTEGER NOT NULL,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS "s_ml_data" (
	"id"	INTEGER,
	"data_type"	TEXT NOT NULL UNIQUE,
	"data_json"	TEXT NOT NULL,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS "s_metadata" (
	"version"	TEXT NOT NULL,
	"updated_at"	INTEGER NOT NULL,
	"created_at"	INTEGER NOT NULL,
	PRIMARY KEY("version")
);

CREATE TABLE IF NOT EXISTS "s_pending_metadata_sync" (
	"id"	INTEGER,
	"book_id"	INTEGER NOT NULL,
	"reason"	TEXT NOT NULL,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("book_id") REFERENCES "d_books"("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "s_page_fields" (
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
	"updated_at"    INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	UNIQUE("page", "field_key")
);

CREATE TABLE IF NOT EXISTS s_filter_sets (
	id          INTEGER PRIMARY KEY AUTOINCREMENT,
	name        TEXT NOT NULL,
	active      INTEGER NOT NULL DEFAULT 0,
	operator    TEXT NOT NULL DEFAULT 'AND',
	created_at  INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	updated_at  INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS s_filter_conditions (
	id              INTEGER PRIMARY KEY AUTOINCREMENT,
	filter_set_id   INTEGER NOT NULL REFERENCES s_filter_sets(id),
	field           TEXT NOT NULL,
	operator        TEXT NOT NULL,
	filter_values   TEXT NOT NULL  -- JSON array di FilterValue
);

-- Lookup di sistema con i18n

CREATE TABLE IF NOT EXISTS s_place_types (
	id          INTEGER PRIMARY KEY AUTOINCREMENT,
	key         TEXT NOT NULL UNIQUE,
	created_at  INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	updated_at  INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS s_place_type_translations (
	place_type_id   INTEGER NOT NULL REFERENCES s_place_types(id),
	language_code   TEXT NOT NULL,
	label           TEXT NOT NULL,
	PRIMARY KEY (place_type_id, language_code)
);

CREATE TABLE IF NOT EXISTS "s_role_translations" (
	"id" INTEGER PRIMARY KEY AUTOINCREMENT,
	"role_id" INTEGER NOT NULL REFERENCES "d_roles"("id") ON DELETE CASCADE,
	"language_code" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	UNIQUE("role_id", "language_code")
);

CREATE TABLE IF NOT EXISTS "s_format_translations" (
	"id" INTEGER PRIMARY KEY AUTOINCREMENT,
	"format_id" INTEGER NOT NULL REFERENCES "d_formats"("id") ON DELETE CASCADE,
	"language_code" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	UNIQUE("format_id", "language_code")
);

CREATE TABLE IF NOT EXISTS "s_genre_translations" (
	"id" INTEGER PRIMARY KEY AUTOINCREMENT,
	"genre_id" INTEGER NOT NULL REFERENCES "d_genres"("id") ON DELETE CASCADE,
	"language_code" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	UNIQUE("genre_id", "language_code")
);

CREATE TABLE IF NOT EXISTS "s_type_translations" (
	"id" INTEGER PRIMARY KEY AUTOINCREMENT,
	"type_id" INTEGER NOT NULL REFERENCES "d_types"("id") ON DELETE CASCADE,
	"language_code" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	UNIQUE("type_id", "language_code")
);

CREATE TABLE IF NOT EXISTS s_person_language_roles (
	id         INTEGER PRIMARY KEY AUTOINCREMENT,
	code       TEXT NOT NULL UNIQUE,  -- 'native', 'writing', 'fluent', 'reading'
	created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS s_person_language_role_translations (
	role_id       INTEGER NOT NULL REFERENCES s_person_language_roles(id),
	language_code TEXT NOT NULL,
	label         TEXT NOT NULL,
	PRIMARY KEY (role_id, language_code)
);

CREATE TABLE IF NOT EXISTS s_content_language_roles (
	id         INTEGER PRIMARY KEY AUTOINCREMENT,
	code       TEXT NOT NULL UNIQUE,  -- 'original', 'translation'
	created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS s_content_language_role_translations (
	role_id       INTEGER NOT NULL REFERENCES s_content_language_roles(id),
	language_code TEXT NOT NULL,
	label         TEXT NOT NULL,
	PRIMARY KEY (role_id, language_code)
);

CREATE TABLE IF NOT EXISTS s_book_language_roles (
	id         INTEGER PRIMARY KEY AUTOINCREMENT,
	code       TEXT NOT NULL UNIQUE,  -- 'original', 'translation'
	created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS s_book_language_role_translations (
	role_id       INTEGER NOT NULL REFERENCES s_book_language_roles(id),
	language_code TEXT NOT NULL,
	label         TEXT NOT NULL,
	PRIMARY KEY (role_id, language_code)
);

-- ============================================================
-- TABELLE DI DOMINIO (d_)
-- ============================================================

CREATE TABLE IF NOT EXISTS "d_formats" (
	"id"	INTEGER,
	"key"	TEXT NOT NULL UNIQUE,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS "d_publishers" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL,
	"country"	TEXT,
	"website"	TEXT,
	"notes"	TEXT,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS "d_series" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL UNIQUE,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS "d_roles" (
	"id"	INTEGER,
	"key"	TEXT NOT NULL UNIQUE,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS "d_tags" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL UNIQUE,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS "d_genres" (
	"id"	INTEGER,
	"key"	TEXT NOT NULL UNIQUE,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS "d_types" (
	"id"	INTEGER,
	"key"	TEXT NOT NULL UNIQUE,
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS "d_languages" (
	id             INTEGER,
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

CREATE TABLE IF NOT EXISTS "d_people" (
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

CREATE TABLE IF NOT EXISTS d_places (
	id          INTEGER PRIMARY KEY AUTOINCREMENT,
	continent   TEXT,
	country     TEXT,
	city        TEXT,
	circa       INTEGER NOT NULL DEFAULT 0,
	disputed    INTEGER NOT NULL DEFAULT 0,
	created_at  INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	updated_at  INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS "d_aliases" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL,
	"person_id"	INTEGER NOT NULL,
	"alias_normalized"	TEXT,
	"confidence"	REAL NOT NULL DEFAULT 0.9 CHECK("confidence" >= 0.0 AND "confidence" <= 1.0),
	"created_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"updated_at"	INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("person_id") REFERENCES "d_people"("id") ON DELETE CASCADE,
	UNIQUE("person_id","name")
);

CREATE TABLE IF NOT EXISTS "d_books" (
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
	FOREIGN KEY("format_id") REFERENCES "d_formats"("id") ON DELETE SET NULL,
	FOREIGN KEY("publisher_id") REFERENCES "d_publishers"("id") ON DELETE SET NULL,
	FOREIGN KEY("series_id") REFERENCES "d_series"("id") ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS "d_contents" (
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
	FOREIGN KEY("type_id") REFERENCES "d_types"("id") ON DELETE SET NULL,
	FOREIGN KEY("genre_id") REFERENCES "d_genres"("id") ON DELETE SET NULL
);

-- ============================================================
-- TABELLE DI RELAZIONE (x_)
-- ============================================================

CREATE TABLE IF NOT EXISTS "x_books_contents" (
	"book_id"	INTEGER NOT NULL,
	"content_id"	INTEGER NOT NULL,
	PRIMARY KEY("book_id","content_id"),
	FOREIGN KEY("content_id") REFERENCES "d_contents"("id") ON DELETE CASCADE,
	FOREIGN KEY("book_id") REFERENCES "d_books"("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "x_books_people_roles" (
	"book_id"	INTEGER NOT NULL,
	"person_id"	INTEGER NOT NULL,
	"role_id"	INTEGER NOT NULL,
	PRIMARY KEY("book_id","person_id","role_id"),
	FOREIGN KEY("book_id") REFERENCES "d_books"("id") ON DELETE CASCADE,
	FOREIGN KEY("person_id") REFERENCES "d_people"("id") ON DELETE CASCADE,
	FOREIGN KEY("role_id") REFERENCES "d_roles"("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "x_books_tags" (
	"book_id"	INTEGER NOT NULL,
	"tag_id"	INTEGER NOT NULL,
	PRIMARY KEY("book_id","tag_id"),
	FOREIGN KEY("book_id") REFERENCES "d_books"("id") ON DELETE CASCADE,
	FOREIGN KEY("tag_id") REFERENCES "d_tags"("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "x_contents_people_roles" (
	"content_id"	INTEGER NOT NULL,
	"person_id"	INTEGER NOT NULL,
	"role_id"	INTEGER NOT NULL,
	PRIMARY KEY("content_id","person_id","role_id"),
	FOREIGN KEY("content_id") REFERENCES "d_contents"("id") ON DELETE CASCADE,
	FOREIGN KEY("role_id") REFERENCES "d_roles"("id") ON DELETE CASCADE,
	FOREIGN KEY("person_id") REFERENCES "d_people"("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "x_contents_tags" (
	"content_id"	INTEGER NOT NULL,
	"tag_id"	INTEGER NOT NULL,
	PRIMARY KEY("content_id","tag_id"),
	FOREIGN KEY("content_id") REFERENCES "d_contents"("id") ON DELETE CASCADE,
	FOREIGN KEY("tag_id") REFERENCES "d_tags"("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS x_person_places (
	person_id       INTEGER NOT NULL REFERENCES d_people(id),
	place_id        INTEGER NOT NULL REFERENCES d_places(id),
	place_type_id   INTEGER NOT NULL REFERENCES s_place_types(id),
	PRIMARY KEY (person_id, place_id, place_type_id)
);

CREATE TABLE IF NOT EXISTS x_publisher_places (
	publisher_id    INTEGER NOT NULL REFERENCES d_publishers(id),
	place_id        INTEGER NOT NULL REFERENCES d_places(id),
	place_type_id   INTEGER NOT NULL REFERENCES s_place_types(id),
	PRIMARY KEY (publisher_id, place_id, place_type_id)
);

CREATE TABLE IF NOT EXISTS x_person_languages (
	person_id   INTEGER NOT NULL REFERENCES d_people(id),
	language_id INTEGER NOT NULL REFERENCES d_languages(id),
	role_id     INTEGER NOT NULL REFERENCES s_person_language_roles(id),
	PRIMARY KEY (person_id, language_id, role_id)
);

CREATE TABLE IF NOT EXISTS x_content_languages (
	content_id  INTEGER NOT NULL REFERENCES d_contents(id),
	language_id INTEGER NOT NULL REFERENCES d_languages(id),
	role_id     INTEGER NOT NULL REFERENCES s_content_language_roles(id),
	PRIMARY KEY (content_id, language_id, role_id)
);

CREATE TABLE IF NOT EXISTS x_book_languages (
	book_id     INTEGER NOT NULL REFERENCES d_books(id),
	language_id INTEGER NOT NULL REFERENCES d_languages(id),
	role_id     INTEGER NOT NULL REFERENCES s_book_language_roles(id),
	PRIMARY KEY (book_id, language_id, role_id)
);

-- ============================================================
-- INDICI
-- ============================================================

CREATE INDEX IF NOT EXISTS "idx_people_name_search" ON "d_people" (
	"name" COLLATE NOCASE
);
CREATE INDEX IF NOT EXISTS "idx_series_name_search" ON "d_series" (
	"name" COLLATE NOCASE
);
CREATE INDEX IF NOT EXISTS "idx_publishers_name_search" ON "d_publishers" (
	"name" COLLATE NOCASE
);
CREATE INDEX IF NOT EXISTS "idx_tags_name_search" ON "d_tags" (
	"name" COLLATE NOCASE
);
CREATE INDEX IF NOT EXISTS "idx_genres_name_search" ON "d_genres" (
	"key" COLLATE NOCASE
);
CREATE INDEX IF NOT EXISTS "idx_people_dates" ON "d_people" (
	"birth_date_year",
	"death_date_year"
);
CREATE INDEX IF NOT EXISTS "idx_people_normalized_search" ON "d_people" (
	"normalized_key" COLLATE NOCASE
) WHERE "normalized_key" IS NOT NULL;
CREATE INDEX IF NOT EXISTS "idx_aliases_normalized_search" ON "d_aliases" (
	"alias_normalized" COLLATE NOCASE
) WHERE "alias_normalized" IS NOT NULL;
CREATE INDEX IF NOT EXISTS "idx_aliases_person_lookup" ON "d_aliases" (
	"person_id",
	"name"
);
CREATE INDEX IF NOT EXISTS "idx_audit_log_lookup" ON "s_audit_log" (
	"table_name",
	"record_id",
	"timestamp"
);
CREATE INDEX IF NOT EXISTS "idx_audit_log_timestamp" ON "s_audit_log" (
	"timestamp"
);
CREATE INDEX IF NOT EXISTS "idx_stats_cache_key" ON "s_stats_cache" (
	"cache_key"
);
CREATE INDEX IF NOT EXISTS "idx_stats_cache_expires" ON "s_stats_cache" (
	"expires_at"
);
CREATE INDEX IF NOT EXISTS "idx_languages_codes" ON "d_languages" (
	"iso_code_2char",
	"iso_code_3char"
);
CREATE INDEX IF NOT EXISTS "idx_books_name_search" ON "d_books" (
	"name" COLLATE NOCASE
);
CREATE INDEX IF NOT EXISTS "idx_contents_name_search" ON "d_contents" (
	"name" COLLATE NOCASE
);
CREATE INDEX IF NOT EXISTS "idx_books_search_optimized" ON "d_books" (
	"name",
	"publication_date_year",
	"series_id"
);
CREATE INDEX IF NOT EXISTS "idx_contents_search_optimized" ON "d_contents" (
	"name",
	"type_id",
	"publication_date_year"
);
CREATE INDEX IF NOT EXISTS "idx_contents_genre_lookup" ON "d_contents" (
	"genre_id"
);
CREATE INDEX IF NOT EXISTS "idx_books_series_lookup" ON "d_books" (
	"series_id",
	"series_index"
);
CREATE INDEX IF NOT EXISTS "idx_books_metadata" ON "d_books" (
	"publisher_id",
	"format_id",
	"series_id"
);
CREATE INDEX IF NOT EXISTS "idx_books_file_info" ON "d_books" (
	"file_link",
	"file_size",
	"file_hash"
) WHERE "file_link" IS NOT NULL;
CREATE INDEX IF NOT EXISTS "idx_books_dates_combined" ON "d_books" (
	"publication_date_year",
	"created_at",
	"last_modified_date"
);
CREATE INDEX IF NOT EXISTS "idx_contents_dates" ON "d_contents" (
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
CREATE INDEX IF NOT EXISTS "idx_pending_sync_book_lookup" ON "s_pending_metadata_sync" (
	"book_id"
);
CREATE UNIQUE INDEX IF NOT EXISTS "idx_books_file_hash_unique" ON "d_books" (
	"file_hash"
) WHERE "file_hash" IS NOT NULL;
CREATE INDEX IF NOT EXISTS "idx_books_isbn" ON "d_books" (
	"isbn"
) WHERE "isbn" IS NOT NULL;
CREATE UNIQUE INDEX IF NOT EXISTS "idx_publishers_name_unique" ON "d_publishers" (
	LOWER(TRIM("name"))
);
CREATE UNIQUE INDEX IF NOT EXISTS "idx_series_name_unique" ON "d_series" (
	LOWER(TRIM("name"))
);
CREATE UNIQUE INDEX IF NOT EXISTS "idx_tags_name_unique" ON "d_tags" (
	LOWER(TRIM("name"))
);
CREATE INDEX IF NOT EXISTS "idx_books_bulk_filters_covering" ON "d_books" (
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
CREATE INDEX IF NOT EXISTS "idx_contents_type_lookup" ON "d_contents" (
	"type_id",
	"publication_date_year",
	"id"
);
CREATE INDEX IF NOT EXISTS "idx_pending_sync_composite" ON "s_pending_metadata_sync" (
	"book_id",
	"created_at",
	"reason"
);

-- ============================================================
-- TRIGGER
-- ============================================================

-- 1. Normalizzazione nome persona
CREATE TRIGGER IF NOT EXISTS normalize_person_name
	AFTER INSERT ON d_people
	FOR EACH ROW
	WHEN NEW.normalized_key IS NULL
BEGIN
	UPDATE d_people
	SET normalized_key = LOWER(TRIM(NEW.name))
	WHERE id = NEW.id;
END;

-- 2. Timestamp d_people
CREATE TRIGGER IF NOT EXISTS update_people_timestamp
	AFTER UPDATE ON d_people
	FOR EACH ROW
	WHEN NEW.updated_at = OLD.updated_at
BEGIN
	UPDATE d_people
	SET updated_at = strftime('%s', 'now')
	WHERE id = NEW.id;
END;

-- 3. Timestamp d_publishers
CREATE TRIGGER IF NOT EXISTS update_publishers_timestamp
	AFTER UPDATE ON d_publishers
	FOR EACH ROW
	WHEN NEW.updated_at = OLD.updated_at
BEGIN
	UPDATE d_publishers
	SET updated_at = strftime('%s', 'now')
	WHERE id = NEW.id;
END;

-- 4. Normalizzazione alias
CREATE TRIGGER IF NOT EXISTS normalize_alias_name
	AFTER INSERT ON d_aliases
	FOR EACH ROW
	WHEN NEW.alias_normalized IS NULL
BEGIN
	UPDATE d_aliases
	SET alias_normalized = LOWER(TRIM(NEW.name))
	WHERE id = NEW.id;
END;

-- 5. Timestamp s_system_config
CREATE TRIGGER IF NOT EXISTS update_config_timestamp
	AFTER UPDATE ON s_system_config
	FOR EACH ROW
	WHEN NEW.updated_at = OLD.updated_at
BEGIN
	UPDATE s_system_config
	SET updated_at = strftime('%s', 'now')
	WHERE key = NEW.key;
END;

-- 6. Audit insert d_people
CREATE TRIGGER IF NOT EXISTS audit_people_insert
	AFTER INSERT ON d_people
	FOR EACH ROW
BEGIN
	INSERT OR IGNORE INTO s_audit_log (table_name, record_id, operation, new_values)
	VALUES ('d_people', NEW.id, 'INSERT',
	        json_object('name', NEW.name, 'verified', NEW.verified));
END;

-- 7. Audit update d_people
CREATE TRIGGER IF NOT EXISTS audit_people_update
	AFTER UPDATE ON d_people
	FOR EACH ROW
BEGIN
	INSERT OR IGNORE INTO s_audit_log (table_name, record_id, operation, old_values, new_values)
	VALUES ('d_people', NEW.id, 'UPDATE',
	        json_object('name', OLD.name, 'verified', OLD.verified),
	        json_object('name', NEW.name, 'verified', NEW.verified));
END;

-- 8. Audit delete d_people
CREATE TRIGGER IF NOT EXISTS audit_people_delete
	AFTER DELETE ON d_people
	FOR EACH ROW
BEGIN
	INSERT OR IGNORE INTO s_audit_log (table_name, record_id, operation, old_values)
	VALUES ('d_people', OLD.id, 'DELETE',
	        json_object('name', OLD.name, 'verified', OLD.verified));
END;

-- 10. Timestamp d_series
CREATE TRIGGER IF NOT EXISTS update_series_timestamp
	AFTER UPDATE ON d_series
	FOR EACH ROW
	WHEN NEW.updated_at = OLD.updated_at
BEGIN
	UPDATE d_series
	SET updated_at = strftime('%s', 'now')
	WHERE id = NEW.id;
END;

-- 11. Timestamp d_tags
CREATE TRIGGER IF NOT EXISTS update_tags_timestamp
	AFTER UPDATE ON d_tags
	FOR EACH ROW
	WHEN NEW.updated_at = OLD.updated_at
BEGIN
	UPDATE d_tags
	SET updated_at = strftime('%s', 'now')
	WHERE id = NEW.id;
END;

-- 12. Timestamp d_aliases
CREATE TRIGGER IF NOT EXISTS update_aliases_timestamp
	AFTER UPDATE ON d_aliases
	FOR EACH ROW
	WHEN NEW.updated_at = OLD.updated_at
BEGIN
	UPDATE d_aliases
	SET updated_at = strftime('%s', 'now')
	WHERE id = NEW.id;
END;

-- 13. last_modified_date d_books
CREATE TRIGGER IF NOT EXISTS update_books_modified_date
	AFTER UPDATE ON d_books
	FOR EACH ROW
	WHEN NEW.last_modified_date = OLD.last_modified_date
BEGIN
	UPDATE d_books
	SET last_modified_date = strftime('%s', 'now')
	WHERE id = NEW.id;
END;

-- 14. Cleanup s_audit_log
CREATE TRIGGER IF NOT EXISTS cleanup_old_audit_logs
	AFTER INSERT ON s_audit_log
	WHEN (SELECT COUNT(*) FROM s_audit_log) > 10000
BEGIN
	DELETE FROM s_audit_log
	WHERE timestamp < strftime('%s', 'now', '-90 days')
	AND id NOT IN (
		SELECT id FROM s_audit_log
		ORDER BY timestamp DESC
		LIMIT 10000
	);
END;

-- 15. Cleanup s_stats_cache
CREATE TRIGGER IF NOT EXISTS cleanup_expired_cache
	AFTER INSERT ON s_stats_cache
BEGIN
	DELETE FROM s_stats_cache WHERE expires_at < strftime('%s', 'now');
END;

-- 16. Timestamp d_languages
CREATE TRIGGER IF NOT EXISTS update_languages_timestamp
	AFTER UPDATE ON d_languages
	FOR EACH ROW
	WHEN NEW.updated_at = OLD.updated_at
BEGIN
	UPDATE d_languages SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

-- 17. Timestamp s_person_language_roles
CREATE TRIGGER IF NOT EXISTS update_person_language_roles_timestamp
	AFTER UPDATE ON s_person_language_roles
	FOR EACH ROW
	WHEN NEW.updated_at = OLD.updated_at
BEGIN
	UPDATE s_person_language_roles SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

-- 18. Timestamp s_place_types
CREATE TRIGGER IF NOT EXISTS update_s_place_types_timestamp
	AFTER UPDATE ON s_place_types
	FOR EACH ROW
	WHEN NEW.updated_at = OLD.updated_at
BEGIN
	UPDATE s_place_types SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

-- 19. Timestamp s_content_language_roles
CREATE TRIGGER IF NOT EXISTS update_content_language_roles_timestamp
	AFTER UPDATE ON s_content_language_roles
	FOR EACH ROW
	WHEN NEW.updated_at = OLD.updated_at
BEGIN
	UPDATE s_content_language_roles SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

-- 20. Timestamp s_book_language_roles
CREATE TRIGGER IF NOT EXISTS update_book_language_roles_timestamp
	AFTER UPDATE ON s_book_language_roles
	FOR EACH ROW
	WHEN NEW.updated_at = OLD.updated_at
BEGIN
	UPDATE s_book_language_roles SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

COMMIT;

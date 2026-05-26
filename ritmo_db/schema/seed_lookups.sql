-- Seed SQL — Lookup di sistema
-- Il valore 'other' viene sempre inserito per ultimo.

-- ============================================================
-- d_roles
-- ============================================================
INSERT OR IGNORE INTO d_roles (key, created_at, updated_at) VALUES
    ('author', strftime('%s', 'now'), strftime('%s', 'now')),
    ('translator', strftime('%s', 'now'), strftime('%s', 'now')),
    ('editor', strftime('%s', 'now'), strftime('%s', 'now')),
    ('cover_artist', strftime('%s', 'now'), strftime('%s', 'now')),
    ('illustrator', strftime('%s', 'now'), strftime('%s', 'now')),
    ('editorial_consultant', strftime('%s', 'now'), strftime('%s', 'now')),
    ('photographer', strftime('%s', 'now'), strftime('%s', 'now')),
    ('preface_writer', strftime('%s', 'now'), strftime('%s', 'now')),
    ('afterword_writer', strftime('%s', 'now'), strftime('%s', 'now')),
    ('commentator', strftime('%s', 'now'), strftime('%s', 'now')),
    ('other', strftime('%s', 'now'), strftime('%s', 'now'));  -- sempre per ultimo

INSERT OR IGNORE INTO s_role_translations (role_id, language_code, name) VALUES
    -- author
    ((SELECT id FROM d_roles WHERE key = 'author'), 'it', 'Autore'),
    ((SELECT id FROM d_roles WHERE key = 'author'), 'en', 'Author'),
    ((SELECT id FROM d_roles WHERE key = 'author'), 'fr', 'Auteur'),
    ((SELECT id FROM d_roles WHERE key = 'author'), 'de', 'Autor'),
    ((SELECT id FROM d_roles WHERE key = 'author'), 'es', 'Autor'),
    -- translator
    ((SELECT id FROM d_roles WHERE key = 'translator'), 'it', 'Traduttore'),
    ((SELECT id FROM d_roles WHERE key = 'translator'), 'en', 'Translator'),
    ((SELECT id FROM d_roles WHERE key = 'translator'), 'fr', 'Traducteur'),
    ((SELECT id FROM d_roles WHERE key = 'translator'), 'de', 'Übersetzer'),
    ((SELECT id FROM d_roles WHERE key = 'translator'), 'es', 'Traductor'),
    -- editor
    ((SELECT id FROM d_roles WHERE key = 'editor'), 'it', 'Curatore'),
    ((SELECT id FROM d_roles WHERE key = 'editor'), 'en', 'Editor'),
    ((SELECT id FROM d_roles WHERE key = 'editor'), 'fr', 'Éditeur'),
    ((SELECT id FROM d_roles WHERE key = 'editor'), 'de', 'Herausgeber'),
    ((SELECT id FROM d_roles WHERE key = 'editor'), 'es', 'Editor'),
    -- cover_artist
    ((SELECT id FROM d_roles WHERE key = 'cover_artist'), 'it', 'Cover artist'),
    ((SELECT id FROM d_roles WHERE key = 'cover_artist'), 'en', 'Cover artist'),
    ((SELECT id FROM d_roles WHERE key = 'cover_artist'), 'fr', 'Illustrateur de couverture'),
    ((SELECT id FROM d_roles WHERE key = 'cover_artist'), 'de', 'Cover-Künstler'),
    ((SELECT id FROM d_roles WHERE key = 'cover_artist'), 'es', 'Artista de portada'),
    -- illustrator
    ((SELECT id FROM d_roles WHERE key = 'illustrator'), 'it', 'Illustratore'),
    ((SELECT id FROM d_roles WHERE key = 'illustrator'), 'en', 'Illustrator'),
    ((SELECT id FROM d_roles WHERE key = 'illustrator'), 'fr', 'Illustrateur'),
    ((SELECT id FROM d_roles WHERE key = 'illustrator'), 'de', 'Illustrator'),
    ((SELECT id FROM d_roles WHERE key = 'illustrator'), 'es', 'Ilustrador'),
    -- editorial_consultant
    ((SELECT id FROM d_roles WHERE key = 'editorial_consultant'), 'it', 'Consulente editoriale'),
    ((SELECT id FROM d_roles WHERE key = 'editorial_consultant'), 'en', 'Editorial consultant'),
    ((SELECT id FROM d_roles WHERE key = 'editorial_consultant'), 'fr', 'Consultant éditorial'),
    ((SELECT id FROM d_roles WHERE key = 'editorial_consultant'), 'de', 'Redaktionsberater'),
    ((SELECT id FROM d_roles WHERE key = 'editorial_consultant'), 'es', 'Consultor editorial'),
    -- photographer
    ((SELECT id FROM d_roles WHERE key = 'photographer'), 'it', 'Fotografo'),
    ((SELECT id FROM d_roles WHERE key = 'photographer'), 'en', 'Photographer'),
    ((SELECT id FROM d_roles WHERE key = 'photographer'), 'fr', 'Photographe'),
    ((SELECT id FROM d_roles WHERE key = 'photographer'), 'de', 'Fotograf'),
    ((SELECT id FROM d_roles WHERE key = 'photographer'), 'es', 'Fotógrafo'),
    -- preface_writer
    ((SELECT id FROM d_roles WHERE key = 'preface_writer'), 'it', 'Prefatore'),
    ((SELECT id FROM d_roles WHERE key = 'preface_writer'), 'en', 'Preface writer'),
    ((SELECT id FROM d_roles WHERE key = 'preface_writer'), 'fr', 'Préfacier'),
    ((SELECT id FROM d_roles WHERE key = 'preface_writer'), 'de', 'Vorwortschreiber'),
    ((SELECT id FROM d_roles WHERE key = 'preface_writer'), 'es', 'Prologuista'),
    -- afterword_writer
    ((SELECT id FROM d_roles WHERE key = 'afterword_writer'), 'it', 'Postfatore'),
    ((SELECT id FROM d_roles WHERE key = 'afterword_writer'), 'en', 'Afterword writer'),
    ((SELECT id FROM d_roles WHERE key = 'afterword_writer'), 'fr', 'Postfacier'),
    ((SELECT id FROM d_roles WHERE key = 'afterword_writer'), 'de', 'Nachwortschreiber'),
    ((SELECT id FROM d_roles WHERE key = 'afterword_writer'), 'es', 'Epílogo escritor'),
    -- commentator
    ((SELECT id FROM d_roles WHERE key = 'commentator'), 'it', 'Commentatore'),
    ((SELECT id FROM d_roles WHERE key = 'commentator'), 'en', 'Commentator'),
    ((SELECT id FROM d_roles WHERE key = 'commentator'), 'fr', 'Commentateur'),
    ((SELECT id FROM d_roles WHERE key = 'commentator'), 'de', 'Kommentator'),
    ((SELECT id FROM d_roles WHERE key = 'commentator'), 'es', 'Comentarista'),
    -- other — sempre per ultimo
    ((SELECT id FROM d_roles WHERE key = 'other'), 'it', 'Altro'),
    ((SELECT id FROM d_roles WHERE key = 'other'), 'en', 'Other'),
    ((SELECT id FROM d_roles WHERE key = 'other'), 'fr', 'Autre'),
    ((SELECT id FROM d_roles WHERE key = 'other'), 'de', 'Andere'),
    ((SELECT id FROM d_roles WHERE key = 'other'), 'es', 'Otro');

-- ============================================================
-- d_types
-- ============================================================
INSERT OR IGNORE INTO d_types (key, created_at, updated_at) VALUES
    ('novel',       strftime('%s', 'now'), strftime('%s', 'now')),
    ('novella',     strftime('%s', 'now'), strftime('%s', 'now')),
    ('short_story', strftime('%s', 'now'), strftime('%s', 'now')),
    ('essay',       strftime('%s', 'now'), strftime('%s', 'now')),
    ('biography',   strftime('%s', 'now'), strftime('%s', 'now')),
    ('manual',      strftime('%s', 'now'), strftime('%s', 'now')),
    ('comic',       strftime('%s', 'now'), strftime('%s', 'now')),
    ('graphic_novel', strftime('%s', 'now'), strftime('%s', 'now')),
    ('poetry',      strftime('%s', 'now'), strftime('%s', 'now')),
    ('theatre',     strftime('%s', 'now'), strftime('%s', 'now')),
    ('other',       strftime('%s', 'now'), strftime('%s', 'now'));  -- sempre per ultimo

INSERT OR IGNORE INTO s_type_translations (type_id, language_code, name) VALUES
    -- novel
    ((SELECT id FROM d_types WHERE key = 'novel'), 'it', 'Romanzo'),
    ((SELECT id FROM d_types WHERE key = 'novel'), 'en', 'Novel'),
    ((SELECT id FROM d_types WHERE key = 'novel'), 'fr', 'Roman'),
    ((SELECT id FROM d_types WHERE key = 'novel'), 'de', 'Roman'),
    ((SELECT id FROM d_types WHERE key = 'novel'), 'es', 'Novela'),
    -- novella
    ((SELECT id FROM d_types WHERE key = 'novella'), 'it', 'Romanzo breve'),
    ((SELECT id FROM d_types WHERE key = 'novella'), 'en', 'Novella'),
    ((SELECT id FROM d_types WHERE key = 'novella'), 'fr', 'Roman court'),
    ((SELECT id FROM d_types WHERE key = 'novella'), 'de', 'Novelle'),
    ((SELECT id FROM d_types WHERE key = 'novella'), 'es', 'Novela corta'),
    -- short_story
    ((SELECT id FROM d_types WHERE key = 'short_story'), 'it', 'Racconto'),
    ((SELECT id FROM d_types WHERE key = 'short_story'), 'en', 'Short story'),
    ((SELECT id FROM d_types WHERE key = 'short_story'), 'fr', 'Nouvelle'),
    ((SELECT id FROM d_types WHERE key = 'short_story'), 'de', 'Kurzgeschichte'),
    ((SELECT id FROM d_types WHERE key = 'short_story'), 'es', 'Cuento'),
    -- essay
    ((SELECT id FROM d_types WHERE key = 'essay'), 'it', 'Saggio'),
    ((SELECT id FROM d_types WHERE key = 'essay'), 'en', 'Essay'),
    ((SELECT id FROM d_types WHERE key = 'essay'), 'fr', 'Essai'),
    ((SELECT id FROM d_types WHERE key = 'essay'), 'de', 'Essay'),
    ((SELECT id FROM d_types WHERE key = 'essay'), 'es', 'Ensayo'),
    -- biography
    ((SELECT id FROM d_types WHERE key = 'biography'), 'it', 'Biografia'),
    ((SELECT id FROM d_types WHERE key = 'biography'), 'en', 'Biography'),
    ((SELECT id FROM d_types WHERE key = 'biography'), 'fr', 'Biographie'),
    ((SELECT id FROM d_types WHERE key = 'biography'), 'de', 'Biografie'),
    ((SELECT id FROM d_types WHERE key = 'biography'), 'es', 'Biografía'),
    -- manual
    ((SELECT id FROM d_types WHERE key = 'manual'), 'it', 'Manuale'),
    ((SELECT id FROM d_types WHERE key = 'manual'), 'en', 'Manual'),
    ((SELECT id FROM d_types WHERE key = 'manual'), 'fr', 'Manuel'),
    ((SELECT id FROM d_types WHERE key = 'manual'), 'de', 'Handbuch'),
    ((SELECT id FROM d_types WHERE key = 'manual'), 'es', 'Manual'),
    -- comic
    ((SELECT id FROM d_types WHERE key = 'comic'), 'it', 'Fumetto'),
    ((SELECT id FROM d_types WHERE key = 'comic'), 'en', 'Comic'),
    ((SELECT id FROM d_types WHERE key = 'comic'), 'fr', 'Bande dessinée'),
    ((SELECT id FROM d_types WHERE key = 'comic'), 'de', 'Comic'),
    ((SELECT id FROM d_types WHERE key = 'comic'), 'es', 'Cómic'),
    -- graphic_novel
    ((SELECT id FROM d_types WHERE key = 'graphic_novel'), 'it', 'Graphic novel'),
    ((SELECT id FROM d_types WHERE key = 'graphic_novel'), 'en', 'Graphic novel'),
    ((SELECT id FROM d_types WHERE key = 'graphic_novel'), 'fr', 'Roman graphique'),
    ((SELECT id FROM d_types WHERE key = 'graphic_novel'), 'de', 'Graphic Novel'),
    ((SELECT id FROM d_types WHERE key = 'graphic_novel'), 'es', 'Novela gráfica'),
    -- poetry
    ((SELECT id FROM d_types WHERE key = 'poetry'), 'it', 'Poesia'),
    ((SELECT id FROM d_types WHERE key = 'poetry'), 'en', 'Poetry'),
    ((SELECT id FROM d_types WHERE key = 'poetry'), 'fr', 'Poésie'),
    ((SELECT id FROM d_types WHERE key = 'poetry'), 'de', 'Lyrik'),
    ((SELECT id FROM d_types WHERE key = 'poetry'), 'es', 'Poesía'),
    -- theatre
    ((SELECT id FROM d_types WHERE key = 'theatre'), 'it', 'Teatro'),
    ((SELECT id FROM d_types WHERE key = 'theatre'), 'en', 'Theatre'),
    ((SELECT id FROM d_types WHERE key = 'theatre'), 'fr', 'Théâtre'),
    ((SELECT id FROM d_types WHERE key = 'theatre'), 'de', 'Theater'),
    ((SELECT id FROM d_types WHERE key = 'theatre'), 'es', 'Teatro'),
    -- other — sempre per ultimo
    ((SELECT id FROM d_types WHERE key = 'other'), 'it', 'Altro'),
    ((SELECT id FROM d_types WHERE key = 'other'), 'en', 'Other'),
    ((SELECT id FROM d_types WHERE key = 'other'), 'fr', 'Autre'),
    ((SELECT id FROM d_types WHERE key = 'other'), 'de', 'Andere'),
    ((SELECT id FROM d_types WHERE key = 'other'), 'es', 'Otro');

-- ============================================================
-- d_formats
-- ============================================================
INSERT OR IGNORE INTO d_formats (key, created_at, updated_at) VALUES
    ('epub', strftime('%s', 'now'), strftime('%s', 'now')),
    ('pdf', strftime('%s', 'now'), strftime('%s', 'now')),
    ('mobi', strftime('%s', 'now'), strftime('%s', 'now')),
    ('azw3', strftime('%s', 'now'), strftime('%s', 'now')),
    ('djvu', strftime('%s', 'now'), strftime('%s', 'now')),
    ('cbz', strftime('%s', 'now'), strftime('%s', 'now')),
    ('cbr', strftime('%s', 'now'), strftime('%s', 'now')),
    ('txt', strftime('%s', 'now'), strftime('%s', 'now')),
    ('other', strftime('%s', 'now'), strftime('%s', 'now'));  -- sempre per ultimo

INSERT OR IGNORE INTO s_format_translations (format_id, language_code, name) VALUES
    ((SELECT id FROM d_formats WHERE key = 'epub'), 'it', 'EPUB'),
    ((SELECT id FROM d_formats WHERE key = 'epub'), 'en', 'EPUB'),
    ((SELECT id FROM d_formats WHERE key = 'epub'), 'fr', 'EPUB'),
    ((SELECT id FROM d_formats WHERE key = 'epub'), 'de', 'EPUB'),
    ((SELECT id FROM d_formats WHERE key = 'epub'), 'es', 'EPUB'),
    ((SELECT id FROM d_formats WHERE key = 'pdf'), 'it', 'PDF'),
    ((SELECT id FROM d_formats WHERE key = 'pdf'), 'en', 'PDF'),
    ((SELECT id FROM d_formats WHERE key = 'pdf'), 'fr', 'PDF'),
    ((SELECT id FROM d_formats WHERE key = 'pdf'), 'de', 'PDF'),
    ((SELECT id FROM d_formats WHERE key = 'pdf'), 'es', 'PDF'),
    ((SELECT id FROM d_formats WHERE key = 'mobi'), 'it', 'MOBI'),
    ((SELECT id FROM d_formats WHERE key = 'mobi'), 'en', 'MOBI'),
    ((SELECT id FROM d_formats WHERE key = 'mobi'), 'fr', 'MOBI'),
    ((SELECT id FROM d_formats WHERE key = 'mobi'), 'de', 'MOBI'),
    ((SELECT id FROM d_formats WHERE key = 'mobi'), 'es', 'MOBI'),
    ((SELECT id FROM d_formats WHERE key = 'azw3'), 'it', 'AZW3'),
    ((SELECT id FROM d_formats WHERE key = 'azw3'), 'en', 'AZW3'),
    ((SELECT id FROM d_formats WHERE key = 'azw3'), 'fr', 'AZW3'),
    ((SELECT id FROM d_formats WHERE key = 'azw3'), 'de', 'AZW3'),
    ((SELECT id FROM d_formats WHERE key = 'azw3'), 'es', 'AZW3'),
    ((SELECT id FROM d_formats WHERE key = 'djvu'), 'it', 'DjVu'),
    ((SELECT id FROM d_formats WHERE key = 'djvu'), 'en', 'DjVu'),
    ((SELECT id FROM d_formats WHERE key = 'djvu'), 'fr', 'DjVu'),
    ((SELECT id FROM d_formats WHERE key = 'djvu'), 'de', 'DjVu'),
    ((SELECT id FROM d_formats WHERE key = 'djvu'), 'es', 'DjVu'),
    ((SELECT id FROM d_formats WHERE key = 'cbz'), 'it', 'CBZ'),
    ((SELECT id FROM d_formats WHERE key = 'cbz'), 'en', 'CBZ'),
    ((SELECT id FROM d_formats WHERE key = 'cbz'), 'fr', 'CBZ'),
    ((SELECT id FROM d_formats WHERE key = 'cbz'), 'de', 'CBZ'),
    ((SELECT id FROM d_formats WHERE key = 'cbz'), 'es', 'CBZ'),
    ((SELECT id FROM d_formats WHERE key = 'cbr'), 'it', 'CBR'),
    ((SELECT id FROM d_formats WHERE key = 'cbr'), 'en', 'CBR'),
    ((SELECT id FROM d_formats WHERE key = 'cbr'), 'fr', 'CBR'),
    ((SELECT id FROM d_formats WHERE key = 'cbr'), 'de', 'CBR'),
    ((SELECT id FROM d_formats WHERE key = 'cbr'), 'es', 'CBR'),
    ((SELECT id FROM d_formats WHERE key = 'txt'), 'it', 'Testo semplice'),
    ((SELECT id FROM d_formats WHERE key = 'txt'), 'en', 'Plain text'),
    ((SELECT id FROM d_formats WHERE key = 'txt'), 'fr', 'Texte brut'),
    ((SELECT id FROM d_formats WHERE key = 'txt'), 'de', 'Klartext'),
    ((SELECT id FROM d_formats WHERE key = 'txt'), 'es', 'Texto plano'),
    -- other — sempre per ultimo
    ((SELECT id FROM d_formats WHERE key = 'other'), 'it', 'Altro'),
    ((SELECT id FROM d_formats WHERE key = 'other'), 'en', 'Other'),
    ((SELECT id FROM d_formats WHERE key = 'other'), 'fr', 'Autre'),
    ((SELECT id FROM d_formats WHERE key = 'other'), 'de', 'Andere'),
    ((SELECT id FROM d_formats WHERE key = 'other'), 'es', 'Otro');

-- ============================================================
-- s_person_language_roles
-- ============================================================
INSERT OR IGNORE INTO s_person_language_roles (code, created_at, updated_at) VALUES
    ('native',  strftime('%s', 'now'), strftime('%s', 'now')),
    ('writing', strftime('%s', 'now'), strftime('%s', 'now')),
    ('fluent',  strftime('%s', 'now'), strftime('%s', 'now')),
    ('reading', strftime('%s', 'now'), strftime('%s', 'now')),
    ('other',   strftime('%s', 'now'), strftime('%s', 'now'));  -- sempre per ultimo

INSERT OR IGNORE INTO s_person_language_role_translations (role_id, language_code, label) VALUES
    ((SELECT id FROM s_person_language_roles WHERE code = 'native'), 'it', 'Madrelingua'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'native'), 'en', 'Native language'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'native'), 'fr', 'Langue maternelle'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'native'), 'de', 'Muttersprache'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'writing'), 'it', 'Lingua di scrittura'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'writing'), 'en', 'Writing language'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'writing'), 'fr', 'Langue d''écriture'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'writing'), 'de', 'Schreibsprache'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'fluent'), 'it', 'Fluente'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'fluent'), 'en', 'Fluent'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'fluent'), 'fr', 'Courant'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'fluent'), 'de', 'Fließend'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'reading'), 'it', 'Lettura'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'reading'), 'en', 'Reading'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'reading'), 'fr', 'Lecture'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'reading'), 'de', 'Lesen'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'other'), 'it', 'Altro'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'other'), 'en', 'Other'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'other'), 'fr', 'Autre'),
    ((SELECT id FROM s_person_language_roles WHERE code = 'other'), 'de', 'Andere');

-- ============================================================
-- s_place_types
-- ============================================================
INSERT OR IGNORE INTO s_place_types (key, created_at, updated_at) VALUES
    ('birth',     strftime('%s', 'now'), strftime('%s', 'now')),
    ('death',     strftime('%s', 'now'), strftime('%s', 'now')),
    ('activity',  strftime('%s', 'now'), strftime('%s', 'now')),
    ('residence', strftime('%s', 'now'), strftime('%s', 'now')),
    ('other',     strftime('%s', 'now'), strftime('%s', 'now'));  -- sempre per ultimo

INSERT OR IGNORE INTO s_place_type_translations (place_type_id, language_code, label) VALUES
    ((SELECT id FROM s_place_types WHERE key = 'birth'), 'it', 'Luogo di nascita'),
    ((SELECT id FROM s_place_types WHERE key = 'birth'), 'en', 'Place of birth'),
    ((SELECT id FROM s_place_types WHERE key = 'birth'), 'fr', 'Lieu de naissance'),
    ((SELECT id FROM s_place_types WHERE key = 'birth'), 'de', 'Geburtsort'),
    ((SELECT id FROM s_place_types WHERE key = 'death'), 'it', 'Luogo di morte'),
    ((SELECT id FROM s_place_types WHERE key = 'death'), 'en', 'Place of death'),
    ((SELECT id FROM s_place_types WHERE key = 'death'), 'fr', 'Lieu de décès'),
    ((SELECT id FROM s_place_types WHERE key = 'death'), 'de', 'Sterbeort'),
    ((SELECT id FROM s_place_types WHERE key = 'activity'), 'it', 'Luogo di attività'),
    ((SELECT id FROM s_place_types WHERE key = 'activity'), 'en', 'Place of activity'),
    ((SELECT id FROM s_place_types WHERE key = 'activity'), 'fr', 'Lieu d''activité'),
    ((SELECT id FROM s_place_types WHERE key = 'activity'), 'de', 'Wirkungsort'),
    ((SELECT id FROM s_place_types WHERE key = 'residence'), 'it', 'Residenza'),
    ((SELECT id FROM s_place_types WHERE key = 'residence'), 'en', 'Residence'),
    ((SELECT id FROM s_place_types WHERE key = 'residence'), 'fr', 'Résidence'),
    ((SELECT id FROM s_place_types WHERE key = 'residence'), 'de', 'Wohnort'),
    ((SELECT id FROM s_place_types WHERE key = 'other'), 'it', 'Altro'),
    ((SELECT id FROM s_place_types WHERE key = 'other'), 'en', 'Other'),
    ((SELECT id FROM s_place_types WHERE key = 'other'), 'fr', 'Autre'),
    ((SELECT id FROM s_place_types WHERE key = 'other'), 'de', 'Andere');

-- ============================================================
-- d_languages
-- ============================================================
INSERT OR IGNORE INTO d_languages (iso_code_2char, iso_code_3char, official_name, native_name, created_at, updated_at) VALUES
    ('it', 'ita', 'Italian',      'italiano',          strftime('%s', 'now'), strftime('%s', 'now')),
    ('en', 'eng', 'English',      'English',           strftime('%s', 'now'), strftime('%s', 'now')),
    ('fr', 'fra', 'French',       'français',          strftime('%s', 'now'), strftime('%s', 'now')),
    ('es', 'spa', 'Spanish',      'español',           strftime('%s', 'now'), strftime('%s', 'now')),
    ('de', 'deu', 'German',       'Deutsch',           strftime('%s', 'now'), strftime('%s', 'now')),
    ('zh', 'zho', 'Chinese',      '中文',               strftime('%s', 'now'), strftime('%s', 'now')),
    ('ja', 'jpn', 'Japanese',     '日本語',             strftime('%s', 'now'), strftime('%s', 'now')),
    ('ko', 'kor', 'Korean',       '한국어',             strftime('%s', 'now'), strftime('%s', 'now')),
    ('la', 'lat', 'Latin',        'lingua latina',     strftime('%s', 'now'), strftime('%s', 'now')),
    (NULL, 'grc', 'Ancient Greek','ἀρχαία ἑλληνική',   strftime('%s', 'now'), strftime('%s', 'now'));

-- ============================================================
-- s_content_language_roles
-- ============================================================
INSERT OR IGNORE INTO s_content_language_roles (code, created_at, updated_at) VALUES
    ('original', strftime('%s', 'now'), strftime('%s', 'now')),
    ('source',   strftime('%s', 'now'), strftime('%s', 'now')),
    ('actual',   strftime('%s', 'now'), strftime('%s', 'now')),
    ('other',    strftime('%s', 'now'), strftime('%s', 'now'));  -- sempre per ultimo

INSERT OR IGNORE INTO s_content_language_role_translations (role_id, language_code, label) VALUES
    ((SELECT id FROM s_content_language_roles WHERE code = 'original'), 'it', 'Lingua originale'),
    ((SELECT id FROM s_content_language_roles WHERE code = 'original'), 'en', 'Original language'),
    ((SELECT id FROM s_content_language_roles WHERE code = 'original'), 'fr', 'Langue originale'),
    ((SELECT id FROM s_content_language_roles WHERE code = 'original'), 'de', 'Originalsprache'),
    ((SELECT id FROM s_content_language_roles WHERE code = 'source'), 'it', 'Lingua di partenza'),
    ((SELECT id FROM s_content_language_roles WHERE code = 'source'), 'en', 'Source language'),
    ((SELECT id FROM s_content_language_roles WHERE code = 'source'), 'fr', 'Langue source'),
    ((SELECT id FROM s_content_language_roles WHERE code = 'source'), 'de', 'Ausgangssprache'),
    ((SELECT id FROM s_content_language_roles WHERE code = 'actual'), 'it', 'Lingua del testo'),
    ((SELECT id FROM s_content_language_roles WHERE code = 'actual'), 'en', 'Text language'),
    ((SELECT id FROM s_content_language_roles WHERE code = 'actual'), 'fr', 'Langue du texte'),
    ((SELECT id FROM s_content_language_roles WHERE code = 'actual'), 'de', 'Textsprache'),
    ((SELECT id FROM s_content_language_roles WHERE code = 'other'), 'it', 'Altro'),
    ((SELECT id FROM s_content_language_roles WHERE code = 'other'), 'en', 'Other'),
    ((SELECT id FROM s_content_language_roles WHERE code = 'other'), 'fr', 'Autre'),
    ((SELECT id FROM s_content_language_roles WHERE code = 'other'), 'de', 'Andere');

-- ============================================================
-- s_book_language_roles
-- ============================================================
INSERT OR IGNORE INTO s_book_language_roles (code, created_at, updated_at) VALUES
    ('actual', strftime('%s', 'now'), strftime('%s', 'now')),
    ('other',  strftime('%s', 'now'), strftime('%s', 'now'));  -- sempre per ultimo

INSERT OR IGNORE INTO s_book_language_role_translations (role_id, language_code, label) VALUES
    ((SELECT id FROM s_book_language_roles WHERE code = 'actual'), 'it', 'Lingua del testo'),
    ((SELECT id FROM s_book_language_roles WHERE code = 'actual'), 'en', 'Text language'),
    ((SELECT id FROM s_book_language_roles WHERE code = 'actual'), 'fr', 'Langue du texte'),
    ((SELECT id FROM s_book_language_roles WHERE code = 'actual'), 'de', 'Textsprache'),
    ((SELECT id FROM s_book_language_roles WHERE code = 'other'),  'it', 'Altro'),
    ((SELECT id FROM s_book_language_roles WHERE code = 'other'),  'en', 'Other'),
    ((SELECT id FROM s_book_language_roles WHERE code = 'other'),  'fr', 'Autre'),
    ((SELECT id FROM s_book_language_roles WHERE code = 'other'),  'de', 'Andere');

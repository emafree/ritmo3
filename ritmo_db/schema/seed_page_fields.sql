-- Seed della tabella page_fields.
-- Eseguito dopo la creazione dello schema.
-- DELETE + INSERT garantisce uno stato sempre deterministico e aggiornabile.

DELETE FROM page_fields;

INSERT OR IGNORE INTO page_fields (page, field_key, data_kind, sort_order, enum_values, relation_type, target_table, target_field) VALUES
    -- book_page
    ('book_page', 'field-original-title',   'string', 10, NULL, 'direct',   NULL,             NULL),
    ('book_page', 'field-publication-date', 'date',   20, NULL, 'direct',   NULL,             NULL),
    ('book_page', 'field-isbn',             'string', 30, NULL, 'direct',   NULL,             NULL),
    ('book_page', 'field-publisher',        'string', 40, NULL, 'fk',       'publishers',     'publisher_id'),
    ('book_page', 'field-notes',            'string', 50, NULL, 'direct',   NULL,             NULL),
    ('book_page', 'field-tags',             'string', 55, NULL, 'junction', 'x_books_tags',          NULL),
    ('book_page', 'field-people',           'person', 60, NULL, 'junction', 'x_books_people_roles',   NULL),

    -- content_page
    ('content_page', 'field-title',    'string', 10, NULL,                                                               'direct',   NULL,                      NULL),
    ('content_page', 'field-author',   'person', 20, NULL,                                                               'junction', 'x_contents_people_roles', NULL),
    ('content_page', 'field-language', 'enum',   30, '["it","en","fr","de","es","pt","ru","zh","ja","ko","ar","nl","pl","sv","da","fi","no","cs","hu","ro","tr","uk","ca","hr","sk","sl","bg","el","he","id","vi","th","ms"]', 'direct', NULL, NULL),
    ('content_page', 'field-people',   'person', 40, NULL,                                                               'junction', 'x_contents_people_roles', NULL),

    -- people_page
    ('people_page', 'field-display-name', 'string', 10, NULL, 'direct', NULL, NULL),
    ('people_page', 'field-nationality',  'string', 20, NULL, 'direct', NULL, NULL),
    ('people_page', 'field-birth-date',   'date',   30, NULL, 'direct', NULL, NULL),
    ('people_page', 'field-death-date',   'date',   40, NULL, 'direct', NULL, NULL);

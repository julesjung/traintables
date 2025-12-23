CREATE VIRTUAL TABLE stations_fts USING fts5(
    id UNINDEXED,
    name,
    tokenize = 'unicode61 remove_diacritics 2'
);

INSERT INTO stations_fts (id, name)
SELECT id, name
FROM stations;
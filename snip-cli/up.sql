PRAGMA foreign_keys = ON;
PRAGMA foreign_keys;

DROP TABLE snippets;
DROP TABLE tags;
DROP TABLE snippet_to_tags;
DROP TABLE snippet_groups;
DROP TABLE snippets_fts;
DROP TABLE tags_fts;

CREATE TABLE snippets (
    id integer PRIMARY KEY, 
    date text NOT NULL,
    snippet text NOT NULL,
    filename text, 
    filetype text, 
    group_id integer, 
    description text,
    FOREIGN KEY (group_id) REFERENCES snippet_groups (id)
);

CREATE VIRTUAL TABLE snippets_fts USING fts5 (
    filename,  
    snippet,
    description,
);

CREATE TABLE snippet_groups (
    id integer PRIMARY KEY,
    count integer NOT NULL
);


CREATE TABLE tags (
    id INTEGER PRIMARY KEY, 
    tag TEXT NOT NULL,
    unique(tag)
);

CREATE VIRTUAL TABLE tags_fts USING fts5 (
    id UNINDEXED,
    tag, 
    content='tags',
    content_rowid='id'
);

CREATE TABLE snippet_to_tags (
    snippet_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (snippet_id) REFERENCES snippets (id),
    FOREIGN KEY (tag_id) REFERENCES tags (id)
);


CREATE TRIGGER snippets_on_insert AFTER INSERT ON snippets BEGIN
 INSERT INTO snippets_fts(
        filename, 
        snippet, 
        description
    ) 
    VALUES (
        new.filename, 
        new.snippet, 
        new.description
    );
END;

INSERT INTO snippet_groups (id, count) VALUES (0, 0);
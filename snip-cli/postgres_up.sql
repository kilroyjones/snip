\c snip

DROP TABLE IF EXISTS snippet_to_tags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS snippets;
DROP TABLE IF EXISTS snippet_groups;

CREATE TABLE snippet_groups (
    id serial PRIMARY KEY,
    count INT NOT NULL
);

CREATE TABLE tags (
    id INT PRIMARY KEY, 
    tag VARCHAR(255) NOT NULL,
    CONSTRAINT UNIQUE_TAG UNIQUE(tag)
);

ALTER TABLE tags ADD "document_vectors" tsvector;
CREATE INDEX idx_fts_doc_vec_tags ON tags using gin(document_vectors);
UPDATE 
    tags
SET 
    document_vectors = (to_tsvector(tag));

CREATE TRIGGER tsvectorupdate BEFORE INSERT OR UPDATE
    ON tags FOR EACH ROW EXECUTE PROCEDURE
    tsvector_update_trigger(document_vectors, 'pg_catalog.english', tag);

CREATE TABLE snippets (
    id serial PRIMARY KEY, 
    date TIMESTAMP NOT NULL,
    snippet VARCHAR NOT NULL,
    filename VARCHAR(255), 
    filetype VARCHAR(255), 
    group_id INT, 
    description VARCHAR,
    FOREIGN KEY (group_id) REFERENCES snippet_groups (id)
);

ALTER TABLE snippets ADD "document_vectors" tsvector;
CREATE INDEX idx_fts_doc_vec_snippets ON snippets using gin(document_vectors);
UPDATE 
    snippets
SET 
    document_vectors = (to_tsvector(snippet) || to_tsvector(description));

CREATE TRIGGER tsvectorupdate BEFORE INSERT OR UPDATE
    ON snippets FOR EACH ROW EXECUTE PROCEDURE
    tsvector_update_trigger(document_vectors, 'pg_catalog.english', snippet, description);

CREATE TABLE snippet_to_tags (
    snippet_id INT NOT NULL,
    tag_id INT NOT NULL,
    FOREIGN KEY (snippet_id) REFERENCES snippets (id),
    FOREIGN KEY (tag_id) REFERENCES tags (id)
);


DROP DATABASE snip;
CREATE DATABASE snip;

\c snip

DROP TABLE IF EXISTS snippet_to_tags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS quotes;
DROP TABLE IF EXISTS images;

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

-- TABLE quotes start
CREATE TABLE quotes (
    id serial PRIMARY KEY, 
    date TIMESTAMP NOT NULL,
    quote VARCHAR NOT NULL,
    source VARCHAR,
    code_type VARCHAR,
    description VARCHAR,
    icon_path VARCHAR,
    trashed BOOLEAN NOT NULL DEFAULT false 
);

ALTER TABLE quotes ADD "document_vectors" tsvector;
CREATE INDEX idx_fts_doc_vec_quotes ON quotes using gin(document_vectors);
UPDATE 
    quotes
SET 
    document_vectors = (to_tsvector(quote) || to_tsvector(description));

CREATE TRIGGER tsvectorupdate BEFORE INSERT OR UPDATE
    ON quotes FOR EACH ROW EXECUTE PROCEDURE
    tsvector_update_trigger(document_vectors, 'pg_catalog.english', quote, description);

-- TABLE images start
CREATE TABLE images (
    id serial PRIMARY KEY, 
    date TIMESTAMP NOT NULL,
    source VARCHAR,
    description VARCHAR,
    original_path VARCHAR,
    thumbnail_path VARCHAR,
    trashed BOOLEAN NOT NULL DEFAULT false 
);

ALTER TABLE images ADD "document_vectors" tsvector;
CREATE INDEX idx_fts_doc_vec_images ON images using gin(document_vectors);
UPDATE 
    images
SET 
    document_vectors = (to_tsvector(description) || to_tsvector(original_path));

CREATE TRIGGER tsvectorupdate BEFORE INSERT OR UPDATE
    ON images FOR EACH ROW EXECUTE PROCEDURE
    tsvector_update_trigger(document_vectors, 'pg_catalog.english', description, original_path);

-- TABLE bookmarks start
CREATE TABLE bookmarks (
    id serial PRIMARY KEY, 
    date TIMESTAMP NOT NULL,
    link VARCHAR,
    description VARCHAR,
    icon_path VARCHAR,
    trashed BOOLEAN NOT NULL DEFAULT false 
);

ALTER TABLE bookmarks ADD "document_vectors" tsvector;
CREATE INDEX idx_fts_doc_vec_bookmarks ON bookmarks using gin(document_vectors);
UPDATE 
    bookmarks
SET 
    document_vectors = (to_tsvector(description) || to_tsvector(link));

CREATE TRIGGER tsvectorupdate BEFORE INSERT OR UPDATE
    ON bookmarks FOR EACH ROW EXECUTE PROCEDURE
    tsvector_update_trigger(document_vectors, 'pg_catalog.english', description, link);

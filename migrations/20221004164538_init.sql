-- Add migration script here
CREATE TABLE IF NOT EXISTS quotes (id INT NOT NULL PRIMARY KEY, title VARCHAR(255), hash VARCHAR(255), uploaded_at BIGINT, admin_key VARCHAR(255) DEFAULT NULL);
CREATE SEQUENCE quotes_id_seq;
ALTER TABLE quotes ALTER COLUMN id SET DEFAULT nextval('quotes_id_seq');
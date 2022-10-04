-- Add migration script here
CREATE TABLE IF NOT EXISTS quotes (title VARCHAR(255), hash VARCHAR(255), uploaded_at DATE);
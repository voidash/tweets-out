-- Add migration script here
  CREATE TABLE IF NOT EXISTS Posts (
    date BIGINT PRIMARY KEY,
    title TEXT,
    posted BOOL,
    description TEXT NOT NULL,
    image_path TEXT
  );


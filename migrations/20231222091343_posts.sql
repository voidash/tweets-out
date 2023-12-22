-- Add migration script here
  CREATE TABLE IF NOT EXISTS Posts (
    date INTEGER PRIMARY KEY,
    title TEXT,
    description TEXT NOT NULL,
    image_path TEXT 
)        


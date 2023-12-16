-- Add migration script here
CREATE TABLE IF NOT EXISTS quotes (
    id UUID PRIMARY KEY,
    book varchar NOT NULL,
    quote TEXT NOT NULL,
    inserted_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    UNIQUE (book, quote)
)
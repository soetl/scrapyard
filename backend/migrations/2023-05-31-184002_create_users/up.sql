CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    image TEXT,
    password_hash TEXT NOT NULL
);
CREATE TABLE pastes (
    id SERIAL PRIMARY KEY,
    owner INTEGER NOT NULL REFERENCES users ON DELETE CASCADE,
    filename TEXT NOT NULL,
    mime TEXT NOT NULL,
    link TEXT NOT NULL,
    type TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
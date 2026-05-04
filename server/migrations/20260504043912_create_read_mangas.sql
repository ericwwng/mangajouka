CREATE TABLE read_mangas (
    id UUID PRIMARY KEY,
    user_id Uuid NOT NULL,
    manga_id TEXT NOT NULL,
    rating INTEGER,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, manga_id)
);

CREATE INDEX idx_read_mangas_user_id ON read_mangas(user_id);

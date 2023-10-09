DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

DROP TABLE IF EXISTS filtered_mangas;

CREATE TABLE filtered_mangas (
    user_id INTEGER NOT NULL,
    manga_id TEXT NOT NULL,

    FOREIGN KEY(user_id) REFERENCES users(id),
    UNIQUE(user_id, manga_id)
);

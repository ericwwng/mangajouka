DROP TABLE IF EXISTS manga_information;

CREATE TABLE manga_information (
    manga_id TEXT NOT NULL,
    manga_name TEXT,
    manga_description TEXT,
    manga_tags TEXT[], 

    UNIQUE(manga_id)
);

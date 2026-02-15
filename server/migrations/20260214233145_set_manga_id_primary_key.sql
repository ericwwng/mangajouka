ALTER TABLE manga_information ALTER COLUMN manga_id SET NOT NULL;

ALTER TABLE manga_information
ADD PRIMARY KEY (manga_id);

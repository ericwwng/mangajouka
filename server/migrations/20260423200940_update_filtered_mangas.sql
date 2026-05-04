ALTER TABLE filtered_mangas ADD COLUMN IF NOT EXISTS id UUID DEFAULT gen_random_uuid();

-- Clean up the constraints
-- Drop the old unique constraint
ALTER TABLE filtered_mangas DROP CONSTRAINT IF EXISTS filtered_mangas_user_id_manga_id_key;

-- Re-add the unique constraint to ensure no duplicate filters per user
ALTER TABLE filtered_mangas ADD CONSTRAINT unique_user_manga_filter UNIQUE (user_id, manga_id);

ALTER TABLE filtered_mangas ADD PRIMARY KEY (id);

-- Add a performance index
-- This speeds up the "Find all filtered mangas for this user" query
CREATE INDEX IF NOT EXISTS idx_filtered_mangas_user_id ON filtered_mangas(user_id);

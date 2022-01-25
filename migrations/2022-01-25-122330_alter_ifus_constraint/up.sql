ALTER TABLE ifus
    ALTER COLUMN file_id SET NOT NULL;

ALTER TABLE ifus
    ADD CONSTRAINT unique_file_id UNIQUE (file_id);

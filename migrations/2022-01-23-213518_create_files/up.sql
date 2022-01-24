CREATE TABLE files (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    content_type VARCHAR(255) NOT NULL,
    size BIGINT NOT NULL,
    data BYTEA NOT NULL
);

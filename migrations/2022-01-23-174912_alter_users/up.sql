ALTER TABLE users
    ADD COLUMN admin_status BOOLEAN NOT NULL DEFAULT FALSE;

UPDATE users
    SET admin_status = TRUE
    WHERE id = 1;

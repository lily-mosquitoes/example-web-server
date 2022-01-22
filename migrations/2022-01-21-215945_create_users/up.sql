CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    last_login TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users (username, password_hash)
    VALUES ('development', '$argon2id$v=19$m=4096,t=3,p=1$Ilc/+P4oEbTRy4+BtFp57Q$9zS4oQIZORGvnmZD1bN4HUYaTC+R9o7T9jmpih0vGfU');

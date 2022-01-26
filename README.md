# Rust Example Web Server

## setup
- setup environment variables at `.env`

- run postgresql (provided on `docker-compose.yml`):

     `docker-compose up -d`

- install diesel:

     `cargo install diesel_cli --no-default-features --freatures postgres`

- run the migrations:

     `diesel migration run`

- install node modules

     `npm install`

- run tailwind

     `npx tailwindcss -i ./static_src/css/styles.css -o ./static/css/styles.css --watch`

- run the rocket application:

     `cargo run`

## using diesel
- to generate a new migration:

     `diesel migration generate migration_name_here`

- then write the SQL for the new migration at `up.sql` and to revert it at `down.sql`, save and run:

     `diesel migration run`

# Rust Example Web Server

## setup
- setup environment variables at `.env`

- run postgresql (provided on docker-compose)
`docker-compose up -d`

- install diesel
`cargo install diesel_cli --no-default-features --freatures postgres`

- run the migrations
`diesel migration run`

- run the rocket application
`cargo run`

## using diesel
- to generate a new migration
`diesel migration generate migration_name_here`
Then write the SQL for the new migration at `up.sql` and to revert it at `down.sql`

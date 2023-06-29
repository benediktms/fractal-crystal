set dotenv-load

alias t := test
alias mg := migration-generate
alias mu := migrate-up
alias md := migrate-down

get-config:
    @echo "DATABASE_URL: $DATABASE_URL"

# run tests for a specific cargo package
test PACKAGE:
    cargo test --package {{PACKAGE}}

# run tests for all cargo packages
test-all:
    cargo test --workspace

# run a specific cargo binary
run BIN:
    cargo run --bin {{BIN}}

migration-generate NAME:
    sea-orm-cli migrate generate {{NAME}} \
        --database-url $DATABASE_URL \
        --migration-dir packages/migration/src

migrate-up:
    sea-orm-cli migrate up \
        --database-url $DATABASE_URL \
        --migration-dir packages/migration/src \
        -d packages/migration

migrate-down:
    sea-orm-cli migrate down

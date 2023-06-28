alias t := test

host := `uname -a`

# run tests for a specific cargo package
test TEST:
    cargo test --package {{TEST}}

# run tests for all cargo packages
test-all:
    cargo test --workspace

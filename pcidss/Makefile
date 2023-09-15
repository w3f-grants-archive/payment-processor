# Description: Makefile for pcidss

# Build the project
build:
	$ cargo build --release

# Run tests
test:
	$ cargo test -- --test-threads=1

# Run lints and format checks
lint:
	$ cargo clippy -- -D warnings && cargo fmt --all --check

# Run in development mode
run:
	$ cargo run -p pcidss-oracle -r -- --dev

build-docker:
	$ docker build -t pcidss-oracle .
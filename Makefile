SHELL := /bin/sh

.PHONY: run parse file author test fmt clippy precommit build clean doc

run:
	cargo run -- --help

parse:
	# Usage: make parse COLOR=#1A2B3C
	cargo run -- parse $(COLOR)

file:
	# Usage: make file PATH=./colors.txt
	cargo run -- file $(PATH)

author:
	cargo run -- author

test:
	cargo test --all

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

precommit: fmt clippy test
	@echo "Pre-commit checks passed."

build:
	cargo build --release

clean:
	cargo clean

doc:
	cargo doc --no-deps --open

.PHONY: build run clean

run:
	cargo run

build:
	cargo build

release:
	cargo build --release

clean:
	cargo clean

test:
	cargo test

doc:
	cargo doc --open

fmt:
	cargo fmt

check:
	cargo check

clippy:
	cargo clippy -- -D warnings
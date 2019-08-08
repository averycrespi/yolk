.PHONY: all build doc run test

all: run

build:
	cargo build

doc:
	cargo doc --open

run:
	cargo run

test:
	cargo test

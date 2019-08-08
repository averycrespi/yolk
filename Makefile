.PHONY: all build doc test

all: test build

build:
	cargo build

doc:
	cargo doc --open

test:
	cargo test

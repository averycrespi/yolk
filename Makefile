.PHONY: all build doc run

all: run

build:
	cargo build

doc:
	cargo doc --open

run:
	cargo run

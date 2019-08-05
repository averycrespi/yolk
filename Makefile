.PHONY: all doc open run

all: run

doc:
	cargo doc

open:
	cargo doc --open

run:
	cargo run

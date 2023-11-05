.PHONY: build
build:
	cargo build --bin solution

.PHONY: run
run:
	cargo run --bin solution

.PHONY: test
test:
	cargo test --bin solution -- --nocapture

.PHONY: sf singlefile
sf: singlefile
singlefile:
	../singlefile $(shell pwd)

.PHONY: sf_build
sf_build:
	cargo build --bin singlefile

.PHONY: sf_run
sf_run:
	cargo run --bin singlefile

.PHONY: sf_copy
sf_copy:
	cat src/bin/singlefile.rs | pbcopy

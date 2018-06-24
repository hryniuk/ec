.PHONY: all
all: build test doc

.PHONY: build
build:
	cargo build

.PHONY: test
test:
	cargo test

.PHONY: doc
doc:
	cargo rustdoc -- --no-defaults --passes strip-hidden --passes collapse-docs --passes unindent-comments --passes strip-priv-imports


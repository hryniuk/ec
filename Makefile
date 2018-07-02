.PHONY: default
default: build test

.PHONY: test
test:
	cargo test

.PHONY: all
all: build test doc

.PHONY: build
build:
	cargo build

.PHONY: doc
doc:
	cargo rustdoc -- --no-defaults --passes strip-hidden --passes collapse-docs --passes unindent-comments --passes strip-priv-imports


all: build test doc

build:
	cargo build

test:
	cargo test

doc:
	cargo rustdoc -- --no-defaults --passes strip-hidden --passes collapse-docs --passes unindent-comments --passes strip-priv-imports


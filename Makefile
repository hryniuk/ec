.PHONY: default
default: build test

.PHONY: test
test: ut system_tests

.PHONY: ut
ut:
	cargo test

.PHONY: system_tests
system_tests:
	python3 system_tests.py --bin target/debug/ec --test-files test_files/

.PHONY: all
all: build test doc

.PHONY: build
build:
	cargo build

.PHONY: doc
doc:
	cargo rustdoc -- --no-defaults --passes strip-hidden --passes collapse-docs --passes unindent-comments --passes strip-priv-imports


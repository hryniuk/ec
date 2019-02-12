.PHONY: default
default: build test ## Builds debug version and run all tests.

.PHONY: test
test: build ut system_tests

.PHONY: ut
ut: ## Runs unit tests.
	cargo test

.PHONY: system_tests
system_tests: ## Runs tests which check if EC correctly parses existing test ALFs.
	python3 system_tests.py --bin build/debug/ec --test-files test_files/

.PHONY: all
all: build test doc

.PHONY: build
build: ## Builds debug version under build/ directory.
	mkdir -p build
	cargo build --target-dir build

.PHONY: doc
doc: ## Generates docs by running `cargo rustdoc` with predetermined flags.
	cargo rustdoc -- --no-defaults --passes strip-hidden --passes collapse-docs --passes unindent-comments --passes strip-priv-imports

.PHONY: clean
clean: ## Runs `cargo clean`.
	cargo clean

PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

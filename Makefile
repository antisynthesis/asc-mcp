.PHONY: all build clean test lint fmt clippy install run e2e parity

BINARY_NAME=asc-mcp
BUILD_DIR=bin
CARGO=cargo

all: build

build:
	@./script/build.zsh

clean:
	@rm -rf $(BUILD_DIR)
	@$(CARGO) clean

test:
	@./script/test.zsh

lint:
	@./script/lint.zsh

fmt:
	@$(CARGO) fmt

clippy:
	@$(CARGO) clippy --all-targets -- -D warnings

install:
	@./script/install.zsh

run: build
	@./$(BUILD_DIR)/$(BINARY_NAME) serve

e2e: build
	@./e2e/e2e.zsh

# Differential parity check against the original Go server (needs Go).
parity:
	@./tests/parity/build-go-reference.sh "" target/asc-mcp-go
	@$(CARGO) build
	@python3 tests/parity/protocol.py --go target/asc-mcp-go --rust target/debug/$(BINARY_NAME)
	@python3 tests/parity/parity.py --go target/asc-mcp-go --rust target/debug/$(BINARY_NAME)

.DEFAULT_GOAL := build

.PHONY: all build clean test lint fmt vet install run e2e

BINARY_NAME=asc-mcp
BUILD_DIR=bin
GO=go

all: build

build:
	@./script/build.zsh

clean:
	@rm -rf $(BUILD_DIR)
	@$(GO) clean

test:
	@./script/test.zsh

lint:
	@./script/lint.zsh

fmt:
	@$(GO) fmt ./...

vet:
	@$(GO) vet ./...

install:
	@./script/install.zsh

run: build
	@./$(BUILD_DIR)/$(BINARY_NAME) serve

e2e: build
	@./e2e/e2e.zsh

.DEFAULT_GOAL := build

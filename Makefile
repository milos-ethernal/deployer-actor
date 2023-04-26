build: install-toolchain
	cargo build --workspace

lint-check: install-toolchain
	cargo fmt --check

clippy-check: install-toolchain
	cargo clippy --workspace -- -D warnings

lint:
	cargo fmt --all

test: install-toolchain
	cargo test

test-integration: install-toolchain
	cargo test --package integration

clean:
	rm -rf Cargo.lock
	cargo clean

install-toolchain:
	rustup update
	rustup component add rustfmt
	rustup component add clippy
	rustup target add wasm32-unknown-unknown

CURRENT_DIR := $(dir $(abspath $(firstword $(MAKEFILE_LIST))))
WBUILD_DIR := $(CURRENT_DIR)target/debug/wbuild
OUTPUT_DIR := $(CURRENT_DIR)output

output: build
	mkdir -p output
	rm -rf $(OUTPUT_DIR)/*
	$(foreach folder, $(wildcard $(WBUILD_DIR)/*), $(foreach file, $(wildcard $(folder)/*.compact.wasm), cp -f $(file) $(OUTPUT_DIR)))

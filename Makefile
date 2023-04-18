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
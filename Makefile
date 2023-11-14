ROOT := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

.PHONY: build-rust
build-rust:
	@cd ./alloy-rs && cargo build --release
	@cp ./alloy-rs/target/release/liballoy_rs.a ./alloy-go/clib/

.PHONY: test-rust
test-rust:
	@cd ./alloy-rs && cargo test

.PHONY: clean-rust
clean-rust:
	@cd ./alloy-rs && cargo clean

.PHONY: check-rust
check-rust:
	@cd ./alloy-rs && cargo check

.PHONY: format-rust
format-rust:
	@cd ./alloy-rs && cargo fmt --all -- --check

.PHONY: lint-rust
lint-rust:
	@cd ./alloy-rs && cargo clippy --fix

.PHONY: clean-go
clean-go:
	@cd ./examples && go clean -cache && go clean -modcache

.PHONY: clean-all
clean-all: clean-go clean-rust

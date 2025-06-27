.PHONY: build release test clean check fmt clippy install docs examples

# Default target
all: build

# Build in debug mode
build:
	cargo build

# Build in release mode
release:
	cargo build --release

# Run tests
test:
	cargo test

# Run integration tests
test-integration:
	cargo test --test integration_tests

# Run benchmarks
bench:
	cargo bench

# Clean build artifacts
clean:
	cargo clean
	rm -rf target/
	rm -rf docs/_build/

# Check code without building
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Run clippy linter
clippy:
	cargo clippy -- -D warnings

# Install dependencies
install:
	rustup update
	cargo install cargo-audit
	cargo install cargo-tarpaulin

# Generate documentation
docs:
	cargo doc --no-deps --open

# Build examples
examples:
	cargo build --examples

# Run example
example-parse:
	cargo run --example parse examples/sample_telemetry.json

example-render:
	cargo run --example render examples/sample_telemetry.json output.webm

# Security audit
audit:
	cargo audit

# Code coverage
coverage:
	cargo tarpaulin --out Html

# Install pre-commit hooks
install-hooks:
	pre-commit install

# Run pre-commit hooks
hooks:
	pre-commit run --all-files

# Development setup
setup: install install-hooks
	@echo "Development environment setup complete!"

# Quick development cycle
dev: fmt clippy test

# Release preparation
release-prep: clean fmt clippy test audit
	@echo "Release preparation complete!"

# Help
help:
	@echo "Available targets:"
	@echo "  build          - Build in debug mode"
	@echo "  release        - Build in release mode"
	@echo "  test           - Run tests"
	@echo "  test-integration - Run integration tests"
	@echo "  bench          - Run benchmarks"
	@echo "  clean          - Clean build artifacts"
	@echo "  check          - Check code without building"
	@echo "  fmt            - Format code"
	@echo "  clippy         - Run clippy linter"
	@echo "  install        - Install dependencies"
	@echo "  docs           - Generate documentation"
	@echo "  examples       - Build examples"
	@echo "  audit          - Security audit"
	@echo "  coverage       - Code coverage"
	@echo "  setup          - Development setup"
	@echo "  dev            - Quick development cycle"
	@echo "  release-prep   - Release preparation" 
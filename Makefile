# GridPointer Makefile

.PHONY: all build release test clean install uninstall fmt clippy bench docs

# Default target
all: build

# Development build
build:
	cargo build

# Release build with optimizations
release:
	cargo build --release

# Run all tests
test:
	cargo test --all

# Run tests with coverage (requires cargo-tarpaulin)
test-coverage:
	cargo tarpaulin --out Html --output-dir coverage/

# Clean build artifacts
clean:
	cargo clean
	rm -rf coverage/

# Install to ~/.local/bin
install: release
	mkdir -p ~/.local/bin
	cp target/release/gridpointer ~/.local/bin/
	chmod +x ~/.local/bin/gridpointer
	@echo "GridPointer installed to ~/.local/bin/gridpointer"

# Install systemd service
install-service: install
	mkdir -p ~/.config/systemd/user
	cp examples/gridpointer.service ~/.config/systemd/user/
	systemctl --user daemon-reload
	@echo "Systemd service installed. Enable with: systemctl --user enable gridpointer"

# Uninstall
uninstall:
	systemctl --user stop gridpointer 2>/dev/null || true
	systemctl --user disable gridpointer 2>/dev/null || true
	rm -f ~/.local/bin/gridpointer
	rm -f ~/.config/systemd/user/gridpointer.service
	systemctl --user daemon-reload
	@echo "GridPointer uninstalled"

# Format code
fmt:
	cargo fmt --all

# Check formatting
fmt-check:
	cargo fmt --all -- --check

# Linting
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Run benchmarks
bench:
	cargo bench

# Generate documentation
docs:
	cargo doc --no-deps --open

# Development workflow - format, lint, test
dev-check: fmt clippy test
	@echo "All checks passed!"

# Package for distribution (requires cargo-deb)
package-deb: release
	cargo deb

# Package for AUR (creates PKGBUILD)
package-aur:
	@echo "Creating PKGBUILD for AUR..."
	@echo "# Maintainer: Your Name <email@example.com>" > PKGBUILD
	@echo "pkgname=gridpointer" >> PKGBUILD
	@echo "pkgver=$(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')" >> PKGBUILD
	@echo "pkgrel=1" >> PKGBUILD
	@echo 'pkgdesc="Grid-based cursor control daemon for Wayland"' >> PKGBUILD
	@echo 'arch=("x86_64")' >> PKGBUILD
	@echo 'url="https://github.com/yourusername/gridpointer"' >> PKGBUILD
	@echo 'license=("MIT")' >> PKGBUILD
	@echo 'depends=("wayland" "systemd")' >> PKGBUILD
	@echo 'makedepends=("rust" "cargo")' >> PKGBUILD
	@echo 'source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")' >> PKGBUILD
	@echo "Generated PKGBUILD for version $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')"

# Run in development mode with debug logging
dev-run:
	RUST_LOG=gridpointer=debug cargo run

# Profile performance (requires cargo-profiling tools)
profile:
	cargo build --release
	perf record --call-graph=dwarf target/release/gridpointer
	perf report

# Memory leak check (requires valgrind)
memcheck:
	cargo build
	valgrind --leak-check=full --show-leak-kinds=all target/debug/gridpointer

# Static analysis (requires cargo-audit)
audit:
	cargo audit

# Security check
security-check: audit clippy
	@echo "Security checks completed"

# Full CI pipeline
ci: fmt-check clippy test audit
	@echo "CI pipeline completed successfully"

# Help
help:
	@echo "Available targets:"
	@echo "  build          - Development build"
	@echo "  release        - Optimized release build"
	@echo "  test           - Run all tests"
	@echo "  clean          - Clean build artifacts"
	@echo "  install        - Install to ~/.local/bin"
	@echo "  install-service - Install with systemd service"
	@echo "  uninstall      - Remove installation"
	@echo "  fmt            - Format code"
	@echo "  clippy         - Run linter"
	@echo "  bench          - Run benchmarks"
	@echo "  docs           - Generate documentation"
	@echo "  dev-check      - Format, lint, and test"
	@echo "  dev-run        - Run with debug logging"
	@echo "  ci             - Full CI pipeline"
	@echo "  help           - Show this help"


.PHONY: all build release package install clean test help

# Default target
all: build

# Build in debug mode
build:
	@echo "🔨 Building JxPoolMiner (debug)..."
	@. "$$HOME/.cargo/env" 2>/dev/null || true && cargo build

# Build in release mode
release:
	@echo "🔨 Building JxPoolMiner (release)..."
	@. "$$HOME/.cargo/env" 2>/dev/null || true && cargo build --release

# Create installable package
package: release
	@echo "📦 Creating installable package..."
	@. "$$HOME/.cargo/env" 2>/dev/null || true && ./package.sh

# Install locally
install: release
	@echo "📥 Installing JxPoolMiner..."
	@if [ "$$(uname)" = "Linux" ]; then \
		mkdir -p $$HOME/.local/bin; \
		cp target/release/jxpoolminer $$HOME/.local/bin/; \
		chmod +x $$HOME/.local/bin/jxpoolminer; \
		echo "✅ Installed to $$HOME/.local/bin/jxpoolminer"; \
	elif [ "$$(uname)" = "Darwin" ]; then \
		mkdir -p /usr/local/bin; \
		cp target/release/jxpoolminer /usr/local/bin/; \
		chmod +x /usr/local/bin/jxpoolminer; \
		echo "✅ Installed to /usr/local/bin/jxpoolminer"; \
	else \
		echo "❌ Unsupported platform"; \
		exit 1; \
	fi

# Run tests
test:
	@echo "🧪 Running tests..."
	@. "$$HOME/.cargo/env" 2>/dev/null || true && cargo test

# Run with logging
run:
	@echo "🚀 Running JxPoolMiner..."
	@. "$$HOME/.cargo/env" 2>/dev/null || true && cargo run --release

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@. "$$HOME/.cargo/env" 2>/dev/null || true && cargo clean
	@rm -rf dist/

# Check code
check:
	@echo "🔍 Checking code..."
	@. "$$HOME/.cargo/env" 2>/dev/null || true && cargo check

# Format code
fmt:
	@echo "✨ Formatting code..."
	@. "$$HOME/.cargo/env" 2>/dev/null || true && cargo fmt

# Lint code
lint:
	@echo "🔎 Linting code..."
	@. "$$HOME/.cargo/env" 2>/dev/null || true && cargo clippy -- -D warnings

# Show help
help:
	@echo "JxPoolMiner Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  make build      - Build in debug mode"
	@echo "  make release    - Build in release mode"
	@echo "  make package    - Create installable package"
	@echo "  make install    - Install locally"
	@echo "  make test       - Run tests"
	@echo "  make run        - Run the application"
	@echo "  make clean      - Clean build artifacts"
	@echo "  make check      - Check code without building"
	@echo "  make fmt        - Format code"
	@echo "  make lint       - Lint code with clippy"
	@echo "  make help       - Show this help message"

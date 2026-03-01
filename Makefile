.PHONY: build check test fmt fmt-check lint clippy run clean set-version build-release coverage

# =============================================================================
# Build Commands
# =============================================================================

build:
	cargo build

build-release:
	cargo build --release

check:
	cargo check

test:
	cargo test --all-features

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

lint: clippy

clippy-check:
	cargo clippy --all-targets --all-features -- -D warnings

run:
	cargo run

coverage:
	cargo test --no-run
	mkdir -p target/cov
	for file in target/debug/deps/*-*; do \
		[ -x "$${file}" ] || continue; \
		[ "$${file}" = "$${file%.d}" ] || continue; \
		mkdir -p "target/cov/$$(basename $$file)"; \
		kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$$(basename $$file)" "$$file" || true; \
	done

clean:
	cargo clean

set-version:
	@if [ -z "$(VERSION)" ]; then \
		echo "Error: VERSION is required. Usage: make set-version VERSION=1.2.3"; \
		exit 1; \
	fi
	cargo install cargo-edit@0.12.3 --quiet --locked
	cargo set-version $(VERSION)
	cargo update --workspace
	@echo "Version set to $(VERSION) and Cargo.lock updated"

# =============================================================================
# Local Development & Testing
# =============================================================================
# These commands use the debug build (cargo run) for faster iteration.
# They assume test-config.toml exists in the project root.
# =============================================================================

# Build and run all CLI commands to test the entire CLI
test-all: build
	@echo "=== Testing init commands ==="
	@cargo run -- --config test-config.toml init init --force
	@echo ""
	@echo "=== Testing index commands ==="
	@cargo run -- --config test-config.toml index index --force
	@echo ""
	@echo "=== Testing query commands ==="
	@cargo run -- --config test-config.toml query search "productivity"
	@cargo run -- --config test-config.toml query backlinks "Home"
	@cargo run -- --config test-config.toml query links "Home"
	@cargo run -- --config test-config.toml query unresolved
	@cargo run -- --config test-config.toml query tags
	@cargo run -- --config test-config.toml query tags --list
	@echo ""
	@echo "=== Testing analyze commands ==="
	@cargo run -- --config test-config.toml analyze bloat --threshold 50000
	@cargo run -- --config test-config.toml analyze related "Home" --limit 5
	@echo ""
	@echo "=== Testing diagnose commands ==="
	@cargo run -- --config test-config.toml diagnose orphans
	@cargo run -- --config test-config.toml diagnose broken-links
	@echo ""
	@echo "=== Testing view commands ==="
	@cargo run -- --config test-config.toml view stats
	@cargo run -- --config test-config.toml view describe "Home"
	@echo ""
	@echo "=== All CLI commands tested successfully ==="

# =============================================================================
# Sanity Tests - Quick smoke tests using release build
# =============================================================================

# Full sanity: init, index, search, stats
init-sanity: build-release
	./target/release/obsidian-cli-inspector --config test-config.toml init init
	./target/release/obsidian-cli-inspector --config test-config.toml index index
	./target/release/obsidian-cli-inspector --config test-config.toml query search "productivity"
	./target/release/obsidian-cli-inspector --config test-config.toml view stats

# Search sanity: init, index, search
search-sanity: build-release
	./target/release/obsidian-cli-inspector --config test-config.toml init init
	./target/release/obsidian-cli-inspector --config test-config.toml index index
	./target/release/obsidian-cli-inspector --config test-config.toml query search "productivity"

# Links sanity: init, index, links, backlinks
links-sanity: build-release
	./target/release/obsidian-cli-inspector --config test-config.toml init init
	./target/release/obsidian-cli-inspector --config test-config.toml index index
	./target/release/obsidian-cli-inspector --config test-config.toml query links "Home.md"
	./target/release/obsidian-cli-inspector --config test-config.toml query backlinks "Home.md"

# Tags sanity: init, index, tags
tags-sanity: build-release
	./target/release/obsidian-cli-inspector --config test-config.toml init init
	./target/release/obsidian-cli-inspector --config test-config.toml index index
	./target/release/obsidian-cli-inspector --config test-config.toml query tags learning

# =============================================================================
# Individual Development Commands (using cargo run for debug build)
# =============================================================================

# Initialize database (drops existing data)
init:
	cargo run -- --config test-config.toml init init --force

# Reinitialize database without forcing
init-noforce:
	cargo run -- --config test-config.toml init init

# Index vault
index:
	cargo run -- --config test-config.toml index index

# Index with verbose output
index-verbose:
	cargo run -- --config test-config.toml index index --verbose

# Index with force (full re-index)
index-force:
	cargo run -- --config test-config.toml index index --force

# Search notes
search:
	cargo run -- --config test-config.toml query search "$(QUERY)"

# Search with custom limit
search-limit:
	cargo run -- --config test-config.toml query search "$(QUERY)" --limit 10

# List backlinks to a note
backlinks:
	cargo run -- --config test-config.toml query backlinks "$(NOTE)"

# List forward links from a note
links:
	cargo run -- --config test-config.toml query links "$(NOTE)"

# List unresolved links
unresolved:
	cargo run -- --config test-config.toml query unresolved

# List all tags
tags:
	cargo run -- --config test-config.toml query tags

# List tags with --list flag
tags-list:
	cargo run -- --config test-config.toml query tags --list

# Find notes by specific tag
tags-specific:
	cargo run -- --config test-config.toml query tags "$(TAG)"

# Analyze bloat (large notes)
bloat:
	cargo run -- --config test-config.toml analyze bloat

# Analyze bloat with custom threshold
bloat-threshold:
	cargo run -- --config test-config.toml analyze bloat --threshold 50000

# Find related notes
related:
	cargo run -- --config test-config.toml analyze related "$(NOTE)"

# Find related notes with custom limit
related-limit:
	cargo run -- --config test-config.toml analyze related "$(NOTE)" --limit 5

# Diagnose orphan notes
diagnose-orphans:
	cargo run -- --config test-config.toml diagnose orphans

# Diagnose orphan notes (excluding templates)
diagnose-orphans-no-templates:
	cargo run -- --config test-config.toml diagnose orphans --exclude-templates

# Diagnose orphan notes (excluding templates and daily notes)
diagnose-orphans-full:
	cargo run -- --config test-config.toml diagnose orphans --exclude-templates --exclude-daily

# Diagnose broken links
diagnose-broken-links:
	cargo run -- --config test-config.toml diagnose broken-links

# View vault statistics
stats:
	cargo run -- --config test-config.toml view stats

# Describe a note
describe:
	cargo run -- --config test-config.toml view describe "$(NOTE)"

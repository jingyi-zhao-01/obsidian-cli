.PHONY: build check test fmt fmt-check lint clippy run clean set-version build-release coverage

# Note: E2E tests are located in e2e/Makefile

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

# =============================================================================
# Machine Contract Commands (JSON output for agent integration)
# =============================================================================

# Search with JSON output
search-json:
	cargo run -- --output json --config test-config.toml query search "$(QUERY)"

# Backlinks with JSON output
backlinks-json:
	cargo run -- --output json --config test-config.toml query backlinks "$(NOTE)"

# Links with JSON output
links-json:
	cargo run -- --output json --config test-config.toml query links "$(NOTE)"

# Unresolved links with JSON output
unresolved-json:
	cargo run -- --output json --config test-config.toml query unresolved

# Tags with JSON output
tags-json:
	cargo run -- --output json --config test-config.toml query tags

# Tags list with JSON output
tags-list-json:
	cargo run -- --output json --config test-config.toml query tags --list

# Bloat analysis with JSON output
bloat-json:
	cargo run -- --output json --config test-config.toml analyze bloat

# Related notes with JSON output
related-json:
	cargo run -- --output json --config test-config.toml analyze related "$(NOTE)"

# Orphan diagnosis with JSON output
orphans-json:
	cargo run -- --output json --config test-config.toml diagnose orphans

# Broken links diagnosis with JSON output
broken-links-json:
	cargo run -- --output json --config test-config.toml diagnose broken-links

# Stats with JSON output
stats-json:
	cargo run -- --output json --config test-config.toml view stats

# Describe with JSON output
describe-json:
	cargo run -- --output json --config test-config.toml view describe "$(NOTE)"

.PHONY: build check test fmt fmt-check lint clippy run clean set-version sanity build-release coverage

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
	@echo "✅ Version set to $(VERSION) and Cargo.lock updated"

	 


init-sanity: build-release
	./target/release/obsidian-cli-inspector --config test-config.toml init
	./target/release/obsidian-cli-inspector --config test-config.toml index
	./target/release/obsidian-cli-inspector --config test-config.toml search "productivity"
	./target/release/obsidian-cli-inspector --config test-config.toml stats


search-sanity: build-release
	./target/release/obsidian-cli-inspector --config test-config.toml init
	./target/release/obsidian-cli-inspector --config test-config.toml index
	./target/release/obsidian-cli-inspector --config test-config.toml search "productivity"

links-sanity: build-release
	./target/release/obsidian-cli-inspector --config test-config.toml init
	./target/release/obsidian-cli-inspector --config test-config.toml index
	./target/release/obsidian-cli-inspector --config test-config.toml links "Home.md"
	./target/release/obsidian-cli-inspector --config test-config.toml backlinks "Home.md"
	./target/release/obsidian-cli-inspector --config test-config.toml unresolved-links


tags-sanity: build-release
	./target/release/obsidian-cli-inspector --config test-config.toml init
	./target/release/obsidian-cli-inspector --config test-config.toml index
	./target/release/obsidian-cli-inspector --config test-config.toml tags learning
	./target/release/obsidian-cli-inspector --config test-config.toml tags --all



init: 
	cargo run -- --config test-config.toml init


index:
	cargo run -- --config test-config.toml index


search:
	cargo run -- --config test-config.toml search "$(QUERY)"

diagnose-orphans:
	cargo run -- --config test-config.toml diagnose-orphans

diagnose-orphans-with-flags:
	cargo run -- --config test-config.toml diagnose-orphans --exclude-templates --exclude-daily

diagnose-broken-links:
	cargo run -- --config test-config.toml diagnose-broken-links






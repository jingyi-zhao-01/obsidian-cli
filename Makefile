.PHONY: build check test fmt lint clippy run clean sanity build-release

build:
	cargo build

build-release:
	cargo build --release

check:
	cargo check

test:
	cargo test

fmt:
	cargo fmt

lint: clippy

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

run:
	cargo run

clean:
	cargo clean

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







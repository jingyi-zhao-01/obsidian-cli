.PHONY: test-all init-sanity search-sanity links-sanity tags-sanity test-machine-contract machine-contract-search machine-contract-query machine-contract-analyze machine-contract-diagnose machine-contract-view

# =============================================================================
# E2E Tests - End-to-end CLI tests
# =============================================================================
# These tests exercise the full CLI workflow using test-config.toml
# =============================================================================

# Full E2E: all CLI commands exercised (uses debug build for faster iteration)
test-cli:
	@cd .. && cargo build
	@echo "=== Testing init commands ==="
	@cd .. && cargo run -- --config test-config.toml init init --force
	@echo ""
	@echo "=== Testing index commands ==="
	@cd .. && cargo run -- --config test-config.toml index index --force
	@echo ""
	@echo "=== Testing query commands ==="
	@cd .. && cargo run -- --config test-config.toml query search "productivity"
	@cd .. && cargo run -- --config test-config.toml query backlinks "Home"
	@cd .. && cargo run -- --config test-config.toml query links "Home"
	@cd .. && cargo run -- --config test-config.toml query unresolved
	@cd .. && cargo run -- --config test-config.toml query tags
	@cd .. && cargo run -- --config test-config.toml query tags --list
	@echo ""
	@echo "=== Testing analyze commands ==="
	@cd .. && cargo run -- --config test-config.toml analyze bloat --threshold 50000
	@cd .. && cargo run -- --config test-config.toml analyze related "Home" --limit 5
	@echo ""
	@echo "=== Testing diagnose commands ==="
	@cd .. && cargo run -- --config test-config.toml diagnose orphans
	@cd .. && cargo run -- --config test-config.toml diagnose broken-links
	@echo ""
	@echo "=== Testing view commands ==="
	@cd .. && cargo run -- --config test-config.toml view stats
	@cd .. && cargo run -- --config test-config.toml view describe "Home"
	@echo ""
	@echo "=== All CLI commands tested successfully ==="

# Sanity Tests - Quick smoke tests using release build

# Full sanity: init, index, search, stats
init-sanity:
	@cd .. && cargo build --release
	@./target/release/obsidian-cli-inspector --config test-config.toml init init
	@./target/release/obsidian-cli-inspector --config test-config.toml index index
	@./target/release/obsidian-cli-inspector --config test-config.toml query search "productivity"
	@./target/release/obsidian-cli-inspector --config test-config.toml view stats

# Search sanity: init, index, search
search-sanity:
	@cd .. && cargo build --release
	@./target/release/obsidian-cli-inspector --config test-config.toml init init
	@./target/release/obsidian-cli-inspector --config test-config.toml index index
	@./target/release/obsidian-cli-inspector --config test-config.toml query search "productivity"

# Links sanity: init, index, links, backlinks
links-sanity:
	@cd .. && cargo build --release
	@./target/release/obsidian-cli-inspector --config test-config.toml init init
	@./target/release/obsidian-cli-inspector --config test-config.toml index index
	@./target/release/obsidian-cli-inspector --config test-config.toml query links "Home.md"
	@./target/release/obsidian-cli-inspector --config test-config.toml query backlinks "Home.md"

# Tags sanity: init, index, tags
tags-sanity:
	@cd .. && cargo build --release
	@./target/release/obsidian-cli-inspector --config test-config.toml init init
	@./target/release/obsidian-cli-inspector --config test-config.toml index index
	@./target/release/obsidian-cli-inspector --config test-config.toml query tags learning

# =============================================================================
# Machine Contract Tests - JSON output for agent integration
# =============================================================================
# Tests JSON output mode (--json flag) to verify machine contracts work
# =============================================================================

# Full machine contract test: all commands with JSON output
test-machine-contract:
	@cd .. && cargo build
	@echo "=== Testing machine contracts (JSON output) ==="
	@cd .. && cargo run -- --config test-config.toml init init --force
	@cd .. && cargo run -- --config test-config.toml index index --force
	@echo ""
	@echo "=== Testing JSON output for query commands ==="
	@cd .. && cargo run -- --output json --config test-config.toml query search "productivity"
	@cd .. && cargo run -- -o json --config test-config.toml query backlinks "Home"
	@cd .. && cargo run -- --output json --config test-config.toml query links "Home"
	@cd .. && cargo run -- --output json --config test-config.toml query unresolved
	@cd .. && cargo run -- --output json --config test-config.toml query tags
	@cd .. && cargo run -- --output json --config test-config.toml query tags --list
	@echo ""
	@echo "=== Testing JSON output for analyze commands ==="
	@cd .. && cargo run -- --output json --config test-config.toml analyze bloat --threshold 50000
	@cd .. && cargo run -- --output json --config test-config.toml analyze related "Home" --limit 5
	@echo ""
	@echo "=== Testing JSON output for diagnose commands ==="
	@cd .. && cargo run -- --output json --config test-config.toml diagnose orphans
	@cd .. && cargo run -- --output json --config test-config.toml diagnose broken-links
	@echo ""
	@echo "=== Testing JSON output for view commands ==="
	@cd .. && cargo run -- --output json --config test-config.toml view stats
	@cd .. && cargo run -- --output json --config test-config.toml view describe "Home"
	@echo ""
	@echo "=== All machine contract tests passed ==="

# Machine contract search tests
machine-contract-search:
	@cd .. && cargo build --release
	@./target/release/obsidian-cli-inspector --config test-config.toml init init
	@./target/release/obsidian-cli-inspector --config test-config.toml index index
	@./target/release/obsidian-cli-inspector --output json --config test-config.toml query search "productivity"
	@./target/release/obsidian-cli-inspector -o json --config test-config.toml query search "productivity"

# Machine contract query tests
machine-contract-query:
	@cd .. && cargo build --release
	@./target/release/obsidian-cli-inspector --config test-config.toml init init
	@./target/release/obsidian-cli-inspector --config test-config.toml index index
	@./target/release/obsidian-cli-inspector --output json --config test-config.toml query backlinks "Home"
	@./target/release/obsidian-cli-inspector --output json --config test-config.toml query links "Home"
	@./target/release/obsidian-cli-inspector --output json --config test-config.toml query unresolved
	@./target/release/obsidian-cli-inspector --output json --config test-config.toml query tags

# Machine contract analyze tests
machine-contract-analyze:
	@cd .. && cargo build --release
	@./target/release/obsidian-cli-inspector --config test-config.toml init init
	@./target/release/obsidian-cli-inspector --config test-config.toml index index
	@./target/release/obsidian-cli-inspector --output json --config test-config.toml analyze bloat
	@./target/release/obsidian-cli-inspector --output json --config test-config.toml analyze related "Home"

# Machine contract diagnose tests
machine-contract-diagnose:
	@cd .. && cargo build --release
	@./target/release/obsidian-cli-inspector --config test-config.toml init init
	@./target/release/obsidian-cli-inspector --config test-config.toml index index
	@./target/release/obsidian-cli-inspector --output json --config test-config.toml diagnose orphans
	@./target/release/obsidian-cli-inspector --output json --config test-config.toml diagnose broken-links

# Machine contract view tests
machine-contract-view:
	@cd .. && cargo build --release
	@./target/release/obsidian-cli-inspector --config test-config.toml init init
	@./target/release/obsidian-cli-inspector --config test-config.toml index index
	@./target/release/obsidian-cli-inspector --output json --config test-config.toml view stats
	@./target/release/obsidian-cli-inspector --output json --config test-config.toml view describe "Home"

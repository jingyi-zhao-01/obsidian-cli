# Obsidian CLI Inspector


- Note: THIS Feature currently is in beta


[![GitHub](https://img.shields.io/badge/GitHub-obsidian--cli-black?logo=github)](https://github.com/jingyi-zhao-01/obsidian-cli)
[![Crates.io](https://img.shields.io/crates/v/obsidian-cli-inspector.svg)](https://crates.io/crates/obsidian-cli-inspector)
[![Build](https://github.com/jingyi-zhao-01/obsidian-cli/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/jingyi-zhao-01/obsidian-cli/actions/workflows/build.yml)
[![codecov](https://codecov.io/gh/jingyi-zhao-01/obsidian-cli-inspector/branch/master/graph/badge.svg?token=4C0B7CEC8C)](https://codecov.io/gh/jingyi-zhao-01/obsidian-cli-inspector)


A local-first, read-only CLI/TUI for inspecting and querying Obsidian vaults. It helps developers quickly find notes, links, and tags without leaving the terminal.

## What you can do

- Search your vault with fast full‑text queries
- Navigate backlinks, forward links, and unresolved links
- Filter notes by tags (AND/OR)
- Explore note relationships via graph view
- Get suggestions for related notes
- Surface large/bloated notes for cleanup
- Use CLI for scripting or TUI for interactive browsing

## Install

### From crates.io

```bash
cargo install obsidian-cli-inspector
```

### From source

```bash
git clone https://github.com/jingyi-zhao-01/obsidian-cli.git
cd obsidian-cli
cargo build --release
```

The binary will be at `target/release/obsidian-cli-inspector`.

## Quick start

1) Create a config file and set your vault path:

```bash
mkdir -p ~/.config/obsidian-cli
cp config.toml.example ~/.config/obsidian-cli/config.toml
```

```toml
vault_path = "/path/to/your/obsidian/vault"
```

2) Initialize and index your vault:

```bash
obsidian-cli-inspector init
obsidian-cli-inspector index
```

3) Explore your notes:

```bash
obsidian-cli-inspector search "your query"
obsidian-cli-inspector backlinks "Note Name"
obsidian-cli-inspector tags productivity
obsidian-cli-inspector stats
```

## Common commands

```bash
obsidian-cli-inspector init [--force]
obsidian-cli-inspector index [--dry-run] [--force] [--verbose]
obsidian-cli-inspector search "query" [--limit 20]
obsidian-cli-inspector backlinks "Note Name"
obsidian-cli-inspector links "Note Name"
obsidian-cli-inspector unresolved-links
obsidian-cli-inspector tags [tag-name] [--all]
obsidian-cli-inspector suggest "Note Name" [--limit 10]
obsidian-cli-inspector bloat [--threshold 50000] [--limit 10]
obsidian-cli-inspector stats
obsidian-cli-inspector tui
obsidian-cli-inspector graph ["Note Name"] [--depth 2]
```

## Configuration

The minimum required setting is `vault_path`. Optional settings include database location, exclusions, and defaults. See [config.toml.example](config.toml.example) for a complete list.

## Status

The core indexing, parsing, chunking, and query features are in place. Advanced recommendations and hygiene features are evolving. See [docs/TODOs.md](docs/TODOs.md) for a user‑focused roadmap.

## License

Apache-2.0

## Contributing

Contributions are welcome. Please open an issue to discuss larger changes.

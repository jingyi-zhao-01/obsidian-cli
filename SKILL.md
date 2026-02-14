---
name: obsidian-cli-inspector
description: Provision the obsidian-cli-inspector skill and run the CLI via npm wrapper.
disable-model-invocation: true
---

1) Install the Rust CLI (required by the wrapper):
- `cargo install obsidian-cli-inspector`

Global options
- `-c, --config <FILE>` Path to config file
- `-h, --help` Print help
- `-V, --version` Print version

Commands
- `obsidian-cli-inspector init` Initialize or reinitialize the database
- `obsidian-cli-inspector index` Index the vault (scan and parse all files)
- `obsidian-cli-inspector search` Search notes using full-text search
- `obsidian-cli-inspector backlinks` List backlinks to a note
- `obsidian-cli-inspector links` List forward links from a note
- `obsidian-cli-inspector unresolved-links` List all unresolved links in the vault
- `obsidian-cli-inspector tags` List notes by tag
- `obsidian-cli-inspector suggest` Suggest related notes not directly linked
- `obsidian-cli-inspector bloat` Detect bloated notes and suggest refactoring
- `obsidian-cli-inspector stats` Show statistics about the vault
- `obsidian-cli-inspector tui` Launch interactive TUI
- `obsidian-cli-inspector graph` Display vault graph information
- `obsidian-cli-inspector help` Print this message or the help of the given subcommand(s)


If the CLI binary is missing, install it before running commands.

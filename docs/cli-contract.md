# CLI Contract

This document describes the CLI interface for Obsidian CLI Inspector.

## Command Structure

The CLI uses a grouped command pattern:

```
obsidian-cli-inspector <group> <function> [args...]
```

## Command Groups

| Group | Description |
|-------|-------------|
| `init` | Database initialization |
| `index` | Vault indexing |
| `query` | Search and retrieval |
| `analyze` | Content analysis |
| `diagnose` | Diagnostics |
| `view` | Display commands |

## Commands

### init

Initialize or reinitialize the database.

```bash
obsidian-cli-inspector init init [--force]
```

| Option | Description |
|--------|-------------|
| `--force`, `-f` | Force reinitialization (drops existing data) |

### index

Index the vault.

```bash
obsidian-cli-inspector index index [--dry-run] [--force] [--verbose]
```

| Option | Description |
|--------|-------------|
| `--dry-run`, `-n` | Perform a dry run without writing to database |
| `--force`, `-f` | Force full re-index (ignores change detection) |
| `--verbose`, `-v` | Show verbose output |

### query

Search and retrieval commands.

```bash
# Search notes using full-text search
obsidian-cli-inspector query search <query> [--limit <n>]

# List backlinks to a note
obsidian-cli-inspector query backlinks <note>

# List forward links from a note
obsidian-cli-inspector query links <note>

# List all unresolved links in the vault
obsidian-cli-inspector query unresolved

# List notes by tag
obsidian-cli-inspector query tags [<tag>] [--list]
```

| Option | Description |
|--------|-------------|
| `--limit`, `-l` | Maximum number of results (default: 20) |
| `--list`, `-l` | List all tags if no tag specified |

### analyze

Content analysis commands.

```bash
# Detect bloated notes
obsidian-cli-inspector analyze bloat [--threshold <n>] [--limit <n>]

# Suggest related notes
obsidian-cli-inspector analyze related <note> [--limit <n>]
```

| Option | Description |
|--------|-------------|
| `--threshold`, `-t` | Minimum size threshold in bytes (default: 50000) |
| `--limit`, `-l` | Maximum number of results (default: 10) |

### diagnose

Diagnostic commands.

```bash
# Diagnose orphan notes
obsidian-cli-inspector diagnose orphans [--exclude-templates] [--exclude-daily]

# Diagnose broken links
obsidian-cli-inspector diagnose broken-links
```

| Option | Description |
|--------|-------------|
| `--exclude-templates` | Exclude template notes |
| `--exclude-daily` | Exclude daily notes |

### view

Display commands.

```bash
# Show statistics about the vault
obsidian-cli-inspector view stats

# Describe file metadata
obsidian-cli-inspector view describe <filename>
```

### tui

Launch interactive TUI.

```bash
obsidian-cli-inspector tui
```

## Examples

```bash
# Initialize and index your vault
obsidian-cli-inspector init init
obsidian-cli-inspector index index

# Search for notes
obsidian-cli-inspector query search rust --limit 10
obsidian-cli-inspector query backlinks "Project Ideas"
obsidian-cli-inspector query tags work
obsidian-cli-inspector query tags --list

# Analyze content
obsidian-cli-inspector analyze bloat --threshold 50000
obsidian-cli-inspector analyze related "Home" --limit 10

# View information
obsidian-cli-inspector view stats
obsidian-cli-inspector view describe "note.md"

# Diagnostics
obsidian-cli-inspector diagnose orphans
obsidian-cli-inspector diagnose broken-links

# Interactive mode
obsidian-cli-inspector tui
```

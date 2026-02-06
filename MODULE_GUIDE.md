# Module Organization Guide

## Quick Navigation

### Core Modules

| Module | Purpose | Key Files |
|--------|---------|-----------|
| **parser** | Parse markdown content | `parser/mod.rs`, `wikilink.rs`, `markdown.rs` |
| **chunker** | Split text into chunks | `chunker/mod.rs`, `heading.rs`, `paragraph.rs` |
| **db** | Database operations | `db/mod.rs`, `operations.rs`, `schema.rs` |
| **commands** | Command handlers | `commands/init.rs`, `stats.rs`, `index.rs` |
| **cli** | CLI argument parsing | `cli.rs` |
| **config** | Configuration management | `config.rs` |
| **scanner** | Vault file scanning | `scanner.rs` |
| **logger** | Logging utilities | `logger.rs` |

## Adding a New Feature

### Adding a New Command

1. Create `src/commands/your_command.rs`:
```rust
use crate::config::Config;
use crate::logger::Logger;
use anyhow::Result;

pub fn execute_your_command(config: &Config, logger: Option<&Logger>) -> Result<()> {
    // Implementation
    Ok(())
}
```

2. Add export to `src/commands/mod.rs`:
```rust
pub mod your_command;
pub use your_command::execute_your_command;
```

3. Add to CLI in `src/cli.rs`:
```rust
#[derive(Subcommand)]
pub enum Commands {
    /// Your command description
    YourCommand { arg: String },
}
```

4. Add dispatch in `src/main.rs`:
```rust
Commands::YourCommand { arg } => {
    let config = load_config(cli.config)?;
    execute_your_command(&config, arg, logger.as_ref())
}
```

### Adding a New Parser Type

1. Create `src/parser/your_parser.rs`:
```rust
use super::{Link, LinkType};

pub fn extract_your_links(content: &str) -> Vec<Link> {
    // Parse implementation
    Vec::new()
}
```

2. Add export and integration to `src/parser/mod.rs`:
```rust
mod your_parser;
use your_parser::extract_your_links;

// In extract_links():
fn extract_links(content: &str) -> Vec<Link> {
    let mut links = extract_wikilinks(content);
    links.extend(extract_markdown_links(content));
    links.extend(extract_your_links(content));  // Add here
    links
}
```

### Adding a New Chunking Strategy

1. Create `src/chunker/your_strategy.rs`:
```rust
use super::Chunk;

pub fn chunk_by_your_strategy(
    content: &str,
    heading_path: Option<&str>,
    base_offset: usize,
) -> Vec<Chunk> {
    // Strategy implementation
    Vec::new()
}
```

2. Use in `src/chunker/mod.rs`:
```rust
mod your_strategy;
use your_strategy::chunk_by_your_strategy;

// In chunk() or chunk_by_paragraphs():
if some_condition {
    return chunk_by_your_strategy(content, heading_path, base_offset);
}
```

## Module Responsibilities

### parser/
Responsible for:
- Extracting structured data from markdown
- Finding and parsing wikilinks
- Finding and parsing markdown links
- Extracting frontmatter
- Extracting tags from content
- Extracting titles

**Does NOT handle**:
- Database operations
- File I/O
- Command logic

### chunker/
Responsible for:
- Splitting text into retrievable chunks
- Maintaining heading hierarchy
- Managing chunk boundaries
- Calculating overlap

**Does NOT handle**:
- Parsing content
- Database operations
- Link resolution

### db/
Responsible for:
- Opening/managing database connections
- Creating schema
- CRUD operations on notes, links, tags, chunks
- Computing statistics

**Does NOT handle**:
- File I/O
- Content parsing
- Vault scanning

### commands/
Responsible for:
- Orchestrating operations for each command
- Coordinating between modules
- Logging and user communication

**Does NOT handle**:
- CLI parsing
- Module implementation

### cli/
Responsible for:
- Defining command structure
- Parsing arguments

**Does NOT handle**:
- Command execution
- Configuration loading

## Testing

### Unit Tests

Located in each module (usually at the end):

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Test implementation
    }
}
```

Run tests:
```bash
cargo test --lib          # Run all library tests
cargo test --lib parser   # Run parser tests only
cargo test                # Run all tests
```

### Adding Tests

1. Add test to the module that contains the code
2. Use `#[test]` attribute
3. Import any helper functions needed
4. Write test assertions

Example:
```rust
#[test]
fn test_my_feature() {
    let result = my_function("input");
    assert_eq!(result, "expected");
}
```

## Dependency Graph

```
main.rs
  ├─→ cli.rs (argument parsing)
  ├─→ config.rs (configuration)
  ├─→ logger.rs (logging)
  └─→ commands/*
       ├─→ parser/ (parse content)
       ├─→ chunker/ (split content)
       ├─→ db/ (store data)
       └─→ scanner/ (find files)

parser/
  ├─→ wikilink.rs
  └─→ markdown.rs

chunker/
  ├─→ heading.rs
  ├─→ paragraph.rs
  ├─→ overlap.rs
  └─→ chunk.rs (data struct)

db/
  ├─→ schema.rs (DDL)
  ├─→ operations.rs (DML)
  └─→ stats.rs (queries)
```

## Common Tasks

### Debug a Parse Issue
1. Look at `parser/mod.rs` for main logic
2. Check `parser/wikilink.rs` or `parser/markdown.rs` for specific type
3. Add test case to reproduce
4. Fix and verify test passes

### Add Database Migration
1. Update schema in `db/schema.rs`
2. Update operations in `db/operations.rs` if needed
3. Run `cargo build` to verify
4. Test with `obsidian-cli init`

### Improve Chunking
1. Review `chunker/mod.rs` for main flow
2. Modify `chunker/heading.rs` or `chunker/paragraph.rs`
3. Add test in `chunker/mod.rs::tests`
4. Run `cargo test --lib chunker`

### Add New Command
1. Create `commands/your_cmd.rs`
2. Add to `commands/mod.rs`
3. Update `cli.rs` with new subcommand
4. Add dispatch in `main.rs`
5. Test with `cargo run`

## Best Practices

1. **Keep modules focused** - Each module should have a single responsibility
2. **Use public APIs** - Import from parent module, not submodules
3. **Test as you code** - Add unit tests alongside implementation
4. **Document complex logic** - Comments explain the "why"
5. **Follow the structure** - New features go in appropriate modules
6. **Run tests before committing** - `cargo test` should pass

## Troubleshooting

### Import not found
- Check that module is exported in `lib.rs`
- Check that submodule is declared in parent `mod.rs`
- Verify path is correct (relative vs absolute)

### Compilation error
- Run `cargo build` to see full error
- Check module visibility (pub keyword)
- Check for circular dependencies

### Test failure
- Run specific test: `cargo test --lib test_name`
- Add `println!` for debugging (appears with `cargo test -- --nocapture`)
- Check test assumptions

---

**Last Updated**: February 5, 2026

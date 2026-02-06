# Phase 2 Implementation Log

This document records what was implemented for Phase 2 (Obsidian Markdown Parsing) and how to validate it. The OKR checklist remains in [TODOs.md](TODOs.md).

## What was implemented

- Markdown link parsing for `[text](path)` including heading (`#heading`) and block (`#^block`) fragments.
- Note identifier normalization:
  - Trim whitespace
  - Remove `./` prefix
  - Strip `.md` extension
  - Normalize path separators from `\\` to `/`
- Link type tracking so the database stores `wikilink` vs `markdown` correctly.
- Parser tests added for markdown links and normalization.

## How to validate

### 1) Parser unit tests
```bash
cargo test parser::tests -- --nocapture
```
Expected output (example):
```
running 4 tests
test parser::tests::test_parse_wikilink_simple ... ok
test parser::tests::test_parse_wikilink_with_alias ... ok
test parser::tests::test_parse_markdown_link_basic ... ok
test parser::tests::test_normalize_note_identifier ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 2) Link type storage
```bash
cargo run -- --config test-config.toml init
cargo run -- --config test-config.toml index
sqlite3 test.db "SELECT kind, COUNT(*) FROM links GROUP BY kind;"
```
Expected output (example):
```
markdown|1
wikilink|115
```
Note: `markdown` appears only if your notes contain `[text](path)` links.


### Verification Steps
1.  Parse a test file with all wikilink variants:
   ```bash
   cargo test parser::tests -- --nocapture
   # Tests pass 
   ```

2.  Verify frontmatter extraction:
   ```bash
   cargo run -- --config test-config.toml index
   sqlite3 test.db "SELECT tag, COUNT(*) FROM tags GROUP BY tag;"
   # Result: 10 distinct tags (productivity, learning, focus, etc.) 
   ```

3.  Test unresolved link detection:
   ```bash
   sqlite3 test.db "SELECT COUNT(*) FROM links WHERE dst_note_id IS NULL;"
   # Result: 115 (link resolution not yet implemented - all tracked as unresolved) 
   ```

4.  Verify parsed links count:
   ```bash
   sqlite3 test.db "SELECT COUNT(*) FROM links;"
   # Result: 115 wikilinks from test vault 
   ```
   ```bash
   sqlite3 test.db "SELECT COUNT(*) FROM links;"
   # Should find ~105 wikilinks from test vault
   ```

5.  Validate markdown link parsing:
   ```bash
   cargo test parser::tests -- --nocapture
   # Should include test_parse_markdown_link_basic 
   ```

6.  Validate normalization behavior:
   ```bash
   cargo test parser::tests -- --nocapture
   # Should include test_normalize_note_identifier 
   ```

7.  Validate link type storage:
   ```bash
   sqlite3 test.db "SELECT kind, COUNT(*) FROM links GROUP BY kind;"
   # Should show both wikilink and markdown when markdown links exist
# ADR: Phase 5 Query Layer Design

Date: 2026-02-10
Status: Proposed

## Context
Phase 5 introduces a query layer to retrieve notes by full-text search, links, and tags. The database schema and chunking already exist, but there is no focused module for query execution or command handlers for retrieval workflows. We need a clear separation between command handlers and query logic, and predictable response formatting for CLI output.

## Decision
Implement a dedicated query module (`src/query/`) that encapsulates all retrieval logic and exposes functions for:
- Full-text search with BM25 ranking (FTS5)
- Backlinks and forward links
- Unresolved link listing
- Tag list, single tag lookup, AND/OR tag selection

Command handlers will call query functions through a database executor to keep responsibilities separated.

## Design Overview

### Module Layout
- `src/query/mod.rs` exports submodules.
- `src/query/search.rs` handles FTS5 search.
- `src/query/links.rs` handles link relationship queries.
- `src/query/tags.rs` handles tag queries.

### Query Execution
- `Database::conn()` returns a `DatabaseQueryExecutor`.
- `DatabaseQueryExecutor::execute_query` provides scoped access to the SQLite connection.
- Query functions are pure and return typed results.

### Search
- Uses `fts_chunks` with `MATCH` and `bm25` ranking.
- Returns note path/title, heading path, excerpt, and rank.

### Links
- Backlinks: join on `dst_note_id` to find source notes.
- Forward links: left join to include unresolved links.
- Unresolved: `dst_note_id IS NULL` with source note and target text.

### Tags
- `list_tags` returns all distinct tags.
- `get_notes_by_tag` returns notes with aggregated tag list.
- AND selection uses `HAVING COUNT(DISTINCT tag) = ?`.
- OR selection uses `WHERE tag IN (...)`.

## Alternatives Considered
- **Raw SQL in command handlers**: rejected due to poor separation of concerns.
- **ORM**: rejected to keep binary lightweight and queries explicit.

## Consequences
- Query logic is centralized and testable.
- Command handlers are thinner and easier to maintain.
- Adding new query types is straightforward.

## Open Questions
- Should search return highlighted snippets using FTS5 `snippet()`?
- Should tag queries support pagination for large vaults?

## References
- [docs/TODOs.md](../TODOs.md)
- [docs/ADR-5/PHASE5.md](PHASE5.md)
- [docs/ADR-5/phase5.mermaid](phase5.mermaid)

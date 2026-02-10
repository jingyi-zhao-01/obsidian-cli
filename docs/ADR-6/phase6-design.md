# ADR: Phase 6 Incremental Indexing Design

Date: 2026-02-10
Status: Proposed

## Context
Phase 6 focuses on incremental indexing so re-indexing scales with changes only. Current indexing performs a full scan and full insert on every run, which is correct but slow and can duplicate rows if not cleaned up. We need a durable scan state, change detection, deletion cleanup, unresolved link re-resolution, and batched writes. The intended flow is captured in [docs/ADR/phase6.mermaid](phase6.mermaid).

## Decision
Adopt an incremental indexing pipeline with these characteristics:

1. Persist scan state per file (path, mtime, size, hash) in SQLite.
2. Detect changes by comparing current scan to persisted state.
3. Re-index only new/modified files.
4. Remove deleted files and dependent records.
5. Re-resolve unresolved links after each run.
6. Batch database writes in transactions.
7. Provide progress hooks (counts + phases) for CLI feedback.

## Design Overview

### Data Model
Add a scan state table to track file metadata:
- `scan_state(path TEXT PRIMARY KEY, mtime INTEGER, size INTEGER, hash TEXT)`

### Pipeline Stages
1. **Scan**: Walk the vault and collect metadata for markdown files.
2. **Diff**: Classify files into `new`, `modified`, `unchanged`, `deleted`.
3. **Ingest**: For `new`/`modified`, parse, extract, chunk, and upsert.
4. **Remove**: For `deleted`, remove note and dependent rows.
5. **Resolve**: Attempt to resolve previously unresolved links.
6. **Commit**: Write changes in a single transaction.
7. **Persist**: Update `scan_state` with current metadata.
8. **Report**: Emit progress and counts.

### Change Detection Rules
- `new`: path not in `scan_state`.
- `modified`: mtime or size differs, or hash differs if computed.
- `unchanged`: metadata matches.
- `deleted`: in `scan_state` but not in current scan.

### Deletion Semantics
For deleted notes, remove:
- `notes` row
- `links` where `src_note_id` or `dst_note_id` matches
- `tags` rows for note
- `chunks` rows for note
- `fts_chunks` rows via triggers

### Re-resolution of Links
After updates, resolve links by:
- Mapping `links.dst_text` to `notes.id` via normalized note identifier
- Updating `dst_note_id` where a matching note exists

### Batching & Transactions
- Use a single transaction per index run.
- Use prepared statements for inserts/updates.
- Avoid per-row commits.

### Progress Reporting
Expose stage-level progress:
- Scan counts
- Change classification counts
- Ingested notes
- Deleted notes
- Resolved links
- Total duration

## Alternatives Considered
- **Full re-index on every run**: Simple but slow and noisy. Rejected.
- **File watcher daemon**: More responsive but increases runtime complexity. Deferred.

## Consequences
- Faster re-indexing for unchanged vaults.
- Requires schema migration to add `scan_state`.
- More complex indexing logic with transactional handling.

## Open Questions
- Should hash be computed always or only when mtime/size changes?
- Should re-resolution run only if new notes were added?

## References
- [docs/TODOs.md](../TODOs.md)
- [docs/ADR/phase6.mermaid](phase6.mermaid)

# PRD: Phase 6 Incremental Indexing

Date: 2026-02-10
Owner: Obsidian CLI Inspector
Status: Draft

## Problem Statement
Indexing currently performs a full scan and full insert on every run. This is slow for large vaults, leads to duplicated data without cleanup, and prevents responsive iterative workflows. Phase 6 introduces incremental indexing to scale with changes only, while keeping the database consistent and fast.

## Goals
- Re-index time scales with changed files only.
- Deleted files leave no orphaned records.
- Unresolved links are re-resolved when possible.
- Index remains consistent after repeated runs.
- Indexing provides clear progress reporting.

## Non-Goals
- Real-time file watching or background daemon.
- Networked sync or multi-user collaboration.
- UI/UX redesign beyond basic progress output.

## Users & Use Cases
- **Local power users** who re-index frequently after small edits.
- **Large vault users** who require fast incremental runs.

Key use case:
- Run `index` twice with no changes; second run should be near-instant.

## Functional Requirements

### FR1: Change Detection
- Persist scan state per file (path, mtime, size, hash).
- Identify `new`, `modified`, `unchanged`, and `deleted` files.

### FR2: Incremental Re-index
- Parse, chunk, and upsert only `new` or `modified` files.
- Ensure updated files replace prior data without duplication.

### FR3: Delete Cleanup
- Remove data for deleted notes, including tags, links, chunks, and FTS rows.

### FR4: Link Re-resolution
- Resolve previously unresolved links when targets appear.
- Update `dst_note_id` in `links` when a match exists.

### FR5: Batch Writes
- Perform indexing writes in transactions.
- Use prepared statements to reduce per-row overhead.

### FR6: Progress Reporting
- Emit phase-level progress and summary counts.
- Report totals for scanned files, changed files, ingested notes, deleted notes.

## Non-Functional Requirements

### NFR1: Performance
- No-change index run should complete in under 1 second on test vault.
- Re-index time should scale approximately with number of changed files.

### NFR2: Consistency
- Repeated index runs without changes must not alter row counts.
- Database referential integrity must hold after deletions and updates.

### NFR3: Reliability
- If indexing fails mid-run, database should remain consistent (transactional rollback).
- Errors should be surfaced with actionable messages.

### NFR4: Maintainability
- Incremental indexing logic should be testable via unit/integration tests.
- Schema changes must be versioned and migratable.

## Success Metrics
- 0 duplicate rows after repeated indexing runs.
- 100% deletion cleanup verification in tests.
- 100% re-resolution for previously missing links when notes appear.

## Acceptance Criteria
- Running `index` twice with no changes shows near-zero ingestion and no DB changes.
- Deleting a note removes it and all related data.
- Adding a missing note resolves previously unresolved links.
- Progress output includes phase counts and totals.

## Dependencies
- Schema migration for `scan_state` table.
- Indexer changes in scan and ingest pipeline.
- Link resolution utilities.

## Risks
- Incorrect diff logic could skip required re-indexing.
- Large transactions could lock DB longer than expected.

## Open Questions
- Should hash computation be conditional to reduce IO?
- Should link re-resolution be limited to runs with new notes?

## References
- [docs/TODOs.md](TODOs.md)
- [docs/ADR/phase6-design.md](ADR/phase6-design.md)
- [docs/ADR/phase6.mermaid](ADR/phase6.mermaid)

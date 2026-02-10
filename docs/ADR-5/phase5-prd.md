# PRD: Phase 5 Query Layer (Basic Retrieval)

Date: 2026-02-10
Owner: Obsidian CLI Inspector
Status: Draft

## Problem Statement
Users need fast, reliable ways to locate notes by search, links, and tags. The system already indexes notes, but there is no consistent query layer for retrieval and no unified output formatting. Phase 5 provides the base retrieval experience used by CLI commands.

## Goals
- Provide full-text search with ranked results.
- Enable backlinks, forward links, and unresolved link listing.
- Support tag discovery with AND/OR selection.
- Keep query latency under target thresholds for typical vaults.

## Non-Goals
- UI/TUI redesign beyond current CLI output.
- Recommendation or semantic similarity features (Phase 7).
- Advanced QA pipelines.

## Users & Use Cases
- **CLI users** who need to find notes by terms, tags, and relationships.
- **Vault maintainers** who need to locate unresolved links quickly.

## Functional Requirements

### FR1: Full-text Search
- Query `fts_chunks` using `MATCH` and BM25 ranking.
- Return note path/title, heading path, matched excerpt, and rank.

### FR2: Backlinks
- For a given note path, return notes that link to it.

### FR3: Forward Links
- For a given note path, return notes it links to.
- Include unresolved links with a clear marker.

### FR4: Unresolved Links
- List links where the target note does not exist.

### FR5: Tag Discovery
- List all distinct tags.
- List notes by single tag.
- AND intersection of tags.
- OR union of tags.

### FR6: Command Integration
- CLI commands call the query layer and format results consistently.

## Non-Functional Requirements

### NFR1: Performance
- Common queries should complete under 100ms on a representative vault.

### NFR2: Correctness
- Queries must reflect the current index state.
- Forward link queries must include unresolved targets.

### NFR3: Reliability
- Errors should return actionable messages and non-zero exit codes.

### NFR4: Maintainability
- Query logic lives in a dedicated module with typed results.

## Success Metrics
- Search results ranked and displayed with heading context.
- Backlinks/forward links/unresolved outputs match database state.
- Tag queries return expected note sets.

## Acceptance Criteria
- `search` returns ranked results with excerpts.
- `backlinks` and `forwardlinks` resolve correctly.
- `unresolved-links` lists broken targets.
- `list-tags` and tag queries work for single, AND, and OR.

## Dependencies
- FTS5 virtual table `fts_chunks` and triggers.
- `links`, `tags`, and `chunks` tables populated by indexing.

## Risks
- Large vaults may need pagination to keep output readable.
- FTS5 ranking behavior may require tuning for relevance.

## Open Questions
- Should CLI support result paging or limit parameters by default?
- Should search support phrase and prefix queries?

## References
- [docs/TODOs.md](../TODOs.md)
- [docs/ADR-5/PHASE5.md](PHASE5.md)
- [docs/ADR-5/phase5.mermaid](phase5.mermaid)

# ADR: Phase 11 Metadata & Tag Intelligence Design

Date: 2026-02-10
Status: Proposed

## Context
Structured filtering improves planning and automation by enabling precise metadata selection and quality checks.

## Decision
Extend query capabilities to include tag algebra and frontmatter-driven filters.

## Design Overview

### Tag Algebra
- Parse AND/OR/NOT expressions.
- Execute set operations over note IDs.

### Frontmatter Queries
- Filter on key presence and simple scalar values.

### Quality Signals
- Detect missing keys and stale timestamps.

## Alternatives Considered
- **Full query language**: deferred to keep CLI simple.

## Consequences
- More complex query parsing.

## Open Questions
- Which frontmatter value types are supported initially?

## References
- [docs/TODOs.md](../TODOs.md)

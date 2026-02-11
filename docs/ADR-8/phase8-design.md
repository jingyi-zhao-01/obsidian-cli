# ADR: Phase 8 Deterministic Search v2 Design

Date: 2026-02-10
Status: Proposed

## Context
Search is expected to be the most frequently called capability. It must remain deterministic, explainable, and useful without embeddings.

## Decision
Extend the search pipeline to incorporate structural signals and expose a deterministic score breakdown.

## Design Overview

### Ranking Pipeline
- Base score from BM25/TF‑IDF over chunks.
- Structural boosts from headings, filenames, and path proximity.
- Deterministic tie-breakers on stable IDs.

### Granularity
- Note-level aggregation from chunks.
- Heading and block‑level result modes.

### Output
- JSON schema includes score components.

## Alternatives Considered
- **Embeddings**: rejected to preserve determinism and offline operation.

## Consequences
- More complex scoring logic and tuning.

## Open Questions
- Default weights for structural signals.

## References
- [docs/TODOs.md](../TODOs.md)

# ADR: Phase 10 Related‑Note Engine Design

Date: 2026-02-10
Status: Proposed

## Context
Agents need related‑note recommendations that are explainable and deterministic without embeddings.

## Decision
Add a related‑note query module using classic graph similarity measures.

## Design Overview

### Similarity Methods
- Shared neighbors
- Jaccard
- Adamic‑Adar
- Preferential attachment

### Ranking
- Deterministic tie‑breakers based on stable IDs.

### Output
- JSON output includes method and contributing metrics.

## Alternatives Considered
- **Embeddings**: rejected for determinism and offline constraints.

## Consequences
- More complex compute for large graphs.

## Open Questions
- Default method and weighting.

## References
- [docs/TODOs.md](../TODOs.md)

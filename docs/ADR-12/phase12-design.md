# ADR: Phase 12 Hygiene & Refactoring Signals Design

Date: 2026-02-10
Status: Proposed

## Context
Autonomous workflows benefit from clear, deterministic hygiene signals to prioritize cleanup and refactoring.

## Decision
Add a hygiene analysis module with deterministic thresholds and rankings.

## Design Overview

### Signals
- Oversized notes: based on word count or chunk count.
- Low‑connectivity: link degree below threshold.
- Duplication risk: title similarity and content fingerprints.

### Output
- JSON output with signal type and contributing metrics.

## Alternatives Considered
- **Automatic edits**: rejected; only signals provided.

## Consequences
- Requires heuristic thresholds and tuning.

## Open Questions
- Default thresholds for size and connectivity.

## References
- [docs/TODOs.md](../TODOs.md)

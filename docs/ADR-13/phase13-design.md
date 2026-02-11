# ADR: Phase 13 Observability & Health Reports Design

Date: 2026-02-10
Status: Proposed

## Context
Agents and users need macro‑level health indicators to understand vault structure and change over time.

## Decision
Add a reporting layer that outputs deterministic health summaries and trend snapshots.

## Design Overview

### Health Metrics
- Note counts, link ratios, orphan percentage, density.

### Trends
- Snapshot metrics per run for growth tracking.

### Output
- JSON output with stable ordering and schemas.

## Alternatives Considered
- **External analytics tooling**: rejected to keep local‑first.

## Consequences
- Requires storage for trend snapshots.

## Open Questions
- Snapshot cadence and retention policy.

## References
- [docs/TODOs.md](../TODOs.md)

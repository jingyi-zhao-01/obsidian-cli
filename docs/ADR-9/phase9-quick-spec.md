# Phase 9 Quick Spec — Graph Primitives

## Goal
Provide core graph operations for structural reasoning over the vault.

## Scope
- Multi-hop neighbors
- Shortest path between notes
- Connected components
- Centrality / hub ranking
- Bridge node detection

## Out of Scope
- Related‑note scoring heuristics
- Visualization tooling

## Functional Requirements
1) **Neighborhood Queries**
- Multi-hop traversal with max depth

2) **Paths**
- Shortest path by hop count

3) **Global Metrics**
- Components, centrality, bridge detection

## Non‑Functional Requirements
- Deterministic ordering and IDs
- Reasonable performance on test vault

## CLI Expectations
- `graph neighbors "Note" --depth N`
- `graph path "A" "B"`
- `graph components`
- `graph centrality --limit N`

## Implementation Tasks
- Build graph query module
- Add traversal + metric utilities
- Add JSON schemas + tests

## Acceptance Criteria
- Graph operations return deterministic JSON
- Results validated on test vault

## Risks / Notes
- Some metrics may be expensive on large vaults.

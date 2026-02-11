# ADR: Phase 9 Graph Primitives Design

Date: 2026-02-10
Status: Proposed

## Context
Structural graph primitives enable navigation, reasoning, and automation without embeddings. They must be deterministic and stable.

## Decision
Implement a graph query layer with traversal, pathfinding, and global metric functions.

## Design Overview

### Graph Model
- Nodes represent notes; edges represent links.
- Use stable note identifiers in outputs.

### Operations
- BFS for multi-hop neighbors and shortest path.
- Component detection via union-find or BFS.
- Centrality and bridge detection computed from link graph.

### Output
- Deterministic ordering and JSON schemas.

## Alternatives Considered
- **External graph database**: rejected for complexity and portability.

## Consequences
- Additional compute cost for global metrics.

## Open Questions
- Cache strategy for expensive metrics.

## References
- [docs/TODOs.md](../TODOs.md)

# ADR: Phase 7 Machine Contracts & Determinism Design

Date: 2026-02-10
Status: Proposed

## Context
To serve as infrastructure for agents, the CLI must provide deterministic, stable, and machine-readable outputs. Current output is human-oriented and may not guarantee ordering, stable identifiers, or consistent error contracts.

## Decision
Introduce a machine-contract layer that standardizes output schemas, ordering, stable identifiers, and exit codes across all commands.

## Design Overview

### Output Contracts
- Add a JSON serializer with per-command schema definitions.
- Define explicit sorting rules and tie-breakers.
- Ensure all outputs include stable note identifiers.

### Stable Identifiers
- Persist a stable note ID in the database.
- Map path changes to the existing stable ID.

### Exit Codes
- Standardize success and error codes.
- In JSON mode, return structured errors with code + message.

### Determinism Rules
- No nondeterministic fields in JSON outputs.
- Sorting guaranteed for all lists.

## Alternatives Considered
- **Ad-hoc per-command JSON**: rejected due to inconsistent contracts.

## Consequences
- Slight overhead for serialization and sorting.
- Requires schema documentation and contract tests.

## Open Questions
- Should stable IDs be UUIDs or hash-based?

## References
- [docs/TODOs.md](../TODOs.md)

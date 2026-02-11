# Phase 7 Quick Spec — Machine Contracts & Determinism

## Goal
Make the CLI a stable, deterministic machine interface suitable for agents and automation.

## Scope
- JSON output for all commands
- Stable identifiers independent of path
- Explicit, documented sort order
- Clean exit code contract
- Deterministic output guarantees

## Out of Scope
- New graph features
- Search ranking changes
- UI/UX enhancements

## Functional Requirements
1) **JSON Output**
- Add `--json` for all commands
- Provide consistent schema per command

2) **Stable Identifiers**
- Introduce note ID stable across moves/renames
- Expose ID in all outputs

3) **Deterministic Ordering**
- Define primary + tie-break sort rules
- Document ordering for each command

4) **Exit Codes**
- Define success, user error, and system error codes
- Ensure consistent error payloads in JSON mode

## Non‑Functional Requirements
- Same input yields identical output across runs
- No new non-deterministic fields (e.g., timestamps) in JSON
- Performance overhead <10% on test vault

## CLI Expectations
- `--json` available on all commands
- `--sort` optional only when supported

## Implementation Tasks
- Add output serializer layer
- Define JSON schemas and examples
- Add stable ID to DB and outputs
- Add deterministic sorting and tests

## Acceptance Criteria
- JSON output matches schema for all commands
- Stable ordering and IDs verified in tests
- Exit code behavior documented and consistent

## Risks / Notes
- Introducing stable IDs requires careful migration plan.

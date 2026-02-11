# Phase 11 Quick Spec — Metadata & Tag Intelligence

## Goal
Enable structured filtering and planning via metadata and tag logic.

## Scope
- Boolean tag algebra (AND/OR/NOT)
- Frontmatter querying
- Missing metadata detection
- Stale vs recent classification
- Ownership/status grouping

## Out of Scope
- UI dashboards

## Functional Requirements
1) **Tag Algebra**
- Support AND/OR/NOT semantics

2) **Frontmatter Queries**
- Filter by key, presence, and simple values

3) **Quality Signals**
- Identify missing or stale metadata

## Non‑Functional Requirements
- Deterministic results and ordering

## CLI Expectations
- `tags "tag1" --all --json`
- `meta where key=value`
- `meta missing key`

## Implementation Tasks
- Add metadata query helpers
- Extend tag query grammar
- Add tests for filtering correctness

## Acceptance Criteria
- Boolean tag queries behave correctly
- Metadata filters deterministic

## Risks / Notes
- Frontmatter schema diversity may require flexible parsing.

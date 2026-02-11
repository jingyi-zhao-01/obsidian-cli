# Phase 13 Quick Spec — Observability & Health Reports

## Goal
Provide macro‑level observability for agents and users.

## Scope
- Note counts
- Link ratios
- Orphan percentage
- Density metrics
- Growth trends

## Out of Scope
- Visualization UI

## Functional Requirements
1) **Summary Report**
- Produce a deterministic JSON report

2) **Trends**
- Track and report growth metrics over time

## Non‑Functional Requirements
- Deterministic report ordering
- Minimal performance impact

## CLI Expectations
- `report health --json`
- `report trends --json`

## Implementation Tasks
- Add reporting queries
- Add persistence for trend snapshots
- Add tests for report stability

## Acceptance Criteria
- Health report deterministic across runs
- Trend snapshots are consistent and documented

## Risks / Notes
- Trend storage requires schema changes.

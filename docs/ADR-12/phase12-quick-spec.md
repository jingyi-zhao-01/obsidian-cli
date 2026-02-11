# Phase 12 Quick Spec — Hygiene & Refactoring Signals

## Goal
Provide actionable hygiene signals for autonomous workflows.

## Scope
- Oversized notes detection
- Low‑connectivity notes
- Duplication risk
- Merge candidates
- Split candidates

## Out of Scope
- Automated refactoring actions

## Functional Requirements
1) **Size Signals**
- Identify oversized notes by thresholds

2) **Connectivity Signals**
- Flag low‑connectivity or orphan notes

3) **Duplication Signals**
- Detect high similarity in titles or content fingerprints

## Non‑Functional Requirements
- Deterministic ranking and thresholds

## CLI Expectations
- `hygiene oversized --limit N`
- `hygiene low-connectivity`
- `hygiene duplicates`

## Implementation Tasks
- Add hygiene query module
- Define thresholds and defaults
- Add tests for signal stability

## Acceptance Criteria
- Hygiene results deterministic across runs
- Thresholds documented

## Risks / Notes
- Risk of false positives in duplication detection.

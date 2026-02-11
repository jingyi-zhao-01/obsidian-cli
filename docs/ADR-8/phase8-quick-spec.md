# Phase 8 Quick Spec — Deterministic Search v2

## Goal
Provide composable, explainable, and deterministic search without embeddings.

## Scope
- BM25/TF‑IDF ranking
- Heading-level and block-level search
- Filename similarity and path proximity signals
- Explainable scoring breakdown

## Out of Scope
- Embedding-based retrieval
- UI rendering of highlights

## Functional Requirements
1) **Ranking Signals**
- Combine BM25/TF‑IDF with structural signals
- Produce deterministic score ties

2) **Granularity**
- Search at note, heading, and block levels

3) **Explainability**
- Include score components in JSON output

## Non‑Functional Requirements
- Deterministic ranking for identical input
- Search latency under 200ms on test vault

## CLI Expectations
- `search "query" --limit N --json`
- Optional `--level note|heading|block`

## Implementation Tasks
- Extend query layer for granularity
- Add scoring breakdown to results
- Add tests for deterministic ranking

## Acceptance Criteria
- Results are stable across runs
- Score breakdown is emitted in JSON mode

## Risks / Notes
- Combining signals may require tuning weights.

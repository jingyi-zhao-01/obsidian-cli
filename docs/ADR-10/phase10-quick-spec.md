# Phase 10 Quick Spec — Related‑Note Engine (Graph‑Native)

## Goal
Provide explainable related‑note retrieval using classic graph measures.

## Scope
- Shared neighbors
- Jaccard similarity
- Adamic‑Adar
- Preferential attachment
- Explanation output

## Out of Scope
- Embedding‑based similarity

## Functional Requirements
1) **Scoring Methods**
- Support multiple graph‑native scores

2) **Explainability**
- Provide contributing factors per result

## Non‑Functional Requirements
- Deterministic ranking and tie‑breakers

## CLI Expectations
- `related "Note" --method jaccard --limit N`
- `related "Note" --json`

## Implementation Tasks
- Implement similarity functions
- Add result explanations
- Add tests for determinism

## Acceptance Criteria
- Related‑note results reproducible across runs
- Explanation data present in JSON mode

## Risks / Notes
- High-degree nodes may dominate results without normalization.

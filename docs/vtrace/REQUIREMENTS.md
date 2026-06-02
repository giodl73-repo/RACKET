# RACKET Requirements

## Scope

Repo: RACKET

VTRACE stage: Requirements

Baseline date: 2026-06-01

## Requirement Matrix

| Requirement ID | Requirement | Source | Rationale | Priority | Owner Surface | Verification Method | Status |
|---|---|---|---|---|---|---|---|
| REQ-RCK-001 | RACKET shall translate COURT snapshots into native frame plans while preserving scene and action intent. | NEED-RCK-001 / CON-RCK-001 | Adapter value depends on faithful contract consumption. | must | `racket-core` | test | proposed |
| REQ-RCK-002 | RACKET shall emit explicit diagnostics for unsupported or dropped COURT features. | NEED-RCK-002 / CON-RCK-001 | Silent loss breaks product fixture trust. | must | diagnostics | test, inspection | proposed |
| REQ-RCK-003 | RACKET shall provide deterministic windowless runtime-loop proof before renderer/backend expansion. | NEED-RCK-003 / CON-RCK-002 | Runtime behavior should be stable before visual backend decisions. | must | runtime loop | test | proposed |
| REQ-RCK-004 | RACKET shall keep product rules, puzzle logic, tabletop rules, and fantasy outside `racket-core`. | NEED-RCK-004 / CON-RCK-003 | Product ownership remains in source repos. | must | package boundaries | review | proposed |
| REQ-RCK-005 | RACKET shall treat product fixture compatibility as adapter proof, not product readiness. | CON-RCK-003 | Engine proof and product validation are separate. | must | fixture diagnostics | inspection | proposed |
| REQ-RCK-006 | RACKET shall defer renderer/backend/input selection until a product fixture requires it and adapter evidence is stable. | CON-RCK-004 | Avoid premature backend lock-in. | should | future work packages | review | proposed |
| REQ-RCK-007 | RACKET shall keep repo-local tests as the baseline validation surface. | MSC-RCK-002 | Current foundation claim is test-backed. | must | Cargo tests | command | proposed |

## Deferred Definitions

| Deferred ID | Item | Disposition |
|---|---|---|
| DEF-RCK-001 | Exact COURT snapshot fields and diagnostics taxonomy. | Defer to specification/interface stages. |
| DEF-RCK-002 | Renderer/backend readiness gate. | Defer to validation and work packages. |
| DEF-RCK-003 | Product fixture evidence labels. | Defer to trace/review. |

## Role Review Summary

RACKET does not currently expose `.roles/`. Requirements review used documented
product lenses: COURT contract steward, engine adapter maintainer, product
boundary steward, runtime diagnostics auditor, and future renderer reviewer.

Findings:

| Lens | Finding | Disposition |
|---|---|---|
| COURT Contract Steward | Snapshot intent preservation must be first-class. | Addressed by REQ-RCK-001. |
| Runtime Diagnostics Auditor | Silent feature loss needs an explicit requirement. | Addressed by REQ-RCK-002. |
| Product Boundary Steward | Product rules must be forbidden in `racket-core`. | Addressed by REQ-RCK-004. |
| Future Renderer Reviewer | Backend selection must be gated, not assumed. | Addressed by REQ-RCK-006. |

Fixed-point decision:

No critical or major actionable findings remain for the requirements stage.
Exact specs, interface schemas, verification command levels, and work packages
are deferred to later VTRACE stages.

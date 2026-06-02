# RACKET Specification Baseline

## Scope

Repo: RACKET

VTRACE stage: Specification Baseline

Baseline date: 2026-06-01

## Baseline Inventory

| Surface | Paths | Baseline Status | Notes |
|---|---|---|---|
| Mission/CONOPS/requirements | `docs/vtrace/` | current | VTRACE planning chain established through requirements. |
| Core adapter | `crates/racket-core` | current | Translates COURT contracts into native engine frame-plan behavior. |
| Foundation wave | `context/waves/2026-05-18-racket-foundation/` | current | Records frame plans, diagnostics, runtime loop, and product fixture smokes. |
| Product fixtures | AMAZE Prism Vault, TIGRIS Parliament references | current | Product repos own rules; RACKET consumes snapshots. |

## Specification Items

| Spec ID | Parent REQ IDs | Type | Baseline | Specification Statement | Verification | Validation | Owner Surface | Risk |
|---|---|---|---|---|---|---|---|---|
| SPEC-RCK-001 | REQ-RCK-001 | software/interface | current | COURT snapshots should translate into native frame plans that preserve scene/action intent. | test | adapter review | `racket-core` | high |
| SPEC-RCK-002 | REQ-RCK-002 | software | current | Unsupported or dropped COURT features should produce explicit diagnostics. | test, inspection | product fixture review | diagnostics | high |
| SPEC-RCK-003 | REQ-RCK-003, REQ-RCK-007 | software | current | Windowless runtime tests are the baseline proof for deterministic frame-plan stepping. | command | n/a | Cargo tests | medium |
| SPEC-RCK-004 | REQ-RCK-004, REQ-RCK-005 | boundary | current | RACKET consumes product fixtures without owning gameplay rules or product readiness. | review | product fixture review | package boundaries | high |
| SPEC-RCK-005 | REQ-RCK-006 | roadmap | target | Renderer/backend/input expansion is gated by product fixture need and stable adapter evidence. | review | readiness review | future work packages | medium |

## Unknowns And Deferred Detail

| Unknown ID | Unknown | Risk | Disposition |
|---|---|---|---|
| SPEC-RCK-UNK-001 | Full COURT field taxonomy is not repeated here. | Interface trace needs exact rows. | Defer to interfaces. |
| SPEC-RCK-UNK-002 | Renderer/backend readiness checklist is not frozen. | Future backend work could start early. | Defer to validation/work packages. |
| SPEC-RCK-UNK-003 | Product fixture evidence labels are not locked. | Product readiness might be conflated with adapter proof. | Defer to validation/trace. |

## Role Review Summary

RACKET does not currently expose `.roles/`. Specification review used documented
product lenses: COURT contract steward, engine adapter maintainer, product
boundary steward, runtime diagnostics auditor, and future renderer reviewer.

Findings:

| Lens | Finding | Disposition |
|---|---|---|
| COURT Contract Steward | Scene/action intent preservation needs a high-risk spec. | Addressed by SPEC-RCK-001. |
| Runtime Diagnostics Auditor | Diagnostics are a controlled behavior, not optional logs. | Addressed by SPEC-RCK-002. |
| Product Boundary Steward | Adapter proof must not imply product readiness. | Addressed by SPEC-RCK-004. |
| Future Renderer Reviewer | Renderer/backend work remains target-scoped. | Addressed by SPEC-RCK-005. |

Fixed-point decision:

No critical or major actionable findings remain for the specification baseline.
Gate is `pass_with_risk` because exact COURT field taxonomy, renderer readiness
checklist, and evidence labels are deferred.

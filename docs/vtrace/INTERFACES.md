# RACKET Interfaces

## Scope

Repo: RACKET

VTRACE stage: Interfaces

Baseline date: 2026-06-01

## Interface Matrix

| Interface ID | Parent Architecture | Interface | Producer | Consumer | Contract | Evidence |
|---|---|---|---|---|---|---|
| IF-RCK-001 | ARCH-RCK-001 | COURT snapshot input | COURT or product fixture | `racket-core` | Snapshot carries portable scene/action intent for adapter translation. | Unit tests or fixture snapshot inspection. |
| IF-RCK-002 | ARCH-RCK-001 | Native frame plan output | `racket-core` | Runtime loop, diagnostics reviewer | Frame plan preserves scene/action intent without adding product rules. | Unit tests over frame-plan translation. |
| IF-RCK-003 | ARCH-RCK-002 | Unsupported feature diagnostics | Diagnostics layer | Product maintainer, adapter reviewer | Unsupported or dropped fields are named explicitly; silent loss is invalid. | Unit tests/inspection. |
| IF-RCK-004 | ARCH-RCK-003 | Windowless runtime loop | Runtime layer | Engine maintainer | Runtime steps deterministic frame plans without renderer/backend selection. | `cargo test --quiet`. |
| IF-RCK-005 | ARCH-RCK-004 | Product fixture adapter proof | AMAZE/TIGRIS fixtures | RACKET adapter | Fixture proves adapter compatibility only; source product keeps gameplay rules and readiness claims. | Product fixture diagnostics/smokes. |

## Boundary Rules

- RACKET consumes COURT/product contracts and must not own product rules.
- Unsupported fields require diagnostics before renderer or backend expansion.
- Windowless runtime proof is not final UX readiness.
- Renderer, backend, and input policy remain gated by product fixture need.

## Role Review Summary

RACKET does not currently expose `.roles/`. Review used documented product
lenses: COURT contract steward, engine adapter maintainer, product boundary
steward, runtime diagnostics auditor, and future renderer reviewer.

No critical or major actionable findings remain. Exact COURT field taxonomy,
renderer readiness checklist, and product fixture labels are deferred to later
stages.

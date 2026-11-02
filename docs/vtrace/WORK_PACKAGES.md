# RACKET Work Packages

## Scope

Repo: RACKET

VTRACE stage: Work Packages

Baseline date: 2026-06-01

## Package Backlog

| WP ID | Source | Title | Outcome | Verification / Validation | Status |
|---|---|---|---|---|---|
| WP-RCK-001 | GAP-RCK-001, REV-RCK-001 | Lock COURT adapter contract taxonomy | `cargo test --quiet` passed; `crates/racket-core/src/lib.rs` defines frame-plan fields and diagnostics for provenance boundary, illegal/guided actions, diagnostic-only actions, unsupported scene roles, and unsupported scene features. | VER-RCK-003, VER-RCK-004, VAL-RCK-001, VAL-RCK-002 | complete |
| WP-RCK-002 | GAP-RCK-003, REV-RCK-002 | Prove product fixture diagnostics | AMAZE and TIGRIS product fixture packages passed from their owning repos; RACKET keeps those as adapter proof only and does not claim product readiness. | VER-RCK-006, VAL-RCK-004 | complete |
| WP-RCK-003 | GAP-RCK-002, REV-RCK-003 | Maintain renderer/backend readiness gate | `README.md` and `PRODUCT_PLAN.md` keep renderer/backend/input selection deferred until a product fixture creates an explicit need. No current product need appeared. | VER-RCK-002, VAL-RCK-005 | gated |

| WP-RCK-004 | TIGRIS Parliament diagnostic handoff; entry review | Bounded WASM client for existing windowless frame-plan/runtime | COURT-owned decoding; unchanged native semantics and product authority | `context/waves/2026-11-02-wasm-frame-workbench/README.md`; delivery evidence pending | in progress |

## Execution Rules

- Adapter proof is not product readiness.
- Unsupported fields require diagnostics.
- Do not add renderer/backend/input commitments without product fixture need.

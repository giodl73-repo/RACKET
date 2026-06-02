# RACKET Mission

## Scope

Repo: RACKET

VTRACE stage: Mission

Baseline date: 2026-06-01

RACKET is the first Rust-native engine adapter for COURT scalable experiences.
Its mission is to consume portable COURT snapshots and produce deterministic
native frame plans, diagnostics, and runtime-loop proof without owning product
rules, renderer choices, art direction, or final input/backend policy.

## Mission Need

| Need ID | Need | Primary User | Success Signal |
|---|---|---|---|
| NEED-RCK-001 | Prove COURT snapshots can become native engine frame plans. | COURT maintainer, engine adapter developer | `racket-core` translates snapshots and preserves scene/action intent. |
| NEED-RCK-002 | Report unsupported COURT features explicitly. | Product repo maintainer, reviewer | Diagnostics name dropped or unsupported fields instead of silently losing intent. |
| NEED-RCK-003 | Step frame plans in a deterministic windowless runtime loop. | Engine/runtime maintainer | Smoke tests validate frame-plan progression without renderer selection. |
| NEED-RCK-004 | Keep product rules inside product repos. | AMAZE/TIGRIS/Court consumer | Product fixtures exercise RACKET without moving gameplay logic into `racket-core`. |

## Mission Success Criteria

| Criterion ID | Criterion | Evidence Surface | Deferred Detail |
|---|---|---|---|
| MSC-RCK-001 | A future agent can identify the current adapter contract and deferred renderer/input choices. | `README.md`, `PRODUCT_PLAN.md`, `context/waves/` | Trace rows deferred to `TRACE.md`. |
| MSC-RCK-002 | Frame-plan and diagnostics behavior is covered by repo-local tests. | `cargo test --quiet` | Command levels deferred to `VERIFICATION.md`. |
| MSC-RCK-003 | Product fixtures can use RACKET while retaining product ownership. | AMAZE Prism Vault and TIGRIS Parliament references | Interface ownership deferred to `INTERFACES.md`. |
| MSC-RCK-004 | Renderer/backend expansion is gated by product fixture need. | Product plan non-goals and deferred wave rows | Work packages deferred until baseline is traced. |

## Constraints

- RACKET must not own product rules, fantasy, puzzle logic, or tabletop rules.
- Unsupported COURT fields must be diagnostic, not silent loss.
- Renderer/backend selection remains deferred until a product fixture requires
  it.
- Windowless runtime proof is valid only for deterministic engine behavior, not
  final UX readiness.

## Initial Validation Expectations

```powershell
cargo test --quiet
```

## Role Review Summary

RACKET does not currently expose `.roles/`. Mission review used documented
product lenses: COURT contract steward, engine adapter maintainer, product
boundary steward, runtime diagnostics auditor, and future renderer reviewer.

Findings:

| Lens | Finding | Disposition |
|---|---|---|
| COURT Contract Steward | Mission must preserve portable scene/action intent from COURT snapshots. | Addressed in NEED-RCK-001. |
| Runtime Diagnostics Auditor | Unsupported features must be reported explicitly. | Addressed in NEED-RCK-002. |
| Product Boundary Steward | Product rules must stay outside `racket-core`. | Addressed in NEED-RCK-004 and constraints. |
| Future Renderer Reviewer | Renderer/backend decisions must stay deferred until evidence requires them. | Addressed in constraints. |

Fixed-point decision:

No critical or major actionable findings remain for the mission stage. Exact
requirements, command levels, interface rows, and work packages are deferred to
later VTRACE stages.

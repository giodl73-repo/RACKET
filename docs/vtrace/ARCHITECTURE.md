# RACKET Architecture

## Scope

Repo: RACKET

VTRACE stage: Architecture

Baseline date: 2026-06-01

## Architecture Elements

| Architecture ID | Parent Specs | Element | Responsibility | Boundary | Verification Target |
|---|---|---|---|---|---|
| ARCH-RCK-001 | SPEC-RCK-001 | COURT snapshot adapter | Converts portable snapshots into native frame plans. | Consumes contracts; does not own product rules. | Unit tests over frame-plan translation. |
| ARCH-RCK-002 | SPEC-RCK-002 | Diagnostics layer | Reports unsupported/dropped COURT features explicitly. | Diagnostics prevent silent loss. | Unit tests/inspection. |
| ARCH-RCK-003 | SPEC-RCK-003 | Windowless runtime loop | Steps deterministic frame plans without renderer selection. | Runtime proof is not final UX readiness. | `cargo test --quiet`. |
| ARCH-RCK-004 | SPEC-RCK-004, SPEC-RCK-005 | Product fixture boundary | Accepts AMAZE/TIGRIS fixtures as adapter proof. | Product repos own gameplay and readiness. | Product fixture tests/diagnostics. |

## Data And Control Flow

```text
COURT snapshot -> RACKET frame plan -> diagnostics
  -> windowless runtime smoke
  -> product fixture proof
  -> renderer/backend gate only when product need exists
```

## Architecture Risks

| Risk ID | Risk | Mitigation |
|---|---|---|
| RISK-RCK-001 | Unsupported COURT fields are silently lost. | Diagnostics layer is required. |
| RISK-RCK-002 | RACKET absorbs product rules. | Product fixture boundary is explicit. |
| RISK-RCK-003 | Renderer/backend choice starts before adapter proof. | Windowless runtime and product-need gates. |

## Role Review Summary

No critical or major actionable findings remain. Exact package IDs, interface
schemas, and verification commands are deferred to later stages.

# RACKET CONOPS

## Scope

Repo: RACKET

VTRACE stage: CONOPS

Baseline date: 2026-06-01

This CONOPS describes how RACKET users consume COURT snapshots, produce native
engine frame plans, report unsupported adapter features, and step deterministic
runtime loops without owning product rules or prematurely choosing renderer and
input backends.

## Operational Scenarios

| Scenario ID | Actor | Trigger | Nominal Flow | Degraded / Failure Flow | Evidence Output |
|---|---|---|---|---|---|
| CON-RCK-001 | Engine adapter developer | COURT snapshot is supplied by a product repo. | Parse snapshot, translate scene/action state into frame plan, preserve product intent. | If required snapshot fields are unsupported, emit diagnostics and avoid silent frame-plan loss. | Frame plan, adapter diagnostics. |
| CON-RCK-002 | Runtime maintainer | A deterministic engine smoke is requested. | Step frame plans in a windowless runtime loop and assert stable progression. | If runtime loop cannot preserve deterministic state, block renderer/backend expansion. | Test output, runtime-loop smoke. |
| CON-RCK-003 | Product fixture maintainer | AMAZE or TIGRIS fixture exercises RACKET. | Consume product-owned COURT snapshot and validate adapter compatibility while product rules stay in source repo. | If RACKET needs gameplay semantics, reject boundary and add interface requirements. | Product fixture diagnostic and runtime proof. |
| CON-RCK-004 | Future renderer reviewer | A renderer/backend expansion is proposed. | Check product fixture need, adapter contract stability, diagnostics coverage, and non-goals before selecting backend. | If product need or contract evidence is missing, defer renderer/backend selection. | Review decision, future work package. |

## Operating Modes

| Mode | Purpose | Entry Condition | Exit Condition |
|---|---|---|---|
| Adapter translation | Convert COURT snapshot into frame plan. | Snapshot contract is available. | Frame plan plus diagnostics are produced. |
| Windowless runtime | Validate deterministic engine stepping. | Frame plan exists. | Runtime smoke passes or expansion is blocked. |
| Product fixture compatibility | Prove adapter across product-owned examples. | Product fixture exists. | Product remains owner and RACKET reports adapter proof. |
| Renderer/backend readiness | Decide whether to add a renderer or input backend. | Product fixture needs visual/input proof. | Work package is approved or deferred. |

## Role Review Summary

RACKET does not currently expose `.roles/`. CONOPS review used documented
product lenses: COURT contract steward, engine adapter maintainer, product
boundary steward, runtime diagnostics auditor, and future renderer reviewer.

Findings:

| Lens | Finding | Disposition |
|---|---|---|
| Runtime Diagnostics Auditor | Unsupported fields need a degraded path before renderer work. | Addressed in CON-RCK-001. |
| Engine Adapter Maintainer | Windowless runtime proof must gate backend expansion. | Addressed in CON-RCK-002. |
| Product Boundary Steward | Product fixtures must not transfer gameplay rules. | Addressed in CON-RCK-003. |
| Future Renderer Reviewer | Renderer/backend selection needs product evidence. | Addressed in CON-RCK-004. |

Fixed-point decision:

No critical or major actionable findings remain for the CONOPS stage. Exact
requirement IDs, validation command levels, interface rows, and work packages
are deferred to later VTRACE stages.

# RACKET Communications Strategy

## Purpose

This artifact maps accepted RACKET VTRACE intent to user-facing docs surfaces.
The docs package explains the COURT adapter contract, deterministic frame-plan
proof, diagnostic behavior, product fixture evidence, and renderer/backend gate
without implying product-rule ownership.

## Surface Plan

| Surface ID | Source IDs | Audience | User Question | Generated Docs | Cadence | Owner | Status |
|---|---|---|---|---|---|---|---|
| COMMS-RCK-README-001 | NEED-RCK-001 / MSC-RCK-001 / WP-RCK-001 | COURT maintainer / engine adapter developer | Where do I start, and what adapter contract is current? | `docs/README.md` docs map plus README routing | every docs wave | RACKET maintainer | planned |
| COMMS-RCK-CONCEPTS-001 | NEED-RCK-001 / REQ-RCK-001 / SPEC-RCK-001 / WP-RCK-001 | engine adapter developer | What is a native frame plan and what does it preserve? | `docs/concepts/frame-plans.md` | when adapter contract changes | RACKET adapter owner | planned |
| COMMS-RCK-DIAGNOSTICS-001 | NEED-RCK-002 / REQ-RCK-002 / WP-RCK-001 | product repo maintainer / reviewer | How are unsupported COURT fields reported instead of dropped? | `docs/reference/diagnostics.md` | when diagnostic taxonomy changes | RACKET diagnostics owner | planned |
| COMMS-RCK-HOWTO-001 | NEED-RCK-003 / VER-RCK-001 / WP-RCK-001 | verification runner / future agent | How do I run deterministic windowless adapter proof? | `docs/how-to/run-windowless-proof.md` | when runtime proof commands change | RACKET runtime owner | planned |
| COMMS-RCK-FIXTURE-001 | NEED-RCK-004 / IF-RCK-003 / WP-RCK-002 | AMAZE / TIGRIS / COURT consumer | What product fixture proof exists without product-rule transfer? | `docs/examples/product-fixture-diagnostics.md` | when product fixture evidence changes | RACKET fixture owner | planned |
| COMMS-RCK-GATE-001 | MSC-RCK-004 / WP-RCK-003 / VAL-RCK-005 | future renderer/backend owner | What must happen before renderer, backend, or input commitments expand? | `docs/concepts/renderer-backend-gate.md` | when product fixture need appears | RACKET product-boundary owner | gated |
| COMMS-RCK-CORPUS-001 | REV-RCK-003 / WP-RCK-001 / WP-RCK-002 / WP-RCK-003 | docs owner / future agent | Who owns adapter docs and gated follow-up updates? | `docs/CORPUS.md` | every docs wave | RACKET docs owner | planned |

## Review Checklist

| Item | Required | Decision | Evidence / Rationale |
|---|---|---|---|
| Docs claims trace to controlled source IDs. | yes | accepted | Rows cite mission, requirements, specs, interfaces, verification, validation, review, and packages. |
| Concepts/tutorials/examples do not overclaim unvalidated behavior. | yes | accepted | Renderer/backend expansion remains gated and product readiness is not claimed. |
| Public interfaces have expected usage or expected output docs. | if applicable | accepted | Adapter, diagnostic, runtime, and product fixture surfaces are mapped. |
| `docs/CORPUS.md` names ownership and update obligations. | if multiple surfaces exist | planned | COMMS-RCK-CORPUS-001 records the corpus surface. |

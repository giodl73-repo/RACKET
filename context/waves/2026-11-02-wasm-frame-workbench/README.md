# Windowless frame workbench â€” entry gate

Status: scoped and reviewed before browser implementation; delivery pending.

WP-RCK-004 serves the maintainer of TIGRIS Parliament's existing
`parliament_ai_court_snapshot()` fixture: inspect an owned COURT snapshot and
replay the existing deterministic windowless frame plan with an explicit count.
The named integration need is diagnosis/reproduction without a native toolchain.
TIGRIS retains rules, scene direction, assessment and release authority.

## Entry evidence

RACKET e2166f6 passes locked workspace tests, strict Clippy and formatting.
TIGRIS 2b2e389 passes both existing `court_` library tests in an isolated
rehearsal using explicit local dependency patches. This is local adapter evidence,
not a clean remote product release. Retain exact lock, patch configuration and log.
Old locked COURT bbdcf567 and current 5fff1e87 have identical production
`court-core` source before `#[cfg(test)]`; the latter retains two policy tests.
No core contract or RUNE descriptor change is proposed.

## Pre-implementation role review

Agent review using repository role lenses, not human signoff:

| Role | Finding and disposition |
|---|---|
| Compatibility Gatekeeper | Reuse COURT inspector's strict decoder through an additive COURT-owned helper. Block a RACKET DTO fork. Retain accepted/not-ready fixtures and exact diagnostics. |
| Runtime Determinism Auditor | Use unchanged native functions. Bound input and expanded output before cloning frames; retain zero-frame and diagnostic-bearing-ready cases. Require independent native/WASM replay. |
| Adapter Boundary Steward | No product rules, contract extensions, compatibility shim or asset handling. COURT owns decoding; RACKET presents its existing native outputs. |
| Product Integrator | TIGRIS Parliament provides a named diagnostic need. Default content must be explicitly synthetic. Native readiness never implies rendering, product acceptance, assessment or asset readiness. |

This gate permits a diagnostic browser client only. WP-RCK-003's renderer,
backend, game input and asset-pipeline gate remains open. Delivery requires
source/legal downloads, bounded worker execution, native parity, independent
delivered-source replay, hosted qualification and fresh live verification.

## Implementation checkpoint

COURT PR 2 merged as 1b9ecb5193e844cb39d0dc76c8ef099ce57edda7. Both committed
locks share that Git source identity; temporary local overrides have been removed.
The root lock changes only COURT's revision. Native core/manifests/RUNE fixtures
remain unchanged from e2166f6. Six original native tests and four adapter tests
pass without local overrides; strict native/host/WASM Clippy passes.

The browser runs the same native plan and runtime. COURT's admitted snapshot,
queries and adapter observations are retained separately from RACKET diagnostics.
Native count readiness can coexist with diagnostics; every requested frame still
repeats the same snapshot. No renderer, game state evolution or action execution
is added. Worker memory is capped at 64 MiB; expanded plan bytes are bounded
before native frame cloning. There are 48 native/WASM request cases, including
controls against the pre-existing accepted/rejected runtime-proof.json totals.

Initial 18 browser checks pass, including actual memory-growth rejection and
frame/diagnostic semantics. A subsequent 19-check run found one restart readiness
wait exceeded its five-second test assertion; the isolated unchanged recovery
check then passed. Preserve that failed attempt and require a clean full packaged
run, hosted qualification and source replay before claiming delivery. The native
engine's actual twelve-second startup deadline is unchanged.

Complete source/legal packaging, hosted qualification, main deployment and fresh
live verification are pending. WP-RCK-003 remains gated. This checkpoint is not
a product adoption, release, assessment, renderer or asset-readiness decision.

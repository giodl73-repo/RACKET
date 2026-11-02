# Windowless frame workbench — entry gate

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

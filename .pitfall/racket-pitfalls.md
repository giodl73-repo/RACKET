# RACKET Pitfalls

## RACKET-PF-01: Adapter Proof Becomes Product Readiness

**Status:** MITIGATED

**Pattern:** A passing RACKET frame-plan, runtime, proof-surface, AMAZE, or
TIGRIS fixture is described as product readiness rather than adapter
compatibility.

**Domain:** README status, VTRACE validation, product repo claims, demos,
release notes, and portfolio dependency updates.

**Actor:** Product integrator, demo author, release-note writer, portfolio
maintainer, or downstream game repo.

**Task:** Use a passing frame-plan, runtime, proof-surface, AMAZE, or TIGRIS
fixture while deciding whether a product is ready to ship or promote.

**Surface:** README status, retained runtime proof, VTRACE validation,
product-fixture notes, dependency updates, and release claims.

**Likely mistake:** Treat adapter compatibility or a deterministic frame report
as product readiness, game-design approval, renderer readiness, or customer
release evidence.

**Consequence:** Product repos can inherit readiness claims before they own
their rules, assets, scene direction, runtime policy, and user-facing tests.

**Owner:** RACKET Product Integrator, Adapter Boundary Steward, and affected
product repo owner.

**Detection difficulty:** Product fixtures are compelling because they exercise
real repos, so adapter proof can sound like game readiness.

**Structural solution:** Keep product readiness claims in product repos and
require Product Integrator and Adapter Boundary Steward review before
promotion, with machine-readable blocked claims for adapter-proof overreach.

**Evidence:** `docs/vtrace/pitfall-boundaries.v1.json`,
`docs/vtrace/VALIDATION.md`, `docs/vtrace/WORK_PACKAGES.md`,
`.roles/ROLE.md`, `.roles/stakeholders/product-integrator.md`, and
`README.md`.

**Test:** `cargo test --quiet --test pitfall_policy`.

## RACKET-PF-02: Renderer Gate Opens Without Fixture Need

**Status:** MITIGATED

**Pattern:** Macroquad, browser, input, backend, or asset-pipeline work begins
because a renderer is attractive, not because a named product fixture needs it.

**Domain:** Product plan, roadmap, engine backend selection, demos, and future
runtime waves.

**Actor:** Engine maintainer, renderer implementer, product integrator, demo
author, or future backend adopter.

**Task:** Decide whether to start Macroquad, browser, input, asset-pipeline, or
backend work after the windowless frame-plan proof passes.

**Surface:** PRODUCT_PLAN, VTRACE review/work-package docs, product fixtures,
runtime waves, demos, and backend-selection notes.

**Likely mistake:** Start renderer/backend/input work because it feels like the
natural next step, instead of because a named product fixture and role review
require it.

**Consequence:** RACKET can absorb product UX and engine-specific obligations
before COURT contracts and product-owned fixture needs justify the expansion.

**Owner:** Runtime Determinism Auditor, Product Integrator, and future renderer
owner.

**Detection difficulty:** Native rendering feels like the natural next step
after frame plans, even though VTRACE keeps the gate deferred.

**Structural solution:** Keep renderer/backend/input work gated by explicit
named fixture need, role review, and VTRACE work-package creation.

**Evidence:** `docs/vtrace/pitfall-boundaries.v1.json`, `PRODUCT_PLAN.md`,
`docs/vtrace/REVIEW.md`, `docs/vtrace/WORK_PACKAGES.md`, and `.roles/ROLE.md`.

**Test:** `cargo test --quiet --test pitfall_policy`.

## RACKET-PF-03: COURT Compatibility Proof Becomes Contract Fork

**Status:** MITIGATED

**Pattern:** RACKET adds compatibility shims, local schemas, or descriptor
fields that quietly fork COURT contracts instead of consuming reviewed COURT
snapshot fields.

**Domain:** COURT adapter changes, RUNE descriptors, public API, fixture
migrations, and compatibility policy.

**Actor:** Adapter maintainer, compatibility reviewer, RUNE descriptor editor,
COURT contract author, or future migration agent.

**Task:** Add compatibility shims, diagnostics, descriptor fields, or fixture
migrations while deciding whether COURT or RACKET owns the contract change.

**Surface:** COURT snapshot mapping, RUNE descriptors, public Rust API,
fixture migrations, VTRACE interfaces, and compatibility policy.

**Likely mistake:** Preserve a local fixture by adding RACKET-owned schema or
shim behavior that quietly forks COURT instead of consuming reviewed COURT
snapshot fields.

**Consequence:** RACKET can become a parallel contract source, making COURT
compatibility look green while downstream adapters no longer share one
contract.

**Owner:** Compatibility Gatekeeper, Adapter Boundary Steward, and COURT
contract owner.

**Detection difficulty:** A local shim can preserve a fixture while hiding that
COURT is no longer the source of contract truth.

**Structural solution:** Require Compatibility Gatekeeper and Adapter Boundary
Steward review before COURT mapping or diagnostic changes, and block any local
schema, shim, descriptor, or public API change that makes RACKET a COURT
contract source.

**Evidence:** `docs/vtrace/pitfall-boundaries.v1.json`,
`.roles/parliament/compatibility-gatekeeper.md`,
`.roles/parliament/adapter-boundary-steward.md`, `.roles/ROLE.md`,
`README.md`, and `docs/vtrace/INTERFACES.md`.

**Test:** `cargo test --quiet --test pitfall_policy`.

## RACKET-PF-04: Unsupported Fields Disappear Silently

**Status:** MITIGATED

**Pattern:** Unsupported scene roles, scene features, provenance boundaries, or
actions are dropped while the adapter still returns a polished frame/runtime
report.

**Domain:** Adapter diagnostics, proof fixtures, COURT snapshots, product
fixtures, and public readiness claims.

**Detection difficulty:** A frame plan can render enough useful content that
missing unsupported intent is overlooked.

**Structural solution:** Preserve structured diagnostics and not-ready reports
for unsupported fields.

**Evidence:** `crates/racket-core/tests/proof_surface.rs`,
`crates/racket-core/tests/fixtures/runtime-proof.json`, and
`docs/vtrace/VALIDATION.md`.

## RACKET-PF-05: Role-Review Evidence Goes Stale

**Status:** MITIGATED

**Pattern:** VTRACE summaries keep saying RACKET has no `.roles/` even after the
local role panel exists, so agents underuse current review gates.

**Domain:** VTRACE stages, wave closeouts, future adapter work, and PITFALL
integration.

**Detection difficulty:** The old summaries still say no critical findings
remain, so stale role-presence wording looks harmless.

**Structural solution:** Keep VTRACE role summaries aligned with `.roles/ROLE.md`
when adding or renaming local review panels.

**Evidence:** `.roles/ROLE.md`, `docs/vtrace/MISSION.md`,
`docs/vtrace/VALIDATION.md`, `docs/vtrace/TRACE.md`, and `git diff --check`.

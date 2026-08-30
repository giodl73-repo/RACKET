# RACKET Principles

## RACKET-P-01: COURT Owns Experience Contracts

**Decision rule:** RACKET consumes COURT snapshots and adapter contracts, but it
must not fork COURT state/action/scene definitions or become the portable
experience source of truth.

**Rationale:** RACKET proves a native engine adapter; duplicating COURT
contracts would create divergent product behavior.

**Test:** Compatibility Gatekeeper and Adapter Boundary Steward review plus
adapter tests protect COURT-to-RACKET mapping.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `.roles/ROLE.md`,
`docs/vtrace/INTERFACES.md`, and `cargo test --workspace`.

## RACKET-P-02: Engine Adapter Is Not Product Rules

**Decision rule:** `racket-core` may translate snapshots into frame plans,
diagnostics, and runtime reports, but product repos own rules, fantasy, content,
scene direction, renderer choices, and readiness claims.

**Rationale:** A shared engine adapter loses value if it becomes one product's
gameplay kernel.

**Test:** Frame-plan tests and product-boundary review verify that adapter proof
does not import gameplay semantics.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`,
`.roles/parliament/adapter-boundary-steward.md`, and `crates/racket-core/src/lib.rs`.

## RACKET-P-03: Unsupported Intent Must Be Diagnosed

**Decision rule:** Unsupported COURT scene roles, scene features, action
states, provenance boundaries, and assessment intent must produce explicit
diagnostics instead of silent loss.

**Rationale:** Silent dropping makes the adapter look ready while losing player
or product intent.

**Test:** Proof-surface tests retain ready and not-ready reports with structured
diagnostics.

**Evidence:** `crates/racket-core/tests/proof_surface.rs`,
`crates/racket-core/tests/fixtures/runtime-proof.json`, and
`docs/vtrace/VALIDATION.md`.

## RACKET-P-04: Deterministic Runtime Comes Before Renderer Choice

**Decision rule:** RACKET proves deterministic, windowless frame stepping before
choosing Macroquad, browser, or another renderer/backend/input stack.

**Rationale:** Renderer realism should not weaken replayable adapter proof.

**Test:** Runtime Determinism Auditor review and runtime-loop tests gate backend
expansion.

**Evidence:** `.roles/parliament/runtime-determinism-auditor.md`,
`docs/vtrace/WORK_PACKAGES.md`, and `cargo test --workspace`.

## RACKET-P-05: RUNE Descriptors Are Evidence, Not Ownership

**Decision rule:** RUNE descriptors describe frame plans, diagnostics, runtime
config, frames, and reports for agent inspection; they do not transfer COURT or
product ownership into RACKET.

**Rationale:** AI-readable metadata is useful only if it preserves the same
ownership boundaries as the Rust API.

**Test:** Descriptor fixture tests compare retained RUNE metadata to
`racket_core::rune_descriptor_collection()`.

**Evidence:** `docs/rune/README.md`, `docs/rune/adapter_contracts.json`, and
`cargo test rune_contract_registry_matches_retained_fixture`.

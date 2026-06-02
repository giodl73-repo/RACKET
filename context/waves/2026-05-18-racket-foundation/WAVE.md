# Wave: RACKET Foundation

## Goal

Create RACKET as the first real engine adapter for COURT and prove it can
consume a COURT snapshot without owning product rules.

## Thesis

The scalable experience framework only matters if at least one engine can run
it. RACKET starts as a small Rust-native adapter that translates COURT contracts
into engine-side frame plans.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Workspace foundation | complete | Created repo skeleton, docs, skills, and first COURT-consuming frame plan. |
| 02 | Wimbledon diagnostics | complete | Added adapter compatibility diagnostics for COURT action, scene, provenance, and unsupported-feature boundaries. |
| 03 | Native loop spike | complete | Added a deterministic windowless runtime loop over frame plans. |
| 04 | Product fixture smokes | complete | AMAZE Prism Vault and TIGRIS Parliament run through RACKET diagnostics/runtime tests from product repos. |
| 05 | Foundation closeout | complete | RACKET pauses at deterministic frame plans, diagnostics, and windowless runtime loops. |
| 06 | Browser/engine comparison | deferred | Compare RACKET's native proof against browser UX needs only when a product fixture requires it. |
| 07 | RUNE adapter contracts | complete | Retained RUNE descriptors for frame-plan, diagnostic, and windowless runtime evidence. |

## Success criteria

- README explains the repo purpose and first command.
- Product plan names COURT as the upstream contract.
- Wave/pulse scaffolding exists.
- Skills exist for future wave and pulse execution.
- Adapter diagnostics report unsupported COURT features without owning product
  rules.
- Runtime loop can step deterministic frame plans without opening a window,
  rendering, or executing product rules.
- Product repos can smoke COURT fixtures through RACKET without moving product
  rules into `racket-core`.
- Closeout records renderer/backend/input selection as deferred.
- RUNE adapter contracts expose frame-plan/runtime evidence without changing the
  COURT or product-rule boundary.
- `cargo test --quiet` passes.

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `git diff --check`


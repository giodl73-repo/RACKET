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
| 04 | Browser/engine comparison | pending | Compare RACKET's native proof against browser UX needs. |

## Success criteria

- README explains the repo purpose and first command.
- Product plan names COURT as the upstream contract.
- Wave/pulse scaffolding exists.
- Skills exist for future wave and pulse execution.
- Adapter diagnostics report unsupported COURT features without owning product
  rules.
- Runtime loop can step deterministic frame plans without opening a window,
  rendering, or executing product rules.
- `cargo test --quiet` passes.

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `git diff --check`



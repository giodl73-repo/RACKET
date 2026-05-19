# Pulse 03: Native loop spike

## Goal

Add the smallest deterministic runtime loop over COURT-derived RACKET frame plans
without opening a window, rendering, or executing product rules.

## Changes

- Add `RacketRuntimeConfig`.
- Add `RacketRuntimeFrame`.
- Add `RacketRuntimeReport`.
- Add `run_runtime_loop()` over a static COURT snapshot.
- Add tests proving deterministic frame count, readiness, diagnostics propagation,
  and no renderer/backend dependency.

## Boundary

RACKET still only consumes COURT snapshots. It does not mutate product state,
interpret product rules, choose a renderer, or replace MUDDLE.

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `git diff --check`

## Status

Complete.


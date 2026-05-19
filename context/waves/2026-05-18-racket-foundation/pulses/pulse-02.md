# Pulse 02: Wimbledon diagnostics

## Goal

Make RACKET's COURT adapter compatibility boundary explicit before adding a
runtime loop or renderer backend.

## Changes

- Add `RacketAdapterDiagnostic`.
- Add frame-plan diagnostics for unavailable, guided-illegal, and diagnostic-only
  actions.
- Add diagnostics for unsupported scene roles.
- Add diagnostics for non-product-authored provenance boundaries.
- Preserve unsupported scene-feature hints as adapter diagnostics.

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `git diff --check`

## Status

Complete.


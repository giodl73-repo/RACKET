# Pulse 07: RUNE adapter contracts

## Goal

Expose RACKET's frame-plan, diagnostic, and windowless runtime records as
retained RUNE descriptor evidence.

## Changes

- Add pinned RUNE descriptor dependencies to `racket-core`.
- Derive RUNE contracts for the stable RACKET adapter data records.
- Add `racket_core::rune_descriptor_collection()` and retained evidence at
  `docs\rune\adapter_contracts.json`.
- Document the RUNE boundary in `docs\rune\README.md`.

## Outcome

AI and portfolio tools can now inspect RACKET's COURT adapter evidence without
source scraping, while product rules and renderer/backend decisions remain out of
scope.

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `git diff --check`

## Status

Complete.


# RACKET RUNE contracts

RACKET exposes its COURT adapter boundary as RUNE descriptor evidence so AI
agents and portfolio tooling can inspect frame-plan, diagnostic, and windowless
runtime outputs without scraping Rust source.

## Retained evidence

- `docs\rune\adapter_contracts.json` is generated from
  `racket_core::rune_descriptor_collection()`.
- The collection id is `racket.adapter_contracts`.
- The first slice covers frame plans, adapter diagnostics, runtime config,
  runtime frames, and runtime reports.

## Boundary

RUNE metadata describes RACKET's product-neutral adapter and runtime evidence
only. COURT remains the source for experience contracts, and product repos keep
rules, fantasy, content, renderer choices, and product-owned conclusions.


---
name: racket-pulse
description: Execute one RACKET pulse with validation.
allowed-tools:
  - Read
  - Write
  - Glob
  - Grep
  - Bash
---

# RACKET Pulse

Use this skill to execute a single RACKET pulse.

## Pulse requirements

- Read `context/waves/PHASES.md`.
- Read the active `WAVE.md`.
- Keep `racket-core` product-neutral.
- Run `cargo fmt --check`, `cargo test --quiet`, and `git diff --check`.
- Update pulse status before commit.


# RACKET Product Plan

## Thesis

COURT needs at least one real engine to prove the scalable experience framework.
RACKET is the Rust-native engine adapter that consumes COURT snapshots and
translates them into frame plans, input affordances, and eventually rendered
surfaces.

## Product promise

Given a COURT snapshot, RACKET can produce a native engine plan without knowing
the product's rules.

## Dependency placement

| System | Responsibility |
|---|---|
| COURT | Portable scalable experience contracts. |
| RACKET | First native engine adapter over COURT contracts. |
| Product repos | Domain rules, product state, and scene meaning. |

## First wave

1. Define `racket-core`.
2. Consume `court-core` by path while remotes/submodules are being established.
3. Translate a COURT snapshot into a frame plan.
4. Prepare for a future Macroquad or browser-backed renderer adapter.

## Non-goals

- RACKET does not own product rules.
- RACKET does not replace Macroquad immediately; it starts as an engine plan
  layer and can adopt rendering backend(s) once the contract is stable.
- RACKET does not choose final art, animation, or asset workflows in the first
  wave.


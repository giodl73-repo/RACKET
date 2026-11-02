# RACKET Product Plan

## Thesis

COURT needs at least one real engine to prove the scalable experience framework.
RACKET is the Rust-native engine adapter that consumes COURT snapshots and
translates them into frame plans, input affordances, and eventually rendered
surfaces.

## Product promise

Given a COURT snapshot, RACKET can produce a native engine plan without knowing
the product's rules. The first runtime loop is deterministic and windowless: it
steps frame plans for smoke validation without choosing a renderer. Passing
adapter proof is not product readiness; readiness belongs to the product repo
that owns rules, assets, scene direction, runtime policy, and user-facing tests.

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
4. Report adapter diagnostics for unsupported COURT features.
5. Step deterministic frame plans in a windowless runtime loop.
6. Prepare for a future Macroquad or browser-backed renderer adapter.
7. Retain RUNE descriptor evidence for the stable adapter records so AI agents
   can inspect frame-plan and runtime evidence without source scraping.

## Review gate

Before RACKET adds a runtime loop or backend, align with COURT's
`specs\engine-adapter-contract.md`:

- consume reviewed COURT snapshot fields,
- preserve action availability and scene role intent,
- report unsupported features explicitly,
- keep product rules out of `racket-core`,
- prove deterministic frame-plan smoke checks.

## Non-goals

- RACKET does not own product rules.
- RACKET does not replace Macroquad immediately; it starts as an engine plan
  layer and can adopt rendering backend(s) once the contract is stable.
- RACKET does not choose final art, animation, or asset workflows in the first
  wave.
- RACKET does not add renderer, backend, input, or asset-pipeline commitments
  without a named product fixture need and VTRACE work package.

# RACKET

RACKET is the first real engine for COURT scalable experiences.

RACKET consumes COURT contracts and turns snapshots into native engine frame
plans. It should prove rendering/input/runtime behavior without owning product
rules.

RACKET's next expansion is gated by COURT's MIT-course-grounded specs, especially
`..\court\specs\engine-adapter-contract.md`.

## First command

```powershell
cargo test --quiet
```

## Relationship to COURT

- COURT defines portable state/action/snapshot/scene contracts.
- RACKET consumes those contracts as an engine adapter.
- Product repos own rules, fantasy, and scene direction.
- RACKET reports unsupported COURT features explicitly instead of silently
  dropping scene, action, provenance, or assessment intent.
- RACKET can step deterministic frame plans without opening a window; renderer
  selection remains future work.

## Current stop point

The foundation wave is wrapped at frame plans, compatibility diagnostics, and a
deterministic windowless runtime loop. AMAZE Prism Vault and TIGRIS Parliament
exercise RACKET from their product repos. Renderer, backend, and input selection
remain deferred until a product fixture needs them.

## RUNE contract evidence

RACKET retains AI-readable RUNE descriptor evidence for its adapter contracts at
`docs\rune\adapter_contracts.json`. See `docs\rune\README.md` for the boundary:
descriptors cover frame plans, diagnostics, and windowless runtime reports while
COURT owns experience contracts and product repos own rules/content.

## Non-goals

- No product-specific gameplay in `racket-core`.
- No full asset editor in the foundation wave.
- No fork of COURT contracts.

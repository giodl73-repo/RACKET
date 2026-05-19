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

## Non-goals

- No product-specific gameplay in `racket-core`.
- No full asset editor in the foundation wave.
- No fork of COURT contracts.


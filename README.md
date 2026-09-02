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

Focused retained proof:

```powershell
cargo test --quiet --test proof_surface
```

`crates/racket-core/tests/fixtures/runtime-proof.json` records one accepted
ready report and one structured not-ready report with an `action-unavailable`
diagnostic. The integration test exercises the public COURT-to-RACKET adapter
path and compares both outcomes to that retained evidence. The proof is
adapter compatibility evidence only; product readiness remains owned by the
product repo's rules, assets, scene direction, runtime policy, and user-facing
tests.

## Relationship to COURT

- COURT defines portable state/action/snapshot/scene contracts.
- COURT's
  [compatibility policy](https://github.com/giodl73-repo/COURT/blob/main/docs/compatibility.md)
  makes RACKET the mandatory first downstream breakage rehearsal and defines
  the migration or last-passing-revision rollback path.
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
remain deferred until a named product fixture needs them and a VTRACE work
package records the role-reviewed expansion.

## RUNE contract evidence

RACKET retains AI-readable RUNE descriptor evidence for its adapter contracts at
`docs\rune\adapter_contracts.json`. See `docs\rune\README.md` for the boundary:
descriptors cover frame plans, diagnostics, and windowless runtime reports while
COURT owns experience contracts and product repos own rules/content.

## Non-goals

- No product-specific gameplay in `racket-core`.
- No full asset editor in the foundation wave.
- No fork of COURT contracts.

## License

RACKET uses separate licenses for software and content. Source code,
executable scripts, tests, configuration, and ordinary software
documentation are MIT-licensed (copyright Gio Della-Libera). Original
non-software content is licensed CC BY-NC 4.0 (copyright Gio Della-Libera);
commercial use of that content requires separate written permission.
Third-party material remains under its own terms.
See [LICENSE](./LICENSE) for the complete notice.

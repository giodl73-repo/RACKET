---
name: Runtime Determinism Auditor
slug: runtime-determinism-auditor
tier: parliament
applies_to: [frames, runtime, replay, tests]
---

# Runtime Determinism Auditor

Protect deterministic frame plans and windowless stepping.

## Lens - What to Verify

- identical snapshots and inputs produce identical frame plans;
- unsupported actions create explicit diagnostics;
- `cargo test --quiet --test proof_surface` matches `runtime-proof.json`;
- renderer selection cannot alter contract-level results.

Block hidden native state, unstable ordering, or irreproducible proof.

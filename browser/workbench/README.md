# RACKET windowless frame workbench

The browser client presents unchanged `RacketFramePlan::from_snapshot` and
`run_runtime_loop` outputs for your own COURT snapshot. COURT's inspector owns
strict JSON admission and snapshot mapping; RACKET maintains no DTO fork.
The default fixture is a synthetic authored diagnostic table, not TIGRIS content.
TIGRIS Parliament's existing fixture supplies the named diagnostic use case;
product adoption/release decisions remain in that repository.

Native `is_scene_ready` counts Surface nodes and player-available commands.
Diagnostics do not veto it. Native completion requires count readiness and at
least one frame. Every frame repeats the same plan; no actions or game state
updates execute. Empty/not-ready snapshots still emit the requested frames.
Reports preserve raw native flags rather than promoting them to renderer,
product, asset, provenance or assessment readiness.

Limits belong to this workbench: 128 KiB UTF-8 snapshots, COURT's strict enums,
signed i32 geometry, 256 actions/nodes, 0–32 decimal frame count, a 1 MiB expanded
plan budget checked before runtime cloning, a compiled 64 MiB WASM memory maximum,
and real 5/12/10 second read/startup/compute deadlines. No truncation or sampling.
The frame count is textual at the Rust boundary to reject fractional or overflowing
values. Unknown JSON extensions reject whole input. Source strings stay inert.

Opened originals retain BOM/CRLF independently of edits and explicit LF copies.
Changing snapshot or frame count clears exports and fences older work/read jobs.
Current text downloads preserve the loaded text until edited; report exports
retain the admitted snapshot, requested count, each plan, and COURT observations.

## Qualification

Rust 1.95.0, wasm32-unknown-unknown, wasm-bindgen CLI 0.2.127, Python 3.11+,
Node 22 and Playwright 1.61.1. Set an absolute CARGO_TARGET_DIR; no TRACKER paths
or local dependency overrides are needed for the committed locks.

```text
cargo +1.95.0 test --workspace --locked
cargo +1.95.0 test --manifest-path browser/workbench/Cargo.toml --locked
cargo +1.95.0 clippy --manifest-path browser/workbench/Cargo.toml --locked --all-targets -- -D warnings
python browser/workbench/build-prototype.py --bindgen /absolute/path/to/wasm-bindgen
python browser/workbench/package-site.py
python browser/workbench/verify-site.py target/racket-release/RACKET/workbench --commit <HEAD>
```

From browser/workbench: `npm ci --ignore-scripts`, `npx playwright install chromium`,
`npm run test:pages`. It serves the complete package at `/RACKET/workbench/` on
port 4193. Packaging requires a clean Git checkout and unchanged native foundation.
`python browser/workbench/verify-source.py --bindgen /absolute/path/to/wasm-bindgen`
extracts the exact delivered ZIP into a path with spaces outside every Git checkout,
checks first-party manifest containment, and repeats native/build/browser behavior.
Source replay omits only the two original-artifact delivery checks, which require
the original package. It makes no cross-directory binary identity claim.

The native CLI reads bounded stdin snapshot bytes and accepts frame count as its
first argument (default 3). Invalid UTF-8 exits nonzero; admission errors produce
JSON. Snapshot/native inspection exports are diagnostic tools, not a framework
wire-format standard or an action execution interface.

Status and exact retained qualification: see the repo-local wave. Hosted candidate,
main deployment, and live verification are separate delivery gates.

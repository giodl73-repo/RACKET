# RACKET build and source delivery

Use browser/workbench/README.md from the complete source.zip for pinned compiler,
bindings, native and browser commands. Rust 1.95.0, wasm-bindgen CLI 0.2.127,
Python 3.11+, Node 22 and Playwright 1.61.1 are required. Set CARGO_TARGET_DIR
to an absolute build directory. The locks pin COURT and RUNE Git dependencies;
Cargo must fetch those dependencies and crates. The ZIP is not an offline vendor
archive. No TRACKER-relative path, secret, or local override is required.

Compiled WASM is pkg/racket_workbench_bg.wasm and JavaScript bindings are
pkg/racket_workbench.js. Complete first-party source is source.zip, independently
hashed in source-files.json. Build provenance identifies the Git revision.
Native source/locks/RUNE/policy tests and runtime fixture remain in the archive.
Dependencies have their original notices in licenses/; Rust notices come from
Rust 1.95.0. Original non-software content retains CC BY-NC 4.0 and prior grants;
software and ordinary software documentation are MIT under the root LICENSE.

Build-prototype.py applies the 64 MiB memory ceiling to the actual release WASM,
then uses the native CLI to retain exact request/response cases for browser parity.
Package-site.py only stages complete source, legal and runtime files; private QA
requests, local Cargo metadata, logs and screenshots stay outside the served tree.
Verify-site.py checks the exact Git source and file inventory, including a 5 MB
site budget. Hosted CI repeats behavior and independent extracted-source replay
before deployment from main. Fresh live checks remain required separately.

The diagnostic client uses existing windowless functions. Native count readiness
and completion do not imply rendering, asset/product readiness or assessment.

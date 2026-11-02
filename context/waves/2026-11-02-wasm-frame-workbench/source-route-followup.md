# Source-route delivery follow-up

The initial workbench copied an upstream crate path into its Native Rust source
link: the repository was RACKET but the path still named crates/court-core/src.
The complete RACKET source ZIP worked; direct source navigation was incorrect.
The full delivery checks previously asserted discoverability without checking
this external route's owning crate. Keep that prior scope explicit.

Correct the source link to crates/racket-core/src and require that exact owning
route in the packaged delivery browser case. Correct the licenses page title
from COURT to RACKET while retaining every original notice and license body.
No Rust, runtime, input contract, flags, source grants or renderer gate changes.

Agent review through Adapter Boundary Steward and Product Integrator lenses:
this closes WP-RCK-004's source-discovery delivery requirement for the existing
TIGRIS diagnostic use case; it does not add renderer or product capability.
Repeat exact package/source/browser hosted qualification and main/live checks
before snapshotting the follow-up. The renderer/backend gate remains open.

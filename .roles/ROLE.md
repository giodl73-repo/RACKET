# RACKET Review Panel

Use this panel for COURT adapter, deterministic frame-plan, diagnostic, and
windowless runtime changes.

## Active Roles

| Role | Protects | Invoke when |
|---|---|---|
| [Adapter Boundary Steward](parliament/adapter-boundary-steward.md) | COURT ownership and product neutrality | Changing contract mapping or product hooks |
| [Runtime Determinism Auditor](parliament/runtime-determinism-auditor.md) | Repeatable frame behavior | Changing frame plans, stepping, or input handling |
| [Compatibility Gatekeeper](parliament/compatibility-gatekeeper.md) | Explicit downstream breakage | Changing COURT consumption or diagnostics |
| [Product Integrator](stakeholders/product-integrator.md) | Honest adoption value | Adding renderer, backend, input, or product fixtures |

## Core Tensions

| Pulls | Against | Because |
|---|---|---|
| Adapter Boundary Steward | Product Integrator | Product convenience can make RACKET own rules that belong above COURT. |
| Runtime Determinism Auditor | Product Integrator | Native-engine realism can weaken windowless replay and proof. |
| Compatibility Gatekeeper | Adapter Boundary Steward | Temporary compatibility code can preserve migration while enlarging the adapter. |

## Review Order

1. Compatibility Gatekeeper establishes COURT contract behavior.
2. Runtime Determinism Auditor proves frame results.
3. Adapter Boundary Steward enforces ownership.
4. Product Integrator evaluates whether expansion is earned by a fixture.

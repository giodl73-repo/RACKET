use court_core::{
    CourtAction, CourtActionAvailability, CourtExperience, CourtExperienceIntent, CourtProvenance,
    CourtSceneNode, CourtSceneRole, CourtSnapshot, CourtSnapshotMetadata, CourtSurfaceKind,
};
use racket_core::{run_runtime_loop, RacketRuntimeConfig};

#[test]
fn open_pitfalls_remain_adapter_and_fixture_boundaries() {
    let pitfalls = repo_text(".pitfall/racket-pitfalls.md");
    for id in ["RACKET-PF-01", "RACKET-PF-02", "RACKET-PF-03"] {
        assert_contains(&pitfalls, id);
    }
    for field in [
        "Actor:",
        "Task:",
        "Surface:",
        "Likely mistake:",
        "Consequence:",
        "Owner:",
        "cargo test --quiet --test pitfall_policy",
    ] {
        assert_contains(&pitfalls, field);
    }
    assert_contains(&pitfalls, "MITIGATED");

    let boundary_manifest = repo_text("docs/vtrace/pitfall-boundaries.v1.json");
    for required in [
        "RACKET-PF-01",
        "RACKET-PF-02",
        "RACKET-PF-03",
        "product readiness",
        "renderer work is next by default",
        "RACKET-owned COURT schema",
        "named product fixture need",
        "VTRACE work-package creation",
        "COURT reviewed snapshot field",
    ] {
        assert_contains(&boundary_manifest, required);
    }

    let roles = repo_text(".roles/ROLE.md");
    for required in [
        "PITFALL gate routing",
        "Product Integrator",
        "Adapter Boundary Steward",
        "Runtime Determinism Auditor",
        "Compatibility Gatekeeper",
        "named product fixture need",
        "could fork",
        "COURT contract truth",
    ] {
        assert_contains(&roles, required);
    }

    let accepted = run_runtime_loop(
        &snapshot(CourtActionAvailability::Legal),
        RacketRuntimeConfig { max_frames: 2 },
    );
    assert!(accepted.completed);
    assert_eq!(accepted.ready_frame_count(), 2);
    assert!(accepted.diagnostics.is_empty());

    let rejected = run_runtime_loop(
        &snapshot(CourtActionAvailability::Unavailable {
            reason: "The product disabled this action.".to_string(),
        }),
        RacketRuntimeConfig { max_frames: 2 },
    );
    assert!(!rejected.completed);
    assert_eq!(rejected.ready_frame_count(), 0);
    assert!(rejected
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "action-unavailable"));

    let readme = repo_text("README.md");
    assert_contains(
        &readme,
        "Product repos own rules, fantasy, and scene direction",
    );
    assert_contains(&readme, "Renderer, backend, and input selection");
    assert_contains(
        &readme,
        "remain deferred until a named product fixture needs them",
    );
    assert_contains(&readme, "adapter compatibility evidence only");
    assert_contains(&readme, "No fork of COURT contracts");

    let product_plan = repo_text("PRODUCT_PLAN.md");
    assert_contains(&product_plan, "without knowing");
    assert_contains(&product_plan, "the product's rules");
    assert_contains(&product_plan, "once the contract is stable");
    assert_contains(&product_plan, "Passing");
    assert_contains(&product_plan, "adapter proof is not product readiness");

    let interfaces = repo_text("docs/vtrace/INTERFACES.md");
    assert_contains(&interfaces, "RACKET now exposes `.roles/ROLE.md`");
    assert_contains(
        &interfaces,
        "older product-lens names map to those local roles",
    );

    let rune_contracts = repo_text("docs/rune/adapter_contracts.json");
    assert_contains(&rune_contracts, "racket.frame_plan");
    assert_contains(&rune_contracts, "court.scene.v1");
}

fn snapshot(availability: CourtActionAvailability) -> CourtSnapshot {
    CourtSnapshot {
        metadata: CourtSnapshotMetadata {
            experience_id: "proof".to_string(),
            experience_version: "0.1.0".to_string(),
            surface: CourtSurfaceKind::Native2d,
            scene_contract_version: "court.scene.v1".to_string(),
        },
        experience: CourtExperience {
            id: "proof".to_string(),
            title: "RACKET Proof".to_string(),
            surface: CourtSurfaceKind::Native2d,
            intent: CourtExperienceIntent {
                product_owner: "RACKET".to_string(),
                audience: "Adapter maintainers".to_string(),
                design_thesis: "Runtime readiness follows explicit COURT state.".to_string(),
                non_goals: vec!["Do not execute product rules.".to_string()],
            },
            provenance: CourtProvenance::product_authored("racket:proof"),
        },
        state_label: "proof".to_string(),
        actions: vec![CourtAction {
            id: "serve".to_string(),
            label: "Serve".to_string(),
            command: "serve".to_string(),
            availability,
        }],
        scene: vec![CourtSceneNode {
            id: "scene".to_string(),
            label: "Scene".to_string(),
            player_read_label: "Proof scene".to_string(),
            product_meaning: "A retained adapter proof.".to_string(),
            role: CourtSceneRole::Surface,
            x: 0,
            y: 0,
            width: 12,
            height: 8,
            provenance: Some(CourtProvenance::product_authored("racket:proof:scene")),
            unsupported_features: Vec::new(),
        }],
    }
}

fn repo_text(relative: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..");
    std::fs::read_to_string(root.join(relative)).expect(relative)
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(haystack.contains(needle), "missing {needle}");
}

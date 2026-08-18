use court_core::{
    CourtAction, CourtActionAvailability, CourtExperience, CourtExperienceIntent, CourtProvenance,
    CourtSceneNode, CourtSceneRole, CourtSnapshot, CourtSnapshotMetadata, CourtSurfaceKind,
};
use racket_core::{run_runtime_loop, RacketRuntimeConfig, RacketRuntimeReport};
use serde_json::{json, Value};

#[test]
fn retained_proof_covers_ready_and_not_ready_reports() {
    let accepted = run_runtime_loop(
        &snapshot(CourtActionAvailability::Legal, CourtSceneRole::Surface),
        RacketRuntimeConfig { max_frames: 2 },
    );
    let rejected = run_runtime_loop(
        &snapshot(
            CourtActionAvailability::Unavailable {
                reason: "The product disabled this action.".to_string(),
            },
            CourtSceneRole::Surface,
        ),
        RacketRuntimeConfig { max_frames: 2 },
    );
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/runtime-proof.json")).expect("proof fixture");

    assert_eq!(report_evidence(&accepted), expected["accepted"]);
    assert_eq!(report_evidence(&rejected), expected["rejected"]);
}

fn report_evidence(report: &RacketRuntimeReport) -> Value {
    json!({
        "completed": report.completed,
        "frameCount": report.frames.len(),
        "readyFrameCount": report.ready_frame_count(),
        "diagnostics": report
            .diagnostics
            .iter()
            .map(|diagnostic| json!({
                "code": diagnostic.code,
                "message": diagnostic.message,
            }))
            .collect::<Vec<_>>(),
    })
}

fn snapshot(availability: CourtActionAvailability, scene_role: CourtSceneRole) -> CourtSnapshot {
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
            role: scene_role,
            x: 0,
            y: 0,
            width: 12,
            height: 8,
            provenance: Some(CourtProvenance::product_authored("racket:proof:scene")),
            unsupported_features: Vec::new(),
        }],
    }
}

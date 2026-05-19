//! RACKET is the first real engine adapter for COURT.
//!
//! It consumes COURT snapshots and translates them into renderable engine plans.
//! Product rules remain in product repos; RACKET owns engine-side interpretation.

use court_core::{
    CourtActionAvailability, CourtProvenanceClass, CourtSceneRole, CourtSnapshot, CourtSurfaceKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RacketFramePlan {
    pub title: String,
    pub surface: CourtSurfaceKind,
    pub experience_version: String,
    pub scene_contract_version: String,
    pub command_count: usize,
    pub player_command_count: usize,
    pub surface_nodes: usize,
    pub actor_nodes: usize,
    pub prop_nodes: usize,
    pub unsupported_scene_feature_count: usize,
    pub diagnostics: Vec<RacketAdapterDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RacketAdapterDiagnostic {
    pub code: String,
    pub message: String,
}

impl RacketFramePlan {
    pub fn from_snapshot(snapshot: &CourtSnapshot) -> Self {
        let diagnostics = collect_diagnostics(snapshot);

        Self {
            title: snapshot.experience.title.clone(),
            surface: snapshot.experience.surface,
            experience_version: snapshot.metadata.experience_version.clone(),
            scene_contract_version: snapshot.metadata.scene_contract_version.clone(),
            command_count: snapshot.actions.len(),
            player_command_count: snapshot.available_commands().count(),
            surface_nodes: count_role(snapshot, CourtSceneRole::Surface),
            actor_nodes: count_role(snapshot, CourtSceneRole::Actor),
            prop_nodes: count_role(snapshot, CourtSceneRole::Prop),
            unsupported_scene_feature_count: snapshot.unsupported_scene_features().count(),
            diagnostics,
        }
    }

    pub fn is_scene_ready(&self) -> bool {
        self.surface_nodes > 0 && self.player_command_count > 0
    }

    pub fn has_diagnostic(&self, code: &str) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == code)
    }
}

fn count_role(snapshot: &CourtSnapshot, role: CourtSceneRole) -> usize {
    snapshot
        .scene
        .iter()
        .filter(|node| node.role == role)
        .count()
}

fn collect_diagnostics(snapshot: &CourtSnapshot) -> Vec<RacketAdapterDiagnostic> {
    let mut diagnostics = Vec::new();

    if !matches!(
        snapshot.experience.provenance.class,
        CourtProvenanceClass::ProductAuthored
    ) {
        diagnostics.push(RacketAdapterDiagnostic {
            code: "experience-provenance-boundary".to_string(),
            message: format!(
                "Experience '{}' uses {:?} provenance; preserve as a boundary.",
                snapshot.experience.id, snapshot.experience.provenance.class
            ),
        });
    }

    for action in &snapshot.actions {
        match &action.availability {
            CourtActionAvailability::Legal | CourtActionAvailability::Destructive { .. } => {}
            CourtActionAvailability::Unavailable { reason } => {
                diagnostics.push(RacketAdapterDiagnostic {
                    code: "action-unavailable".to_string(),
                    message: format!("Action '{}' unavailable: {}", action.id, reason),
                });
            }
            CourtActionAvailability::GuidedIllegal { guidance } => {
                diagnostics.push(RacketAdapterDiagnostic {
                    code: "action-guided-illegal".to_string(),
                    message: format!("Action '{}' requires guidance: {}", action.id, guidance),
                });
            }
            CourtActionAvailability::Diagnostic { note } => {
                diagnostics.push(RacketAdapterDiagnostic {
                    code: "diagnostic-action-skipped".to_string(),
                    message: format!("Action '{}' is diagnostic-only: {}", action.id, note),
                });
            }
        }
    }

    for node in &snapshot.scene {
        if !is_supported_scene_role(node.role) {
            diagnostics.push(RacketAdapterDiagnostic {
                code: "unsupported-scene-role".to_string(),
                message: format!(
                    "Scene node '{}' has unsupported role {:?}",
                    node.id, node.role
                ),
            });
        }

        if let Some(provenance) = &node.provenance {
            if !matches!(provenance.class, CourtProvenanceClass::ProductAuthored) {
                diagnostics.push(RacketAdapterDiagnostic {
                    code: "scene-provenance-boundary".to_string(),
                    message: format!(
                        "Scene node '{}' uses {:?} provenance; preserve as a boundary.",
                        node.id, provenance.class
                    ),
                });
            }
        }

        for feature in &node.unsupported_features {
            diagnostics.push(RacketAdapterDiagnostic {
                code: "unsupported-scene-feature".to_string(),
                message: format!(
                    "Scene node '{}' requests unsupported feature '{}'; fallback: {}",
                    node.id, feature.feature, feature.fallback
                ),
            });
        }
    }

    diagnostics
}

fn is_supported_scene_role(role: CourtSceneRole) -> bool {
    matches!(
        role,
        CourtSceneRole::Surface
            | CourtSceneRole::Zone
            | CourtSceneRole::Actor
            | CourtSceneRole::Prop
            | CourtSceneRole::Hud
            | CourtSceneRole::Text
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use court_core::{
        CourtAction, CourtActionAvailability, CourtExperience, CourtExperienceIntent,
        CourtProvenance, CourtProvenanceClass, CourtSceneNode, CourtSceneRole, CourtSnapshot,
        CourtSnapshotMetadata, CourtSurfaceKind, CourtUnsupportedFeatureHint,
    };

    #[test]
    fn frame_plan_consumes_court_snapshot_without_rules() {
        let snapshot = CourtSnapshot {
            metadata: CourtSnapshotMetadata {
                experience_id: "demo".to_string(),
                experience_version: "0.1.0".to_string(),
                surface: CourtSurfaceKind::Native2d,
                scene_contract_version: "court.scene.v1".to_string(),
            },
            experience: CourtExperience {
                id: "demo".to_string(),
                title: "Demo Court".to_string(),
                surface: CourtSurfaceKind::Native2d,
                intent: CourtExperienceIntent {
                    product_owner: "RACKET".to_string(),
                    audience: "Engine adapter reviewers".to_string(),
                    design_thesis: "A COURT snapshot can become an engine frame plan.".to_string(),
                    non_goals: vec!["Do not own product rules.".to_string()],
                },
                provenance: CourtProvenance::product_authored("racket:demo"),
            },
            state_label: "ready".to_string(),
            actions: vec![
                CourtAction {
                    id: "serve".to_string(),
                    label: "Serve".to_string(),
                    command: "serve".to_string(),
                    availability: CourtActionAvailability::Legal,
                },
                CourtAction {
                    id: "inspect-hidden".to_string(),
                    label: "Inspect hidden diagnostics".to_string(),
                    command: "inspect hidden".to_string(),
                    availability: CourtActionAvailability::Diagnostic {
                        note: "Harness only.".to_string(),
                    },
                },
                CourtAction {
                    id: "jump-net".to_string(),
                    label: "Jump the net".to_string(),
                    command: "jump net".to_string(),
                    availability: CourtActionAvailability::GuidedIllegal {
                        guidance: "Use a legal movement action.".to_string(),
                    },
                },
            ],
            scene: vec![
                CourtSceneNode {
                    id: "court".to_string(),
                    label: "Court".to_string(),
                    player_read_label: "A tennis court".to_string(),
                    product_meaning: "The main play surface.".to_string(),
                    role: CourtSceneRole::Surface,
                    x: 0,
                    y: 0,
                    width: 12,
                    height: 8,
                    provenance: Some(CourtProvenance::product_authored("racket:demo:court")),
                    unsupported_features: vec![CourtUnsupportedFeatureHint {
                        feature: "clay-particle-effect".to_string(),
                        fallback: "Render a flat clay surface.".to_string(),
                    }],
                },
                CourtSceneNode {
                    id: "player".to_string(),
                    label: "Player".to_string(),
                    player_read_label: "Player avatar".to_string(),
                    product_meaning: "The actor controlled by product state.".to_string(),
                    role: CourtSceneRole::Actor,
                    x: 2,
                    y: 3,
                    width: 1,
                    height: 1,
                    provenance: Some(CourtProvenance::product_authored("racket:demo:player")),
                    unsupported_features: Vec::new(),
                },
                CourtSceneNode {
                    id: "replay-video".to_string(),
                    label: "Replay video".to_string(),
                    player_read_label: "A replay screen".to_string(),
                    product_meaning: "A media-like node that this adapter cannot render yet."
                        .to_string(),
                    role: CourtSceneRole::Media,
                    x: 6,
                    y: 1,
                    width: 4,
                    height: 3,
                    provenance: Some(CourtProvenance {
                        class: CourtProvenanceClass::MetadataOnly,
                        source_id: Some("racket:demo:replay-video".to_string()),
                    }),
                    unsupported_features: Vec::new(),
                },
            ],
        };

        let plan = RacketFramePlan::from_snapshot(&snapshot);

        assert_eq!(plan.title, "Demo Court");
        assert_eq!(plan.experience_version, "0.1.0");
        assert_eq!(plan.scene_contract_version, "court.scene.v1");
        assert_eq!(plan.command_count, 3);
        assert_eq!(plan.player_command_count, 1);
        assert_eq!(plan.surface_nodes, 1);
        assert_eq!(plan.actor_nodes, 1);
        assert_eq!(plan.unsupported_scene_feature_count, 1);
        assert!(plan.has_diagnostic("diagnostic-action-skipped"));
        assert!(plan.has_diagnostic("action-guided-illegal"));
        assert!(plan.has_diagnostic("unsupported-scene-feature"));
        assert!(plan.has_diagnostic("unsupported-scene-role"));
        assert!(plan.has_diagnostic("scene-provenance-boundary"));
        assert!(plan.is_scene_ready());
    }
}

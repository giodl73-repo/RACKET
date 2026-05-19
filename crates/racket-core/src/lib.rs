//! RACKET is the first real engine adapter for COURT.
//!
//! It consumes COURT snapshots and translates them into renderable engine plans.
//! Product rules remain in product repos; RACKET owns engine-side interpretation.

use court_core::{CourtSceneRole, CourtSnapshot, CourtSurfaceKind};

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
}

impl RacketFramePlan {
    pub fn from_snapshot(snapshot: &CourtSnapshot) -> Self {
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
        }
    }

    pub fn is_scene_ready(&self) -> bool {
        self.surface_nodes > 0 && self.player_command_count > 0
    }
}

fn count_role(snapshot: &CourtSnapshot, role: CourtSceneRole) -> usize {
    snapshot
        .scene
        .iter()
        .filter(|node| node.role == role)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use court_core::{
        CourtAction, CourtActionAvailability, CourtExperience, CourtExperienceIntent,
        CourtProvenance, CourtSceneNode, CourtSnapshot, CourtSnapshotMetadata, CourtSurfaceKind,
        CourtUnsupportedFeatureHint,
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
            ],
        };

        let plan = RacketFramePlan::from_snapshot(&snapshot);

        assert_eq!(plan.title, "Demo Court");
        assert_eq!(plan.experience_version, "0.1.0");
        assert_eq!(plan.scene_contract_version, "court.scene.v1");
        assert_eq!(plan.command_count, 2);
        assert_eq!(plan.player_command_count, 1);
        assert_eq!(plan.surface_nodes, 1);
        assert_eq!(plan.actor_nodes, 1);
        assert_eq!(plan.unsupported_scene_feature_count, 1);
        assert!(plan.is_scene_ready());
    }
}

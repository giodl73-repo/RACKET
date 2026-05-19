//! RACKET is the first real engine adapter for COURT.
//!
//! It consumes COURT snapshots and translates them into renderable engine plans.
//! Product rules remain in product repos; RACKET owns engine-side interpretation.

use court_core::{CourtSceneRole, CourtSnapshot, CourtSurfaceKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RacketFramePlan {
    pub title: String,
    pub surface: CourtSurfaceKind,
    pub command_count: usize,
    pub player_command_count: usize,
    pub surface_nodes: usize,
    pub actor_nodes: usize,
    pub prop_nodes: usize,
}

impl RacketFramePlan {
    pub fn from_snapshot(snapshot: &CourtSnapshot) -> Self {
        Self {
            title: snapshot.experience.title.clone(),
            surface: snapshot.experience.surface,
            command_count: snapshot.actions.len(),
            player_command_count: snapshot.available_commands().count(),
            surface_nodes: count_role(snapshot, CourtSceneRole::Surface),
            actor_nodes: count_role(snapshot, CourtSceneRole::Actor),
            prop_nodes: count_role(snapshot, CourtSceneRole::Prop),
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
        CourtProvenance, CourtSceneNode, CourtSnapshot, CourtSurfaceKind,
    };

    #[test]
    fn frame_plan_consumes_court_snapshot_without_rules() {
        let snapshot = CourtSnapshot {
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
                    role: CourtSceneRole::Surface,
                    x: 0,
                    y: 0,
                    width: 12,
                    height: 8,
                    provenance: Some(CourtProvenance::product_authored("racket:demo:court")),
                },
                CourtSceneNode {
                    id: "player".to_string(),
                    label: "Player".to_string(),
                    role: CourtSceneRole::Actor,
                    x: 2,
                    y: 3,
                    width: 1,
                    height: 1,
                    provenance: Some(CourtProvenance::product_authored("racket:demo:player")),
                },
            ],
        };

        let plan = RacketFramePlan::from_snapshot(&snapshot);

        assert_eq!(plan.title, "Demo Court");
        assert_eq!(plan.command_count, 2);
        assert_eq!(plan.player_command_count, 1);
        assert_eq!(plan.surface_nodes, 1);
        assert_eq!(plan.actor_nodes, 1);
        assert!(plan.is_scene_ready());
    }
}

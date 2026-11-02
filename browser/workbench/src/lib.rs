//! Bounded presentation of existing windowless native behavior, not a renderer.
use racket_core::{run_runtime_loop, RacketFramePlan, RacketRuntimeConfig};
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

pub const MAX_BYTES: usize = 131_072;
pub const MAX_FRAMES: u32 = 32;
pub const MAX_EXPANDED_PLAN_BYTES: usize = 1_048_576;

fn plan_value(plan: &RacketFramePlan) -> Value {
    json!({
        "title":plan.title,"surface":format!("{:?}",plan.surface),
        "experience_version":plan.experience_version,"scene_contract_version":plan.scene_contract_version,
        "command_count":plan.command_count,"player_command_count":plan.player_command_count,
        "surface_nodes":plan.surface_nodes,"actor_nodes":plan.actor_nodes,"prop_nodes":plan.prop_nodes,
        "unsupported_scene_feature_count":plan.unsupported_scene_feature_count,
        "diagnostics":plan.diagnostics.iter().map(|d|json!({"code":d.code,"message":d.message})).collect::<Vec<_>>(),
        "is_scene_ready":plan.is_scene_ready()
    })
}

fn evaluate(text: &str, frame_text: &str) -> Result<Value, String> {
    if text.len() > MAX_BYTES {
        return Err(format!(
            "Workbench admits at most {MAX_BYTES} UTF-8 bytes; nothing was truncated."
        ));
    }
    if frame_text.is_empty()
        || frame_text.len() > 10
        || !frame_text.bytes().all(|c| c.is_ascii_digit())
    {
        return Err(format!(
            "Frame count must be decimal digits from 0 to {MAX_FRAMES}."
        ));
    }
    let max_frames = frame_text
        .parse::<u32>()
        .map_err(|_| "Frame count overflow.".to_string())?;
    if max_frames > MAX_FRAMES {
        return Err(format!("Workbench admits 0 to {MAX_FRAMES} frames."));
    }
    // COURT owns strict DTO admission and mapping. No local schema is maintained.
    let snapshot = court_inspector::decode_snapshot(text)?;
    let plan = RacketFramePlan::from_snapshot(&snapshot);
    let projected = plan_value(&plan);
    // Bound native cloning and JSON expansion before entering the runtime loop.
    let plan_bytes = serde_json::to_vec_pretty(&projected)
        .map_err(|e| e.to_string())?
        .len();
    if (plan_bytes + 128).saturating_mul(max_frames as usize + 3) > MAX_EXPANDED_PLAN_BYTES {
        return Err("Expanded frame plans exceed the 1 MiB workbench budget. Reduce input diagnostics or frame count; nothing was truncated.".into());
    }
    let runtime = run_runtime_loop(&snapshot, RacketRuntimeConfig { max_frames });
    // Retain COURT's own admitted snapshot/query observations, clearly separated.
    let mut report: Value = serde_json::from_str(&court_inspector::inspect_snapshot(text))
        .map_err(|e| e.to_string())?;
    report["scope"] = json!("Windowless frame-plan diagnostics only. Native is_scene_ready counts Surface nodes and player-available commands; diagnostics do not veto it. Completion is not rendering, action execution, product/asset readiness, provenance verification or assessment success. Every frame repeats the same snapshot; no game state evolves.");
    report["frame_count_requested"] = json!(max_frames);
    report["plan"] = projected;
    report["runtime"] = json!({
        "completed":runtime.completed,"ready_frame_count":runtime.ready_frame_count(),
        "frames":runtime.frames.iter().map(|frame|json!({"frame_index":frame.frame_index,"ready":frame.ready,"plan":plan_value(&frame.plan)})).collect::<Vec<_>>(),
        "diagnostics":runtime.diagnostics.iter().map(|d|json!({"code":d.code,"message":d.message})).collect::<Vec<_>>()
    });
    Ok(report)
}

/// The frame count is textual to reject fractional, signed and overflowing input.
#[wasm_bindgen]
pub fn inspect_frames(text: &str, frame_text: &str) -> String {
    let value =
        evaluate(text, frame_text).unwrap_or_else(|error| json!({"ok":false,"error":error}));
    serde_json::to_string_pretty(&value).expect("JSON values serialize")
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../fixtures/snapshot.json");
    fn report(text: &str, frames: &str) -> Value {
        serde_json::from_str(&inspect_frames(text, frames)).unwrap()
    }
    #[test]
    fn diagnostics_do_not_override_native_readiness_or_frame_semantics() {
        let r = report(EXAMPLE, "3");
        assert_eq!(r["ok"], true);
        assert_eq!(r["plan"]["is_scene_ready"], true);
        assert!(!r["plan"]["diagnostics"].as_array().unwrap().is_empty());
        assert_eq!(r["runtime"]["completed"], true);
        assert_eq!(r["runtime"]["ready_frame_count"], 3);
        for (i, f) in r["runtime"]["frames"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
        {
            assert_eq!(f["frame_index"], i);
            assert_eq!(f["plan"], r["plan"]);
        }
        assert_eq!(report(EXAMPLE, "0")["runtime"]["completed"], false);
    }
    #[test]
    fn not_ready_still_emits_requested_frames_and_uses_core_counts() {
        let mut s: Value = serde_json::from_str(EXAMPLE).unwrap();
        s["scene"] = json!([]);
        let r = report(&s.to_string(), "2");
        assert_eq!(r["runtime"]["frames"].as_array().unwrap().len(), 2);
        assert_eq!(r["runtime"]["completed"], false);
        assert_eq!(r["runtime"]["ready_frame_count"], 0);
    }
    #[test]
    fn strict_admission_and_limits_reject_without_normalizing() {
        for frames in ["", "-1", "1.5", " 3", "+3", "33", "4294967296"] {
            assert_eq!(report(EXAMPLE, frames)["ok"], false);
        }
        assert_eq!(report(&" ".repeat(MAX_BYTES + 1), "3")["ok"], false);
        assert_eq!(report("{}", "3")["ok"], false);
        let mut v: Value = serde_json::from_str(EXAMPLE).unwrap();
        v["scene"][0]["extra"] = json!(true);
        assert_eq!(report(&v.to_string(), "3")["ok"], false);
    }
    #[test]
    fn expansion_is_bounded_before_native_frame_cloning() {
        let mut v: Value = serde_json::from_str(EXAMPLE).unwrap();
        v["scene"][0]["unsupported_features"] =
            json!([{"feature":"x".repeat(40000),"fallback":"reduce"}]);
        let r = report(&v.to_string(), "32");
        assert_eq!(r["ok"], false);
        assert!(r["error"]
            .as_str()
            .unwrap()
            .contains("Expanded frame plans"));
        assert_eq!(report(&v.to_string(), "0")["ok"], true);
    }
}

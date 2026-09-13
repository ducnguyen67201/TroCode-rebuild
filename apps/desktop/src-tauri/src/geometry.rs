//! Validate presentation against its observed target before touching native windows.
use serde_json::Value;
fn rect(value: &Value) -> Option<[f64; 4]> {
    let result = [
        value["x"].as_f64()?,
        value["y"].as_f64()?,
        value["width"].as_f64()?,
        value["height"].as_f64()?,
    ];
    (result.iter().all(|v| v.is_finite() && v.abs() <= 100_000.0)
        && result[2] > 0.0
        && result[3] > 0.0)
        .then_some(result)
}
fn contains(outer: [f64; 4], inner: [f64; 4]) -> bool {
    outer[0] <= inner[0]
        && outer[1] <= inner[1]
        && inner[0] + inner[2] <= outer[0] + outer[2]
        && inner[1] + inner[3] <= outer[1] + outer[3]
}
pub fn grounded(state: &Value) -> bool {
    let cue = &state["cue"];
    let observation = &state["observation"];
    let Some(target) = rect(&state["target"]["bounds"]) else {
        return false;
    };
    let Some(source) = rect(&cue["source"]) else {
        return false;
    };
    if observation["target"] != state["target"]
        || cue["observation_id"] != observation["id"]
        || !contains(target, source)
    {
        return false;
    }
    let Some(elements) = observation["elements"].as_array() else {
        return false;
    };
    let matches: Vec<_> = elements
        .iter()
        .filter(|element| element["id"] == cue["element_id"])
        .collect();
    if matches.len() != 1 || rect(&matches[0]["bounds"]) != Some(source) {
        return false;
    }
    match cue["gesture"].as_str() {
        Some("drag") => {
            let Some(destination) = rect(&cue["destination"]) else {
                return false;
            };
            cue["direction"].is_null()
                && contains(target, destination)
                && elements
                    .iter()
                    .any(|element| rect(&element["bounds"]) == Some(destination))
        }
        Some("scroll") => {
            cue["destination"].is_null()
                && matches!(
                    cue["direction"].as_str(),
                    Some("up" | "down" | "left" | "right")
                )
        }
        Some("point" | "click" | "type") => {
            cue["destination"].is_null() && cue["direction"].is_null()
        }
        _ => false,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn negative_origin_and_grounding_are_checked() {
        let target = json!({"bounds":{"x":-500,"y":0,"width":500,"height":400}});
        let source = json!({"x":-400,"y":20,"width":100,"height":40});
        let mut state = json!({"target":target,"observation":{"id":"snapshot","target":target,"elements":[{"id":"one","bounds":source}]},"cue":{"gesture":"click","observation_id":"snapshot","element_id":"one","source":source,"destination":null,"direction":null}});
        assert!(grounded(&state));
        state["cue"]["source"]["x"] = json!(10);
        assert!(!grounded(&state));
        state["cue"]["source"] = source;
        state["cue"]["observation_id"] = json!("stale");
        assert!(!grounded(&state));
    }
}

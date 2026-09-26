use serde_json::{Map, Value, json};
use std::collections::BTreeSet;

pub(crate) const CURSOR_TOOL_NAMES: [&str; 5] = [
    "show_student_where",
    "show_student_click",
    "show_student_drag",
    "show_student_type",
    "show_student_scroll",
];

pub(crate) fn valid_cursor_tools(value: Option<&Value>) -> bool {
    let Some(tools) = value.and_then(Value::as_array) else {
        return false;
    };
    if tools.len() != CURSOR_TOOL_NAMES.len() {
        return false;
    }
    let mut names = BTreeSet::new();
    tools
        .iter()
        .all(|tool| valid_cursor_tool(tool.as_object(), &mut names))
        && names == CURSOR_TOOL_NAMES.iter().copied().collect::<BTreeSet<_>>()
}

fn valid_cursor_tool<'a>(
    tool: Option<&'a Map<String, Value>>,
    names: &mut BTreeSet<&'a str>,
) -> bool {
    let Some(tool) = tool else { return false };
    let allowed = ["type", "name", "description", "parameters", "strict"];
    if !tool.keys().all(|key| allowed.contains(&key.as_str()))
        || tool.get("type").and_then(Value::as_str) != Some("function")
        || tool.get("strict").and_then(Value::as_bool) != Some(true)
    {
        return false;
    }
    let Some(name) = tool.get("name").and_then(Value::as_str) else {
        return false;
    };
    if !CURSOR_TOOL_NAMES.contains(&name) || !names.insert(name) {
        return false;
    }
    if !tool
        .get("description")
        .and_then(Value::as_str)
        .is_some_and(|description| !description.is_empty() && description.len() <= 512)
    {
        return false;
    }
    let Some(parameters) = tool.get("parameters") else {
        return false;
    };
    if serde_json::to_vec(parameters).map_or(true, |body| body.len() > 32 * 1024)
        || !closed_schema(parameters, 0)
    {
        return false;
    }
    parameters == &cursor_schema(name)
}

fn cursor_target(title: &str) -> Value {
    json!({
        "anyOf": [
            {"$ref": "#/$defs/Selector"},
            {"$ref": "#/$defs/VisualTarget"}
        ],
        "title": title
    })
}

fn cursor_schema(name: &str) -> Value {
    let mut properties = Map::new();
    let required = match name {
        "show_student_drag" => {
            properties.insert("source".into(), cursor_target("Source"));
            properties.insert("destination".into(), cursor_target("Destination"));
            vec!["source", "destination", "caption", "expected"]
        }
        "show_student_scroll" => {
            properties.insert("target".into(), cursor_target("Target"));
            properties.insert(
                "direction".into(),
                json!({"enum":["up","down","left","right"],"title":"Direction","type":"string"}),
            );
            vec!["target", "direction", "caption", "expected"]
        }
        _ => {
            properties.insert("target".into(), cursor_target("Target"));
            vec!["target", "caption", "expected"]
        }
    };
    properties.insert(
        "caption".into(),
        json!({"maxLength":400,"minLength":1,"title":"Caption","type":"string"}),
    );
    properties.insert(
        "expected".into(),
        json!({"anyOf":[{"$ref":"#/$defs/Postcondition"},{"type":"null"}]}),
    );
    json!({
        "$defs": {
            "Postcondition": {
                "additionalProperties": false,
                "properties": {
                    "target": {"$ref":"#/$defs/Selector"},
                    "value": {"maxLength":160,"title":"Value","type":"string"}
                },
                "required": ["target","value"],
                "title": "Postcondition",
                "type": "object"
            },
            "Selector": {
                "additionalProperties": false,
                "properties": {
                    "role": {"maxLength":80,"minLength":1,"title":"Role","type":"string"},
                    "label": {"maxLength":160,"minLength":1,"title":"Label","type":"string"}
                },
                "required": ["role","label"],
                "title": "Selector",
                "type": "object"
            },
            "VisualTarget": {
                "additionalProperties": false,
                "description": "Region normalized to the entire selected-window screenshot, never the desktop.",
                "properties": {
                    "description": {"maxLength":160,"minLength":1,"title":"Description","type":"string"},
                    "x": {"maximum":1,"minimum":0,"title":"X","type":"number"},
                    "y": {"maximum":1,"minimum":0,"title":"Y","type":"number"},
                    "width": {"exclusiveMinimum":0,"maximum":1,"title":"Width","type":"number"},
                    "height": {"exclusiveMinimum":0,"maximum":1,"title":"Height","type":"number"}
                },
                "required": ["description","x","y","width","height"],
                "title": "VisualTarget",
                "type": "object"
            }
        },
        "properties": properties,
        "required": required,
        "title": format!("{name}_args"),
        "type": "object",
        "additionalProperties": false
    })
}

fn closed_schema(value: &Value, depth: usize) -> bool {
    if depth > 12 {
        return false;
    }
    match value {
        Value::Array(values) => values.iter().all(|value| closed_schema(value, depth + 1)),
        Value::Object(object) => {
            let allowed = [
                "$defs",
                "$ref",
                "additionalProperties",
                "anyOf",
                "description",
                "enum",
                "exclusiveMinimum",
                "items",
                "maxLength",
                "maximum",
                "minLength",
                "minimum",
                "properties",
                "required",
                "title",
                "type",
            ];
            object.keys().all(|key| allowed.contains(&key.as_str()))
                && object
                    .get("properties")
                    .is_none_or(|_| object.get("additionalProperties") == Some(&Value::Bool(false)))
                && object.iter().all(|(key, value)| match key.as_str() {
                    "$defs" | "properties" => value.as_object().is_some_and(|entries| {
                        entries
                            .values()
                            .all(|entry| closed_schema(entry, depth + 1))
                    }),
                    _ => closed_schema(value, depth + 1),
                })
        }
        _ => true,
    }
}

#[cfg(test)]
pub(crate) fn test_cursor_tools() -> Value {
    Value::Array(
        CURSOR_TOOL_NAMES
            .iter()
            .map(|name| {
                json!({
                    "type":"function",
                    "name":name,
                    "description":"Visual teaching only.",
                    "strict":true,
                    "parameters": cursor_schema(name)
                })
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn accepts_exact_catalog_and_rejects_capability_expansion() {
        let tools = test_cursor_tools();
        assert!(valid_cursor_tools(Some(&tools)));
        for invalid in [
            json!([{"type":"computer"}]),
            json!([{"type":"function","name":"delete_file"}]),
            json!([]),
        ] {
            assert!(!valid_cursor_tools(Some(&invalid)));
        }
        let mut expanded = tools;
        expanded[0]["parameters"]["properties"]["command"] = json!({"type":"string"});
        assert!(!valid_cursor_tools(Some(&expanded)));
    }

    #[test]
    fn rejects_missing_duplicate_and_weakened_cursor_schemas() {
        let tools = test_cursor_tools();
        let mut missing = tools.clone();
        missing.as_array_mut().unwrap().pop();
        assert!(!valid_cursor_tools(Some(&missing)));

        let mut duplicate = tools.clone();
        duplicate[4] = duplicate[0].clone();
        assert!(!valid_cursor_tools(Some(&duplicate)));

        let mut weakened = tools;
        weakened[0]["parameters"]["properties"]["target"] = json!({"type":"string"});
        assert!(!valid_cursor_tools(Some(&weakened)));
    }
}

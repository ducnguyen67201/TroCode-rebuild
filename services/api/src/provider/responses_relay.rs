use super::bounded_body;
use axum::{
    body::Body,
    http::{HeaderValue, StatusCode, header::CONTENT_TYPE},
    response::Response,
};
use reqwest::Client;
use serde_json::{Value, json};
use std::sync::Arc;

pub(crate) const MAX_RESPONSES_REQUEST_BYTES: usize = 2 * 1024 * 1024;
const MAX_RESPONSES_RESPONSE_BYTES: usize = 2 * 1024 * 1024;
const OPENAI_RESPONSES_URL: &str = "https://api.openai.com/v1/responses";

#[derive(Clone)]
pub(crate) struct ResponsesRelay {
    client: Client,
    api_key: Arc<str>,
    model: Arc<str>,
    upstream: Arc<str>,
}

impl ResponsesRelay {
    pub(crate) fn new(client: Client, api_key: Arc<str>, model: Arc<str>) -> Self {
        Self {
            client,
            api_key,
            model,
            upstream: OPENAI_RESPONSES_URL.into(),
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test(
        client: Client,
        api_key: impl Into<Arc<str>>,
        model: impl Into<Arc<str>>,
        upstream: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            client,
            api_key: api_key.into(),
            model: model.into(),
            upstream: upstream.into(),
        }
    }

    pub(crate) fn model(&self) -> &str {
        &self.model
    }

    pub(crate) fn prepare(&self, mut request: Value) -> Result<Value, RelayError> {
        let object = request.as_object_mut().ok_or(RelayError::InvalidEnvelope)?;
        object.insert("model".into(), Value::String(self.model.to_string()));
        object.insert("store".into(), Value::Bool(false));
        object.insert("stream".into(), Value::Bool(false));
        object.insert("background".into(), Value::Bool(false));
        object.insert("max_output_tokens".into(), json!(1024));
        Ok(request)
    }

    pub(crate) async fn send(&self, request: Value) -> Result<RelayedResponse, RelayError> {
        let response = self
            .client
            .post(self.upstream.as_ref())
            .bearer_auth(self.api_key.as_ref())
            .json(&request)
            .send()
            .await
            .map_err(|_| RelayError::Transport)?;
        let status = response.status();
        let content_type = response.headers().get(CONTENT_TYPE).cloned();
        let body = bounded_body(response, MAX_RESPONSES_RESPONSE_BYTES)
            .await
            .map_err(|_| RelayError::ResponseTooLarge)?;
        Ok(RelayedResponse {
            status,
            content_type,
            body,
        })
    }
}

#[cfg(debug_assertions)]
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct RequestDebugShape {
    pub bytes: usize,
    pub input_items: usize,
    pub text_inputs: usize,
    pub image_inputs: usize,
    pub objective_chars: usize,
    pub completed_guidance_items: usize,
    pub accessibility_targets: usize,
    pub accessibility_complete: Option<bool>,
    pub tool_names: Vec<&'static str>,
    pub tool_choice: &'static str,
}

#[cfg(debug_assertions)]
pub(crate) fn request_debug_shape(request: &Value) -> RequestDebugShape {
    let inputs = request["input"].as_array();
    let content = inputs
        .into_iter()
        .flatten()
        .filter_map(|item| item["content"].as_array())
        .flatten();
    let mut text_inputs = 0;
    let mut image_inputs = 0;
    let mut objective_chars = 0;
    let mut completed_guidance_items = 0;
    let mut accessibility_targets = 0;
    let mut accessibility_complete = None;
    for item in content {
        match item["type"].as_str() {
            Some("input_text") => {
                text_inputs += 1;
                if let Some(text) = item["text"].as_str()
                    && let Ok(prompt) = serde_json::from_str::<Value>(text)
                {
                    objective_chars = prompt["objective"]
                        .as_str()
                        .map_or(0, |value| value.chars().count());
                    completed_guidance_items =
                        prompt["completed_guidance"].as_array().map_or(0, Vec::len);
                    accessibility_targets = prompt["unique_accessibility_targets"]
                        .as_array()
                        .map_or(0, Vec::len);
                    accessibility_complete = prompt["accessibility_complete"].as_bool();
                }
            }
            Some("input_image") => image_inputs += 1,
            _ => {}
        }
    }
    let tool_names = request["tools"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|tool| known_cursor_tool(tool["name"].as_str()?))
        .collect();
    RequestDebugShape {
        bytes: serde_json::to_vec(request).map_or(0, |body| body.len()),
        input_items: inputs.map_or(0, Vec::len),
        text_inputs,
        image_inputs,
        objective_chars,
        completed_guidance_items,
        accessibility_targets,
        accessibility_complete,
        tool_names,
        tool_choice: match request["tool_choice"].as_str() {
            Some("required") => "required",
            Some("auto") => "auto",
            Some("none") => "none",
            _ => "other",
        },
    }
}

#[cfg(debug_assertions)]
fn known_cursor_tool(value: &str) -> Option<&'static str> {
    match value {
        "show_student_where" => Some("show_student_where"),
        "show_student_click" => Some("show_student_click"),
        "show_student_drag" => Some("show_student_drag"),
        "show_student_type" => Some("show_student_type"),
        "show_student_scroll" => Some("show_student_scroll"),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RelayError {
    InvalidEnvelope,
    Transport,
    ResponseTooLarge,
}

pub(crate) struct RelayedResponse {
    status: StatusCode,
    content_type: Option<HeaderValue>,
    body: Vec<u8>,
}

impl RelayedResponse {
    pub(crate) fn status(&self) -> StatusCode {
        self.status
    }

    pub(crate) fn body_len(&self) -> usize {
        self.body.len()
    }

    #[cfg(debug_assertions)]
    pub(crate) fn debug_shape(&self) -> ResponseDebugShape {
        let parsed = serde_json::from_slice::<Value>(&self.body).unwrap_or(Value::Null);
        let output = parsed["output"].as_array();
        let mut item_types = Vec::new();
        let mut cursor_tool = None;
        let mut argument_bytes = 0;
        let mut argument_fields = Vec::new();
        let mut target_kind = "none";
        let mut arguments_valid = false;
        let mut caption_chars = 0;
        let mut target_label_chars = 0;
        let mut destination_kind = "none";
        let mut has_expected = false;
        let mut direction = "none";
        for item in output.into_iter().flatten() {
            item_types.push(match item["type"].as_str() {
                Some("reasoning") => "reasoning",
                Some("function_call") => "function_call",
                Some("message") => "message",
                _ => "other",
            });
            if item["type"].as_str() != Some("function_call") {
                continue;
            }
            cursor_tool = item["name"].as_str().and_then(known_cursor_tool);
            let Some(arguments) = item["arguments"].as_str() else {
                continue;
            };
            argument_bytes = arguments.len();
            let Ok(arguments) = serde_json::from_str::<Value>(arguments) else {
                continue;
            };
            if let Some(object) = arguments.as_object() {
                arguments_valid = true;
                argument_fields = [
                    "target",
                    "source",
                    "destination",
                    "caption",
                    "expected",
                    "direction",
                ]
                .into_iter()
                .filter(|field| object.contains_key(*field))
                .collect();
                let target = object.get("target").or_else(|| object.get("source"));
                target_kind = match target {
                    Some(value) if value.get("role").is_some() => "accessibility",
                    Some(value) if value.get("description").is_some() => "visual",
                    Some(_) => "unknown",
                    None => "none",
                };
                caption_chars = object
                    .get("caption")
                    .and_then(Value::as_str)
                    .map_or(0, |value| value.chars().count());
                target_label_chars = target
                    .and_then(|value| value.get("label"))
                    .and_then(Value::as_str)
                    .map_or(0, |value| value.chars().count());
                destination_kind = match object.get("destination") {
                    Some(value) if value.get("role").is_some() => "accessibility",
                    Some(value) if value.get("description").is_some() => "visual",
                    Some(value) if value.is_null() => "none",
                    Some(_) => "unknown",
                    None => "none",
                };
                has_expected = object.get("expected").is_some_and(|value| !value.is_null());
                direction = match object.get("direction").and_then(Value::as_str) {
                    Some("up") => "up",
                    Some("down") => "down",
                    Some("left") => "left",
                    Some("right") => "right",
                    Some(_) => "other",
                    None => "none",
                };
            }
        }
        ResponseDebugShape {
            output_items: output.map_or(0, Vec::len),
            item_types,
            cursor_tool,
            argument_bytes,
            argument_fields,
            target_kind,
            arguments_valid,
            caption_chars,
            target_label_chars,
            destination_kind,
            has_expected,
            direction,
        }
    }

    pub(crate) fn into_response(self) -> Response {
        let mut response = Response::new(Body::from(self.body));
        *response.status_mut() = self.status;
        response.headers_mut().insert(
            CONTENT_TYPE,
            self.content_type
                .unwrap_or_else(|| HeaderValue::from_static("application/json")),
        );
        response
    }
}

#[cfg(debug_assertions)]
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ResponseDebugShape {
    pub output_items: usize,
    pub item_types: Vec<&'static str>,
    pub cursor_tool: Option<&'static str>,
    pub argument_bytes: usize,
    pub argument_fields: Vec<&'static str>,
    pub target_kind: &'static str,
    pub arguments_valid: bool,
    pub caption_chars: usize,
    pub target_label_chars: usize,
    pub destination_kind: &'static str,
    pub has_expected: bool,
    pub direction: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Json, Router, http::HeaderMap, response::IntoResponse, routing::post};
    use http_body_util::BodyExt;
    use serde_json::json;
    use std::time::Duration;

    fn test_client(timeout: Duration) -> Client {
        Client::builder()
            .timeout(timeout)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap()
    }

    async fn serve(app: Router) -> (String, tokio::task::JoinHandle<()>) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        (format!("http://{address}/responses"), server)
    }

    #[test]
    fn policy_overwrites_only_relay_owned_fields() {
        let relay = ResponsesRelay::for_test(
            test_client(Duration::from_secs(1)),
            "secret",
            "guidance-model",
            "http://127.0.0.1/responses",
        );
        let tools = json!([{
            "type":"function",
            "name":"future_tool",
            "parameters":{"type":"object","future":{"nested":true}}
        }]);
        let input = json!([{"role":"user","content":[
            {"type":"input_text","text":"help"},
            {"type":"input_image","image_url":"data:image/png;base64,AAAA"}
        ]}]);
        let prepared = relay
            .prepare(json!({
                "model":"caller-model",
                "input":input,
                "tools":tools,
                "future_sdk_field":{"nested":[1,2,3]},
                "parallel_tool_calls":true,
                "store":true,
                "stream":true,
                "background":true,
                "max_output_tokens":9999
            }))
            .unwrap();

        assert_eq!(prepared["model"], "guidance-model");
        assert_eq!(prepared["store"], false);
        assert_eq!(prepared["stream"], false);
        assert_eq!(prepared["background"], false);
        assert_eq!(prepared["max_output_tokens"], 1024);
        assert_eq!(prepared["input"], input);
        assert_eq!(prepared["tools"], tools);
        assert_eq!(prepared["future_sdk_field"], json!({"nested":[1,2,3]}));
        assert_eq!(prepared["parallel_tool_calls"], true);
        assert_eq!(
            relay.prepare(json!(["not", "an", "object"])),
            Err(RelayError::InvalidEnvelope)
        );
    }

    #[test]
    fn debug_shapes_report_structure_without_private_content() {
        let request = json!({
            "input":[{"role":"user","content":[
                {"type":"input_text","text":json!({
                    "objective":"private instruction",
                    "completed_guidance":["private prior step"],
                    "accessibility_complete":true,
                    "unique_accessibility_targets":[{"role":"button","label":"private label"}]
                }).to_string()},
                {"type":"input_image","image_url":"data:image/png;base64,private-image"}
            ]}],
            "tools":[{"type":"function","name":"show_student_click"}],
            "tool_choice":"required"
        });
        let request_shape = request_debug_shape(&request);
        assert_eq!(request_shape.input_items, 1);
        assert_eq!(request_shape.text_inputs, 1);
        assert_eq!(request_shape.image_inputs, 1);
        assert_eq!(request_shape.objective_chars, "private instruction".len());
        assert_eq!(request_shape.completed_guidance_items, 1);
        assert_eq!(request_shape.accessibility_targets, 1);
        assert_eq!(request_shape.accessibility_complete, Some(true));
        assert_eq!(request_shape.tool_names, vec!["show_student_click"]);
        assert_eq!(request_shape.tool_choice, "required");
        assert!(!format!("{request_shape:?}").contains("private"));

        let arguments = json!({
            "target":{"role":"button","label":"private label"},
            "caption":"private caption",
            "expected":null
        })
        .to_string();
        let response = RelayedResponse {
            status: StatusCode::OK,
            content_type: None,
            body: serde_json::to_vec(&json!({"output":[
                {"type":"reasoning","encrypted_content":"private reasoning"},
                {"type":"function_call","name":"show_student_click","arguments":arguments}
            ]}))
            .unwrap(),
        };
        let response_shape = response.debug_shape();
        assert_eq!(response_shape.output_items, 2);
        assert_eq!(
            response_shape.item_types,
            vec!["reasoning", "function_call"]
        );
        assert_eq!(response_shape.cursor_tool, Some("show_student_click"));
        assert_eq!(
            response_shape.argument_fields,
            vec!["target", "caption", "expected"]
        );
        assert_eq!(response_shape.target_kind, "accessibility");
        assert!(response_shape.arguments_valid);
        assert_eq!(response_shape.caption_chars, "private caption".len());
        assert_eq!(response_shape.target_label_chars, "private label".len());
        assert_eq!(response_shape.destination_kind, "none");
        assert!(!response_shape.has_expected);
        assert_eq!(response_shape.direction, "none");
        assert!(!format!("{response_shape:?}").contains("private"));
    }

    #[tokio::test]
    async fn forwards_backend_key_and_preserves_upstream_error() {
        let app = Router::new().route(
            "/responses",
            post(|headers: HeaderMap, Json(body): Json<Value>| async move {
                assert_eq!(headers["authorization"], "Bearer backend-secret");
                assert_eq!(body["future_sdk_field"], "preserved");
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error":{"message":"provider rejected request"}})),
                )
                    .into_response()
            }),
        );
        let (upstream, server) = serve(app).await;
        let relay = ResponsesRelay::for_test(
            test_client(Duration::from_secs(1)),
            "backend-secret",
            "guidance-model",
            upstream,
        );

        let response = relay
            .send(
                relay
                    .prepare(json!({"input":"help","future_sdk_field":"preserved"}))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            serde_json::from_slice::<Value>(&response.body).unwrap(),
            json!({"error":{"message":"provider rejected request"}})
        );
        assert!(!String::from_utf8_lossy(&response.body).contains("backend-secret"));
        server.abort();
    }

    #[tokio::test]
    async fn bounds_provider_timeout_and_response_size() {
        let app = Router::new()
            .route(
                "/slow",
                post(|| async {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    Json(json!({"ok":true}))
                }),
            )
            .route(
                "/large",
                post(|| async { "x".repeat(MAX_RESPONSES_RESPONSE_BYTES + 1) }),
            );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        let slow = ResponsesRelay::for_test(
            test_client(Duration::from_millis(25)),
            "secret",
            "model",
            format!("http://{address}/slow"),
        );
        assert_eq!(
            slow.send(slow.prepare(json!({})).unwrap()).await.err(),
            Some(RelayError::Transport)
        );

        let large = ResponsesRelay::for_test(
            test_client(Duration::from_secs(1)),
            "secret",
            "model",
            format!("http://{address}/large"),
        );
        assert_eq!(
            large.send(large.prepare(json!({})).unwrap()).await.err(),
            Some(RelayError::ResponseTooLarge)
        );
        server.abort();
    }

    #[tokio::test]
    async fn relayed_response_rebuilds_only_safe_transport_fields() {
        let response = RelayedResponse {
            status: StatusCode::TOO_MANY_REQUESTS,
            content_type: Some(HeaderValue::from_static("application/problem+json")),
            body: br#"{"error":"limited"}"#.to_vec(),
        }
        .into_response();
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(response.headers()[CONTENT_TYPE], "application/problem+json");
        let body = response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes()
            .to_vec();
        assert_eq!(body, br#"{"error":"limited"}"#);
    }
}

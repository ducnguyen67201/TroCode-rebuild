#[test]
fn shared_corpus() {
    let cases: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/contracts/corpus.json"
    ))
    .unwrap();
    for case in cases.as_array().unwrap() {
        let result = tro_contracts::parse_message(&serde_json::to_vec(&case["value"]).unwrap());
        assert_eq!(
            result.is_ok(),
            case["valid"].as_bool().unwrap(),
            "{}",
            case["name"]
        );
    }
}
#[test]
fn bounds() {
    assert!(tro_contracts::parse_message(&[]).is_err());
    assert!(tro_contracts::parse_message(&vec![b' '; tro_contracts::MAX_FRAME_BYTES + 1]).is_err());
}

#[test]
fn voice_projection_is_closed() {
    let value = serde_json::json!({
        "phase":"idle","revision":1,"utteranceId":null,"runId":null,
        "partialTranscript":"","finalTranscript":"","queuedInstructions":[],"targetTitle":null,
        "message":"Ready.","shortcut":"Command+Control",
        "permissions":{"microphone":"granted","keyboardMonitoring":"granted","ready":true,"recovery":""},
        "confirmation":null,"actionsUsed":0
    });
    assert!(tro_contracts::parse_voice_status(&value).is_ok());
    let mut private = value;
    private["audio"] = serde_json::json!("bytes");
    assert!(tro_contracts::parse_voice_status(&private).is_err());
}

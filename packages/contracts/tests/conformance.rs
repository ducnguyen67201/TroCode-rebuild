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
        "phase":"idle","revision":1,"utteranceId":null,"guidanceId":null,
        "partialTranscript":"","finalTranscript":"","targetTitle":null,
        "message":"Ready.","shortcut":"Command+Control",
        "transcriptionLanguage":"vi",
        "permissions":{"microphone":"granted","keyboardMonitoring":"granted","ready":true,"recovery":""}
    });
    assert!(tro_contracts::parse_voice_status(&value).is_ok());
    let mut missing_language = value.clone();
    missing_language
        .as_object_mut()
        .unwrap()
        .remove("transcriptionLanguage");
    assert!(tro_contracts::parse_voice_status(&missing_language).is_err());
    let mut invalid_language = value.clone();
    invalid_language["transcriptionLanguage"] = serde_json::json!("fr");
    assert!(tro_contracts::parse_voice_status(&invalid_language).is_err());
    let mut private = value;
    private["audio"] = serde_json::json!("bytes");
    assert!(tro_contracts::parse_voice_status(&private).is_err());
    private.as_object_mut().unwrap().remove("audio");
    private["confirmation"] = serde_json::json!({"id":"old"});
    assert!(tro_contracts::parse_voice_status(&private).is_err());
    private.as_object_mut().unwrap().remove("confirmation");
    private["actionsUsed"] = serde_json::json!(1);
    assert!(tro_contracts::parse_voice_status(&private).is_err());
}

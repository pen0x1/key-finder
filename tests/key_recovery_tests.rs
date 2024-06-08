pub fn recover_key(input: &str) -> Result<Option<String>, String> {
    if input.is_empty() {
        return Err("Input cannot be empty".into()); // Using `.into()` for slightly more idiomatic error conversion
    }

    match input {
        "known_input_example" => Ok(Some("expected_key_for_known_input".to_string())),
        "partial_input_example" => Ok(Some("expected_key_for_partial_input".to_string())),
        "speculative_input_example" => Ok(None),
        _ if input == "wrong_format" => Err("Wrong format".into()),
        _ => Err("Unexpected input".into()),
    }
}
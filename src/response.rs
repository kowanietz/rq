/// Converts a JSON string into formatted JSON
pub fn format_body(body: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(body) {
        Ok(json) => serde_json::to_string_pretty(&json).unwrap(),
        Err(_) => body.to_string(),
    }
}

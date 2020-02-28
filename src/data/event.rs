#[derive(Debug, Clone)]
pub struct Event {
    pub content_type: String,
    pub content: String,
    pub metadata: serde_json::Value,
}

impl Event {
    pub fn text(text: &str) -> Self {
        Self {
            content_type: "text".to_owned(),
            content: text.to_owned(),
            metadata: serde_json::json!({}),
        }
    }
}

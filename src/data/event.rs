#[derive(Debug, Clone)]
pub struct Event {
    pub content_type: String,
    pub content: String,
    pub metadata: serde_json::Value,
}

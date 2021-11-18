////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Event {
    pub content_type: String,
    pub content_value: String,
    pub content: serde_json::Value,
    pub ttl_duration: Option<i64>,
    pub low_data_mode: Option<bool>,
}

////////////////////////////////////////////////////////////////////////////////
// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Default for Event {
    fn default() -> Self {
        Self {
            content_type: String::default(),
            content_value: String::default(),
            content: serde_json::json!({}),
            ttl_duration: None,
            low_data_mode: None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Event {
    pub fn new(content_type: &str, content_value: &str, content: serde_json::Value) -> Self {
        Self {
            content_type: content_type.to_owned(),
            content_value: content_value.to_owned(),
            content,
            ttl_duration: None,
            low_data_mode: None,
        }
    }
}

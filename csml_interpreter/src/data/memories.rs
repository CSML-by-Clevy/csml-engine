use crate::data::Literal;

#[derive(Debug, Clone)]
pub enum MemoryType {
    Event(String),
    Metadata,
    Use,
    Remember,
}

#[derive(Debug, Clone)]
pub struct Memories {
    pub key: String,
    pub value: serde_json::Value,
}

impl Memories {
    pub fn new(key: String, value: Literal) -> Self {
        let content_type = &value.content_type;
        let value = value.primitive.format_mem(content_type, true);

        Self { key, value }
    }
}

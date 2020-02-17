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
        Self {
            key,
            value: value.primitive.to_json(),
        }
    }
}

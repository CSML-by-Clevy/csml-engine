use crate::data::Literal;
use crate::data::primitive::{PrimitiveObject};

#[derive(Debug, Clone)]
pub enum MemoryType {
    Event(String),
    Metadata,
    Use,
    Remember,
}

#[derive(Debug, Clone)]
pub struct Memory {
    pub key: String,
    pub value: serde_json::Value,
}

impl Memory {
    pub fn new(key: String, value: Literal) -> Self {
        let content_type = &value.content_type;

        let value = if let Some(obj) = value.additional_info {
            serde_json::json!({
                "_additional_info": PrimitiveObject::obj_literal_to_json(&obj),
                "value": value.primitive.format_mem(content_type, true)
            })
        } else {
            value.primitive.format_mem(content_type, true)
        };

        Self { key, value }
    }
}

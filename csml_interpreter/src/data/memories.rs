use crate::data::primitive::PrimitiveObject;
use crate::data::Literal;

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryType {
    Event(String),
    Metadata,
    Use,
    Remember,
    Constant,
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

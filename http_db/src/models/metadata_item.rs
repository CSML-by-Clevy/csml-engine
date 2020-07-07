/*
 * CSML engine microservices
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataItem {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: serde_json::Value,
}

impl MetadataItem {
    pub fn new(key: String, value: serde_json::Value) -> MetadataItem {
        MetadataItem {
            key: key,
            value: value,
        }
    }
}



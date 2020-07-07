/*
 * CSML engine microservices
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateNodeBody {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "interaction_id")]
    pub interaction_id: String,
    #[serde(rename = "flow_id")]
    pub flow_id: String,
    #[serde(rename = "next_flow", skip_serializing_if = "Option::is_none")]
    pub next_flow: Option<String>,
    #[serde(rename = "step_id")]
    pub step_id: String,
    #[serde(rename = "next_step", skip_serializing_if = "Option::is_none")]
    pub next_step: Option<String>,
}

impl CreateNodeBody {
    pub fn new(
        id: String,
        interaction_id: String,
        flow_id: String,
        step_id: String,
    ) -> CreateNodeBody {
        CreateNodeBody {
            id,
            interaction_id,
            flow_id,
            next_flow: None,
            step_id,
            next_step: None,
        }
    }
}

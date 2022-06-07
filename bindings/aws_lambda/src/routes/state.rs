use csml_engine::Client;

use crate::{format_response, Error};

pub fn get_client_current_state(client: Client) -> Result<serde_json::Value, Error> {
    let res = csml_engine::get_current_state(&client);

    match res {
        Ok(Some(state)) => Ok(serde_json::json!(
            {
                "isBase64Encoded": false,
                "statusCode": 200,
                "headers": { "Content-Type": "application/json" },
                "body": state
            }
        )),
        Ok(None) => Ok(serde_json::json!(
            {
                "isBase64Encoded": false,
                "statusCode": 200,
                "headers": { "Content-Type": "application/json" },
            }
        )),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            return Ok(format_response(400, serde_json::json!(error)));
        }
    }
}

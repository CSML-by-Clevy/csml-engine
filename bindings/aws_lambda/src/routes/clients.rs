use csml_engine::Client;

use crate::{format_response, Error};

pub fn delete_client_data(body: Client) -> Result<serde_json::Value, Error> {
    let res = csml_engine::delete_client(&body);

    match res {
        Ok(_) => Ok(serde_json::json!(
            {
                "statusCode": 204,
            }
        )),
        Err(err) => {
            let error = format!("EngineError: {:?}", err);
            return Ok(format_response(400, serde_json::json!(error)));
        }
    }
}

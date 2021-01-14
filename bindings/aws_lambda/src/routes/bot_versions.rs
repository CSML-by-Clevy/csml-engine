use csml_interpreter::data::csml_bot::CsmlBot;
use csml_engine::{create_bot_version, get_bot_versions, get_last_bot_version, get_bot_by_version_id };

use crate::{format_response, routes::{GetVersionsRequest, GetByIdRequest}};

use lambda_runtime::error::HandlerError;

pub fn add_bot_version(bot: CsmlBot) -> Result<serde_json::Value, HandlerError> {

  let res = create_bot_version(bot);

  match res {
    Ok(data) => {
        Ok(serde_json::json!(
        {
          "isBase64Encoded": false,
          "statusCode": 201,
          "headers": { "Content-Type": "application/json" },
          "body": data,
        }
      ))
    },
    Err(err) => {
        let error = format!("EngineError: {:?}", err);
        return Ok(format_response(400, serde_json::json!(error)))
    }
  }
}

pub fn get_bot_version(path: GetByIdRequest) -> Result<serde_json::Value, HandlerError> {

  let res = get_bot_by_version_id(&path.version_id, &path.bot_id);

  match res {
    Ok(data) => {
      match data {
        Some(data) => {
          Ok(serde_json::json!(
            {
              "isBase64Encoded": false,
              "statusCode": 200,
              "headers": { "Content-Type": "application/json" },
              "body": data.flatten()
            }
          ))
        }
        None => {
          Ok(serde_json::json!({
            "isBase64Encoded": false,
            "statusCode": 400,
            "headers": { "Content-Type": "application/json" },
            "body": "Not found"
          }))
        }
      }
    },
    Err(err) => {
        let error = format!("EngineError: {:?}", err);
        return Ok(format_response(400, serde_json::json!(error)))
    }
  }
}

pub fn get_bot_latest_versions(body: GetVersionsRequest) -> Result<serde_json::Value, HandlerError> {

  let res = get_bot_versions(&body.bot_id, body.limit, body.pagination_key);

  match res {
    Ok(data) => Ok(serde_json::json!(
      {
        "isBase64Encoded": false,
        "statusCode": 200,
        "headers": { "Content-Type": "application/json" },
        "body": data
      }
    )),
    Err(err) => {
        let error = format!("EngineError: {:?}", err);
        return Ok(format_response(400, serde_json::json!(error)))
    }
  }
}

pub fn get_bot_latest_version(bot_id: String) -> Result<serde_json::Value, HandlerError> {

  let res = get_last_bot_version(&bot_id);

  match res {
    Ok(data) => {
      match data {
        Some(data) => {
          Ok(serde_json::json!(
            {
              "isBase64Encoded": false,
              "statusCode": 200,
              "headers": { "Content-Type": "application/json" },
              "body": data.flatten()
            }
          ))
        }
        None => {
          Ok(serde_json::json!({
            "isBase64Encoded": false,
            "statusCode": 400,
            "headers": { "Content-Type": "application/json" },
            "body": "Not found"
          }))
        }
      }
        
    },
    Err(err) => {
        let error = format!("EngineError: {:?}", err);
        return Ok(format_response(400, serde_json::json!(error)))
    }
  }
}
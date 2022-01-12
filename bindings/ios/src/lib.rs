use csml_engine::{
    data::{RunRequest}, start_conversation, user_close_all_conversations,
    Client, CsmlResult
};
use csml_interpreter::data::csml_bot::CsmlBot;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[derive(Debug, serde::Deserialize)]
pub struct LimitPaginationQueryParams {
  limit: Option<i64>,
  pagination_key: Option<String>,
}

unsafe fn get_string(s: *mut c_char) -> String {
    if s.is_null() {
        return "".to_owned()
    }

    CString::from_raw(s)
        .into_string()
        .expect("into_string() call failed")
}


#[no_mangle]
pub unsafe extern "C" fn hello_csml(to: *const c_char) -> *mut c_char {
    let c_str = CStr::from_ptr(to);
    let recipient = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => "you",
    };

    CString::new(format!("Hello from CSML: {}", recipient))
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn release_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    CString::from_raw(s);
}

// #############################################################################

#[no_mangle]
pub unsafe extern "C" fn get_open_conversation(string: *mut c_char) -> *mut c_char {
    let raw_client = get_string(string);

    let client: Client = match serde_json::from_str(&raw_client) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    match csml_engine::get_open_conversation(&client) {
        Ok(Some(conversation)) => {
            let string = serde_json::json!(conversation).to_string();

            CString::new(string)
                .expect("Couldn't create string!")
                .into_raw()
        }
        Ok(None) => {
            CString::new(format!(""))
                .expect("Couldn't create string!")
                .into_raw()
        }
        Err(err) => {
            CString::new(format!("{:?}", err))
                .expect("Couldn't create string!")
                .into_raw()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_client_current_state(string: *mut c_char) -> *mut c_char {
    let raw_client = get_string(string);

    let client: Client = match serde_json::from_str(&raw_client) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    match csml_engine::get_current_state(&client) {
        Ok(Some(conversation)) => {
            let string = serde_json::json!(conversation).to_string();

            CString::new(string)
                .expect("Couldn't create string!")
                .into_raw()
        }
        Ok(None) => {
            CString::new(format!(""))
                .expect("Couldn't create string!")
                .into_raw()
        }
        Err(err) => {
            CString::new(format!("{:?}", err))
                .expect("Couldn't create string!")
                .into_raw()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn CreateClientMemory(
    client: *mut c_char,
    key: *mut c_char,
    value: *mut c_char
) -> *mut c_char {
    let raw_client = get_string(client);

    let client: Client = match serde_json::from_str(&raw_client) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    let key = get_string(key);

    let string = get_string(value);
    let value: serde_json::Value = serde_json::from_str(&string).expect("");

    match csml_engine::create_client_memory(&client, key, value) {
        Ok(_) => {
            CString::new(format!(""))
            .expect("Couldn't create string!")
            .into_raw()
        },
        Err(err) => {
            CString::new(format!("{:?}", err))
                .expect("Couldn't create string!")
                .into_raw()
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn GetBotSteps(json: *mut c_char) -> *mut c_char {

    let string = get_string(json);

    let jsonbot: serde_json::Value = match serde_json::from_str(&string) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    let map = csml_engine::get_steps_from_flow(serde_json::from_value(jsonbot).unwrap());
    let obj = serde_json::json!(map);

    CString::new(obj.to_string())
        .expect("Couldn't create string!")
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn ValidateBot(
    json: *mut c_char,
) -> *mut c_char {
    let string = get_string(json);

    let jsonbot: serde_json::Value = match serde_json::from_str(&string) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    let mut map = serde_json::Map::new();

    match csml_engine::validate_bot(serde_json::from_value(jsonbot).unwrap()) {
        CsmlResult {
            flows: _,
            warnings,
            errors: None,
        } => {
            map.insert(
                "valid".to_owned(),
                serde_json::json!(true)
            );

            if let Some(warnings) = warnings {
                map.insert(
                    "warnings".to_owned(),
                    serde_json::json!(warnings)
                );
            }
        }
        CsmlResult {
            flows: _,
            warnings,
            errors: Some(errors),
        } => {
            map.insert(
                "valid".to_owned(),
                serde_json::json!(false)
            );

            if let Some(warnings) = warnings {
                map.insert(
                    "warnings".to_owned(),
                    serde_json::json!(warnings)
                );
            }

            map.insert(
                "errors".to_owned(),
                serde_json::json!(errors)
            );
        }
    };

    let obj = serde_json::json!(map);
    CString::new(obj.to_string())
        .expect("Couldn't create string!")
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn RunBot(
    json: *mut c_char,
) -> *mut c_char {
    let string = get_string(json);

    let run_request: RunRequest = match serde_json::from_str(&string) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    let bot_opt = match run_request.get_bot_opt() {
        Ok(bot_opt) => bot_opt,
        Err(err) => {
            return CString::new(format!("{:?}", err))
                .expect("Couldn't create string!")
                .into_raw()
        },
    };
    let request = run_request.event;

    let obj = match start_conversation(request, bot_opt) {
        Ok(map) => serde_json::json!(map),
        Err(err) => {
            return CString::new(format!("{:?}", err))
            .expect("Couldn't create string!")
            .into_raw()
        },
    };

    CString::new(obj.to_string())
        .expect("Couldn't create string!")
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn CloseConversations(
    json: *mut c_char,
) -> *mut c_char {
    let string = get_string(json);

    let client: Client = match serde_json::from_str(&string) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    match user_close_all_conversations(client) {
        Ok(_) => {
            return CString::new(format!("true"))
            .expect("Couldn't create string!")
            .into_raw()
        },
        Err(err) => {
            return CString::new(format!("{:?}", err))
            .expect("Couldn't create string!")
            .into_raw()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn CreateBotVersion(
    json: *mut c_char,
) -> *mut c_char {
    let string = get_string(json);

    let bot: CsmlBot = match serde_json::from_str(&string) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    match csml_engine::create_bot_version(bot) {
        Ok(version_data) => {
            let value = serde_json::json!(version_data);

            return CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
        Err(err) => {
            return CString::new(format!("{:?}", err))
            .expect("Couldn't create string!")
            .into_raw()
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn GetBotByVersionId(
    j_bot_id: *mut c_char,
    j_version_id: *mut c_char,
) -> *mut c_char {

    let bot_id = get_string(j_bot_id);

    let version_id = get_string(j_version_id);

    match csml_engine::get_bot_by_version_id(&version_id, &bot_id) {
        Ok(bot) => {
            let value = match bot {
                Some(bot) => {
                    serde_json::json!(
                        bot.flatten()
                    )
                }
                None => {
                    serde_json::json!({
                        "error": "Not found"
                    })
                }
            };

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        }
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()        
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn GetLastBotversion(
    j_bot_id: *mut c_char,
) -> *mut c_char {

    let bot_id = get_string(j_bot_id);

    match csml_engine::get_last_bot_version(&bot_id) {
        Ok(bot) => {
            let value = match bot {
                Some(bot) => {
                    serde_json::json!(
                        bot.flatten()
                    )
                }
                None => {
                    serde_json::json!({
                        "error": "Not found"
                    })
                }
            };

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()

        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn DeleteBotVersion(
    j_bot_id: *mut c_char,
    j_version_id: *mut c_char,
) -> *mut c_char {

    let bot_id = get_string(j_bot_id);

    let version_id = get_string(j_version_id);

    match csml_engine::delete_bot_version_id(&version_id, &bot_id) {
        Ok(value) => {
            let value = serde_json::json!(
                value
            );

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn DeleteBotVersions(
    j_bot_id: *mut c_char,
) -> *mut c_char {

    let bot_id = get_string(j_bot_id);

    match csml_engine::delete_all_bot_versions(&bot_id) {
        Ok(value) => {
            let value= serde_json::json!(
                value
            );

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn FoldBot(
    j_bot: *mut c_char,
) -> *mut c_char {

    let string= get_string(j_bot);

    let bot: CsmlBot = match serde_json::from_str(&string) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    match csml_engine::fold_bot(bot) {
        Ok(flow) => {
            let value = serde_json::json!({"flow": flow});

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn DeleteClientMemory(
    j_client: *mut c_char,
    j_memory_key: *mut c_char,
) -> *mut c_char {
    let string= get_string(j_client);

    let client: Client = match serde_json::from_str(&string) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    let key = get_string(j_memory_key);

    match csml_engine::delete_client_memory(&client, &key) {
        Ok(value) => {
            let value= serde_json::json!(
                value
            );

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn DeleteClientMemories(
    j_client: *mut c_char,
) -> *mut c_char {

    let string = get_string(j_client);

    let client: Client = match serde_json::from_str(&string) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    match csml_engine::delete_client_memories(&client) {
        Ok(value) => {
            let value= serde_json::json!(
                value
            );

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn DeleteClientsData(
    j_client: *mut c_char,
) -> *mut c_char {
    let string = get_string(j_client);

    let client: Client = match serde_json::from_str(&string) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    match csml_engine::delete_client(&client) {
        Ok(value) => {
            let value = serde_json::json!(
                value
            );

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn DeleteBotData(
    j_bot_id: *mut c_char,
) -> *mut c_char {
    let bot_id = get_string(j_bot_id);

    match csml_engine::delete_all_bot_data(&bot_id) {
        Ok(value) => {
            let value = serde_json::json!(
                value
            );

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn DeleteExpiredData(
) -> *mut c_char {
     match csml_engine::delete_expired_data() {
        Ok(value) => {
            let value = serde_json::json!(
                value
            );

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
    }
}


#[no_mangle]
pub unsafe extern "C" fn GetClientMemories(
    j_client: *mut c_char,
) -> *mut c_char {
    let string = get_string(j_client);

    let client: Client = match serde_json::from_str(&string) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    match csml_engine::get_client_memories(&client) {
        Ok(value) => {
            let value= serde_json::json!(
                value
            );

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()        
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            CString::new(value.to_string())
            .expect("Couldn't create string!")
            .into_raw()
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn GetClientMemory(
    j_client: *mut c_char,
    j_memory_key: *mut c_char,
) -> *mut c_char {
    let string = get_string(j_client);

    let client: Client = match serde_json::from_str(&string) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    let memory_key = get_string(j_memory_key);

    match csml_engine::get_client_memory(&client, &memory_key) {
        Ok(value) => {
            let value= serde_json::json!(
                value
            );

            return CString::new(value.to_string())
                .expect("Couldn't create string!")
                .into_raw()
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            return CString::new(value.to_string())
                .expect("Couldn't create string!")
                .into_raw()
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn GetClientConversations(
    j_client: *mut c_char,
    j_limit: *mut c_char,
) -> *mut c_char {
    let string = get_string(j_client);

    let client: Client = match serde_json::from_str(&string) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    let string = get_string(j_limit);

    let params: LimitPaginationQueryParams = match serde_json::from_str(&string) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    match csml_engine::get_client_conversations(&client, params.limit, params.pagination_key) {
        Ok(value) => {
            let value= serde_json::json!(
                value
            );

            return CString::new(value.to_string())
                .expect("Couldn't create string!")
                .into_raw()
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            return CString::new(value.to_string())
                .expect("Couldn't create string!")
                .into_raw()
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn GetClientMessages(
    j_client: *mut c_char,
    j_limit: *mut c_char,
) -> *mut c_char {

    let string = get_string(j_client);

    let client: Client = match serde_json::from_str(&string) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    let string = get_string(j_limit);

    let params: LimitPaginationQueryParams = match serde_json::from_str(&string) {
        Ok(body) => body,
        Err(_err) => {
            return CString::new(format!("client bad format"))
                .expect("Couldn't create string!")
                .into_raw()
        }
    };

    match csml_engine::get_client_messages(&client, params.limit, params.pagination_key) {
        Ok(value) => {
            let value= serde_json::json!(
                value
            );

            return CString::new(value.to_string())
                .expect("Couldn't create string!")
                .into_raw()
        },
        Err(err) => {
            let value = serde_json::json!({
                "error": format!("{:?}", err),
            });

            return CString::new(value.to_string())
                .expect("Couldn't create string!")
                .into_raw()
        },
    }
}


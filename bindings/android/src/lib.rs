use csml_engine::{
    data::{RunRequest}, start_conversation, user_close_all_conversations,
    Client, CsmlResult, ErrorInfo, Warnings
};
use csml_interpreter::data::csml_bot::CsmlBot;

use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

#[derive(Debug, serde::Deserialize)]
pub struct LimitPaginationQueryParams {
  limit: Option<i64>,
  pagination_key: Option<String>,
}

#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString, JObject};
    use self::jni::sys::{jstring};

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_GetOpenConversation(
        env: JNIEnv,
        _: JClass,
        json: JString,
    ) -> jstring {

        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(json).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");

        let client: Client = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };

        let output = match csml_engine::get_open_conversation(&client) {
            Ok(Some(conversation)) => {
                let string = serde_json::json!(conversation).to_string();
                env.new_string(
                    string.as_str()
                ).expect("Couldn't create java string!")
            }
            Ok(None) => {
                env.new_string("").expect("Couldn't create java string!")
            }
            Err(err) => {

                let string = format!("{:?}", err);

                env.new_string(string).expect("Couldn't create java string!")
            }
        };

        output.into_inner()
    }


    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_GetClientCurrentState(
        env: JNIEnv,
        _: JClass,
        json: JString
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(json).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");

        let client: Client = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };

        let output = match csml_engine::get_current_state(&client) {
            Ok(Some(value)) => {
                env.new_string(
                    value.as_str().expect("Couldn't create java string!")
                ).expect("Couldn't create java string!")
            },
            Ok(None) => {
                env.new_string("").expect("Couldn't create java string!")
            },
            Err(err) => {
                let string = format!("{:?}", err);
                env.new_string(string).expect("Couldn't create java string!")
            },
        };

        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_CreateClientMemory(
        env: JNIEnv,
        _: JClass,
        json: JString,
        key: JString,
        value: JString
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(json).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");

        let client: Client = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };

        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(key).expect("invalid pattern string").as_ptr()
            )
        );
        let key = c_string.into_string().expect("into_string() call failed");


        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(value).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");
        let value: serde_json::Value = serde_json::from_str(&string).expect("");

        let output = match csml_engine::create_client_memory(&client, key, value) {
            Ok(_) => {
                env.new_string("").expect("Couldn't create java string!")
            },
            Err(err) => {
                let string = format!("{:?}", err);
                env.new_string(string).expect("Couldn't create java string!")
            },
        };

        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_GetBotSteps(
        env: JNIEnv,
        _: JClass,
        json: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(json).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");

        let jsonbot: serde_json::Value = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };

        let map = csml_engine::get_steps_from_flow(serde_json::from_value(jsonbot).unwrap());
        let obj = serde_json::json!(map);

        let output = env.new_string(
            obj.to_string()
        ).expect("Couldn't create java string!");

        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_ValidateBot(
        env: JNIEnv,
        _: JClass,
        json: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(json).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");

        let jsonbot: serde_json::Value = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
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

        let output = env.new_string(
            obj.to_string()
        ).expect("Couldn't create java string!");

        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_RunBot(
        env: JNIEnv,
        _: JClass,
        json: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(json).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");
        let run_request: RunRequest = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };

        let bot_opt = match run_request.get_bot_opt() {
            Ok(bot_opt) => bot_opt,
            Err(err) => {
                let output = env.new_string(format!("{:?}", err)).expect("Couldn't create java string!");
                return output.into_inner();
            },
        };
        let request = run_request.event;

        let obj = match start_conversation(request, bot_opt) {
            Ok(map) => serde_json::json!(map),
            Err(err) => {
                let output = env.new_string(format!("{:?}", err)).expect("Couldn't create java string!");
                return output.into_inner();
            },
        };

        let output = env.new_string(
            obj.to_string()
        ).expect("Couldn't create java string!");

        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_CloseConversations(
        env: JNIEnv,
        _: JClass,
        json: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(json).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");

        let client: Client = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };

        match user_close_all_conversations(client) {
            Ok(_) => {
                let output = env.new_string(
                    "true"
                ).expect("Couldn't create java string!");

                output.into_inner()
            },
            Err(err) => {
                let output = env.new_string(format!("{:?}", err)).expect("Couldn't create java string!");

                output.into_inner()
            }
        }
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_CreateBotVersion(
        env: JNIEnv,
        _: JClass,
        json: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(json).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");

        let bot: CsmlBot = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };

        let output = match csml_engine::create_bot_version(bot) {
            Ok(version_data) => {
                let value = serde_json::json!(version_data);

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
            Err(err) => {
                let value = serde_json::json!({
                    "error": format!("{:?}", err),
                });

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
        };

        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_GetBotByVersionId(
        env: JNIEnv,
        _: JClass,
        j_bot_id: JString,
        j_version_id: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_bot_id).expect("invalid pattern string").as_ptr()
            )
        );
        let bot_id = c_string.into_string().expect("into_string() call failed");

        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_version_id).expect("invalid pattern string").as_ptr()
            )
        );
        let version_id = c_string.into_string().expect("into_string() call failed");

        let output = match csml_engine::get_bot_by_version_id(&version_id, &bot_id) {
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

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            }
            Err(err) => {
                let value = serde_json::json!({
                    "error": format!("{:?}", err),
                });

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
        };

        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_GetLastBotversion(
        env: JNIEnv,
        _: JClass,
        j_bot_id: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_bot_id).expect("invalid pattern string").as_ptr()
            )
        );
        let bot_id = c_string.into_string().expect("into_string() call failed");

        let output = match csml_engine::get_last_bot_version(&bot_id) {
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

                env.new_string(value.to_string()).expect("Couldn't create java string!")

            },
            Err(err) => {
                let value = serde_json::json!({
                    "error": format!("{:?}", err),
                });

                env.new_string(value.to_string()).expect("Couldn't create java string!")

            },
        };

        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_DeleteBotVersion(
        env: JNIEnv,
        _: JClass,
        j_bot_id: JString,
        j_version_id: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_bot_id).expect("invalid pattern string").as_ptr()
            )
        );
        let bot_id = c_string.into_string().expect("into_string() call failed");

        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_version_id).expect("invalid pattern string").as_ptr()
            )
        );
        let version_id = c_string.into_string().expect("into_string() call failed");

        let output = match csml_engine::delete_bot_version_id(&version_id, &bot_id) {
            Ok(value) => {
                let value= serde_json::json!(
                    value
                );

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
            Err(err) => {
                let value = serde_json::json!({
                    "error": format!("{:?}", err),
                });

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
        };

        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_DeleteBotVersions(
        env: JNIEnv,
        _: JClass,
        j_bot_id: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_bot_id).expect("invalid pattern string").as_ptr()
            )
        );
        let bot_id = c_string.into_string().expect("into_string() call failed");

        let output = match csml_engine::delete_all_bot_versions(&bot_id) {
            Ok(value) => {
                let value= serde_json::json!(
                    value
                );

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
            Err(err) => {
                let value = serde_json::json!({
                    "error": format!("{:?}", err),
                });

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
        };

        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_FoldBot(
        env: JNIEnv,
        _: JClass,
        j_bot: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_bot).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");

        let bot: CsmlBot = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };

        let output = match csml_engine::fold_bot(bot) {
            Ok(flow) => {
                let value = serde_json::json!({"flow": flow});

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
            Err(err) => {
                let value = serde_json::json!({
                    "error": format!("{:?}", err),
                });

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
        };

        output.into_inner()
    }


    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_DeleteClientMemory(
        env: JNIEnv,
        _: JClass,
        j_client: JString,
        j_memory_key: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_client).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");

        let client: Client = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };

        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_memory_key).expect("invalid pattern string").as_ptr()
            )
        );
        let key = c_string.into_string().expect("into_string() call failed");

        let output = match csml_engine::delete_client_memory(&client, &key) {
            Ok(value) => {
                let value= serde_json::json!(
                    value
                );

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
            Err(err) => {
                let value = serde_json::json!({
                    "error": format!("{:?}", err),
                });

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
        };
        
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_DeleteClientMemories(
        env: JNIEnv,
        _: JClass,
        j_client: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_client).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");

        let client: Client = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };

        let output = match csml_engine::delete_client_memories(&client) {
            Ok(value) => {
                let value= serde_json::json!(
                    value
                );

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
            Err(err) => {
                let value = serde_json::json!({
                    "error": format!("{:?}", err),
                });

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
        };
        
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_DeleteClientsData(
        env: JNIEnv,
        _: JClass,
        j_client: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_client).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");

        let client: Client = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };

        let output = match csml_engine::delete_client(&client) {
            Ok(value) => {
                let value= serde_json::json!(
                    value
                );

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
            Err(err) => {
                let value = serde_json::json!({
                    "error": format!("{:?}", err),
                });

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
        };
        
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_DeleteBotData(
        env: JNIEnv,
        _: JClass,
        j_bot_id: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_bot_id).expect("invalid pattern string").as_ptr()
            )
        );
        let bot_id = c_string.into_string().expect("into_string() call failed");

        let output = match csml_engine::delete_all_bot_data(&bot_id) {
            Ok(value) => {
                let value= serde_json::json!(
                    value
                );

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
            Err(err) => {
                let value = serde_json::json!({
                    "error": format!("{:?}", err),
                });

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
        };
        
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_DeleteExpiredData(
        env: JNIEnv,
        _: JClass,
    ) -> jstring {
        let output = match csml_engine::delete_expired_data() {
            Ok(value) => {
                let value= serde_json::json!(
                    value
                );

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
            Err(err) => {
                let value = serde_json::json!({
                    "error": format!("{:?}", err),
                });

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
        };
        
        output.into_inner()
    }


    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_GetClientMemories(
        env: JNIEnv,
        _: JClass,
        j_client: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_client).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");
    
        let client: Client = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };

        let output =  match csml_engine::get_client_memories(&client) {
            Ok(value) => {
                let value= serde_json::json!(
                    value
                );

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
            Err(err) => {
                let value = serde_json::json!({
                    "error": format!("{:?}", err),
                });

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
        };
        
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_GetClientMemory(
        env: JNIEnv,
        _: JClass,
        j_client: JString,
        j_memory_key: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_client).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");
    
        let client: Client = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };

        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_memory_key).expect("invalid pattern string").as_ptr()
            )
        );
        let memory_key = c_string.into_string().expect("into_string() call failed");

        let output =  match csml_engine::get_client_memory(&client, &memory_key) {
            Ok(value) => {
                let value= serde_json::json!(
                    value
                );

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
            Err(err) => {
                let value = serde_json::json!({
                    "error": format!("{:?}", err),
                });

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
        };
        
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_GetClientConversations(
        env: JNIEnv,
        _: JClass,
        j_client: JString,
        j_limit: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_client).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");
    
        let client: Client = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_limit).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");
    
        let params: LimitPaginationQueryParams = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };

        let output =  match csml_engine::get_client_conversations(&client, params.limit, params.pagination_key) {
            Ok(value) => {
                let value= serde_json::json!(
                    value
                );

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
            Err(err) => {
                let value = serde_json::json!({
                    "error": format!("{:?}", err),
                });

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
        };

        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_example_greeting_CsmlBindings_GetClientMessages(
        env: JNIEnv,
        _: JClass,
        j_client: JString,
        j_limit: JString,
    ) -> jstring {
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_client).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");
    
        let client: Client = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };
        let c_string = CString::from(
            CStr::from_ptr(
                env.get_string(j_limit).expect("invalid pattern string").as_ptr()
            )
        );
        let string = c_string.into_string().expect("into_string() call failed");
    
        let params: LimitPaginationQueryParams = match serde_json::from_str(&string) {
            Ok(body) => body,
            Err(_err) => {
                let output = env.new_string("client bad format").expect("Couldn't create java string!");
                return output.into_inner();
            }
        };

        let output =  match csml_engine::get_client_messages(&client, params.limit, params.pagination_key) {
            Ok(value) => {
                let value= serde_json::json!(
                    value
                );

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
            Err(err) => {
                let value = serde_json::json!({
                    "error": format!("{:?}", err),
                });

                env.new_string(value.to_string()).expect("Couldn't create java string!")
            },
        };

        output.into_inner()
    }

}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// fn make_migrations(mut cx: FunctionContext) -> JsResult<JsValue> {

//     match csml_engine::make_migrations() {
//         Ok(value) => {
//             let value = serde_json::json!(
//                 value
//             );

//             Ok(neon_serde::to_value(&mut cx, &value)?)
//         },
//         Err(err) => {
//             let value = serde_json::json!({
//                 "error": format!("{:?}", err),
//             });

//             Ok(neon_serde::to_value(&mut cx, &value)?)
//         },
//     }
// }

// fn get_bot_versions_limit(mut cx: FunctionContext) -> JsResult<JsValue> {
//     let bot_id = cx.argument::<JsString>(0)?.value();

//     let jsparams = cx.argument::<JsValue>(1)?;
//     let jsonparams: Value = neon_serde::from_value(&mut cx, jsparams)?;
//     let params: LimitPaginationQueryParams = serde_json::from_value(jsonparams).unwrap();

//     match csml_engine::get_bot_versions(&bot_id, params.limit, params.pagination_key) {
//         Ok(value) => {
//             Ok(neon_serde::to_value(&mut cx, &value)?)
//         },
//         Err(err) => {
//             let value = serde_json::json!({
//                 "error": format!("{:?}", err),
//             });

//             Ok(neon_serde::to_value(&mut cx, &value)?)
//         },
//     }
// }

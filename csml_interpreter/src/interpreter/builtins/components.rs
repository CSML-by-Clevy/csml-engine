use crate::data::error_info::ErrorInfo;
use std::{fs, env};
use std::io::prelude::*;

pub fn read() -> Result<serde_json::Map<String, serde_json::Value>, ErrorInfo> {
    // by default, load components from the `components` dir in the pwd, or override with env var
    let components_dir = env::var("COMPONENTS_DIR").unwrap_or("./components".to_string());
    let paths = fs::read_dir(components_dir)?;
    let mut components = serde_json::Map::new();

    for path in paths {
        let mut file = fs::File::open(path?.path().to_str().unwrap())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let component: serde_json::Value = serde_json::from_str(&contents)?;

        if let serde_json::Value::Object(mut obj) = component {
            components.append(&mut obj);
        } else {
            // TODO: error msg
            println!("native component bad format {:?}", component);
        }
    }

    Ok(components)
}

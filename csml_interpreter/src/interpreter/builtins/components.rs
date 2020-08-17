use crate::data::error_info::ErrorInfo;
use std::path::Path;
use std::{fs, env};
use std::io::prelude::*;

// TODO: tmp while there is no CARGO_WORKSPACE_MANIFEST_DIR
fn get_components_dir() -> String {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut ancestors = Path::new(&manifest_dir).ancestors();

    while let Some(path) = ancestors.next() {
        let tmp = path.join("components");
        if tmp.exists(){
            return format!("{}", tmp.display()) ;
        }
    }

    panic!("components_dir not found")
}

pub fn read() -> Result<serde_json::Map<String, serde_json::Value>, ErrorInfo> {
    let paths = fs::read_dir(get_components_dir())?;
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

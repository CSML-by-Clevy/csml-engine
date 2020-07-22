use std::fs;
use std::io::prelude::*;

pub fn read() -> Result<serde_json::map::Map<String, serde_json::Value>, std::io::Error> {
    let paths = fs::read_dir("components/")?;
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
            println!("error ------------------{:?}", component);
        }
    }

    Ok(components)
}

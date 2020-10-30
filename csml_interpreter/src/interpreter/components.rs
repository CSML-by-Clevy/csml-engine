pub mod audio;
pub mod button;
pub mod card;
pub mod carousel;
pub mod file;
pub mod image;
pub mod question;
pub mod text;
pub mod typing;
pub mod url;
pub mod video;
pub mod wait;

use crate::data::error_info::ErrorInfo;
use std::{env, fs};
use std::io::prelude::*;
use std::path::Path;

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn read_components_dir(
    dir: &Path,
    map: &mut serde_json::Map<String, serde_json::Value>,
) -> Result<(), ErrorInfo> {

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                let mut file = fs::File::open(path.to_str().unwrap())?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                let component: serde_json::Value = serde_json::from_str(&contents).unwrap();

                if let serde_json::Value::Object(obj) = component {
                    let file_stem = path.file_stem().unwrap();
                    let component_name: &str = file_stem.to_str().unwrap();
                    map.insert(component_name.to_owned(), serde_json::json!(obj));
                } else {
                    // TODO: error msg
                    eprintln!("Invalid custom component formatting {:?}", component);
                }
            }
        }
    }
    else {
        eprintln!("{} is not a directory!", dir.to_str().unwrap());
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn load_components() -> Result<serde_json::Map<String, serde_json::Value>, ErrorInfo> {
    let mut map = serde_json::Map::new();

    audio::add_audio(&mut map);
    button::add_button(&mut map);
    card::add_card(&mut map);
    carousel::add_carousel(&mut map);
    file::add_file(&mut map);
    image::add_image(&mut map);
    question::add_question(&mut map);
    text::add_text(&mut map);
    typing::add_typing(&mut map);
    url::add_url(&mut map);
    video::add_video(&mut map);
    wait::add_wait(&mut map);

    // load components from the `COMPONENTS_DIR` dir if any
    let components_dir = match env::var("COMPONENTS_DIR") {
        Ok(dir) => dir,
        Err(_) => return Ok(map),
    };

    let components_dir = Path::new(&components_dir);
    let _res = read_components_dir(&components_dir, &mut map)?;

    Ok(map)
}

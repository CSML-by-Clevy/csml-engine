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
use std::io::prelude::*;
use std::{env, fs};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn read_components_dir(
    map: &mut serde_json::Map<String, serde_json::Value>,
) -> Result<(), ErrorInfo> {
    // load components from the `COMPONENTS_DIR` dir if any
    let components_dir = match env::var("COMPONENTS_DIR") {
        Ok(components_dir) => components_dir,
        Err(_) => return Ok(()),
    };

    let paths = fs::read_dir(components_dir)?;

    for path in paths {
        let mut file = fs::File::open(path?.path().to_str().unwrap())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let component: serde_json::Value = serde_json::from_str(&contents)?;

        if let serde_json::Value::Object(mut obj) = component {
            map.append(&mut obj);
        } else {
            // TODO: error msg
            eprintln!("native component bad format {:?}", component);
        }
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

    let _res = read_components_dir(&mut map)?;

    Ok(map)
}

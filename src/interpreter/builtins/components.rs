use std::fs;
use std::io::prelude::*;

pub fn read() -> std::io::Result<()> {
    let paths = fs::read_dir("components/")?;
    let mut components = serde_json::Map::new();

    for path in paths {
        // println!("Name: {}", path?.path().display())
        let mut file = fs::File::open(path?.path().to_str().unwrap() )?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let component: serde_json::Value = serde_json::from_str(&contents)?;

        println!("obj => {:#?} \n\n", component);

        let val = component.get("Button").unwrap();

        println!("value => {:#?}", val);

        components
    }

    Ok(())
}
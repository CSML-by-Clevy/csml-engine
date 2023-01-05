use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub const DEFAULT_FLOW: &[u8] = b"start:
    say \"Hello World!\"
";

#[derive(Debug)]
pub enum DataBase {
    MongoDB {
        uri: String,
    },
    Postgres {
        uri: String,
    },
    DynamoDB {
        dynamodb_region: DynamoRegion,
        dynamodb_table: String,
        s3_region: S3Region,
        s3_bucket: String,
    },
}

#[derive(Debug)]
pub enum DynamoRegion {
    Region(String),
    Endpoint(String),
}

#[derive(Debug)]
pub enum S3Region {
    Region(String),
    Endpoint(String),
}

#[derive(Debug)]
pub struct Env {
    pub bot_name: String,
    pub database: DataBase,
    pub encryption: Option<String>,
}

impl Env {
    // pub fn is_completed(&self) -> bool {
    //     // self.database.is_completed()
    //     true
    // }

    pub fn to_text(&self) -> String {
        match &self.database {
            DataBase::MongoDB { uri } => {
                let mut env = format!(
                    "ENGINE_DB_TYPE=mongodb\nMONGODB_DATABASE=csml\nMONGODB_URI={}",
                    uri
                );

                if let Some(encryption) = &self.encryption {
                    env.push_str(&format!("\n\nENCRYPTION_SECRET={}", encryption));
                }

                env
            }
            DataBase::Postgres { uri } => {
                let mut env = format!("ENGINE_DB_TYPE=postgresql\nPOSTGRESQL_URL={}", uri);

                if let Some(encryption) = &self.encryption {
                    env.push_str(&format!("\n\nENCRYPTION_SECRET={}", encryption));
                }

                env
            }
            DataBase::DynamoDB {
                dynamodb_region,
                dynamodb_table,
                s3_region,
                s3_bucket,
            } => {
                let mut env = String::new();

                env.push_str(&"ENGINE_DB_TYPE=dynamodb\n");

                match dynamodb_region {
                    DynamoRegion::Region(region) => {
                        env.push_str(&format!("AWS_REGION={}\n", region))
                    }
                    DynamoRegion::Endpoint(endpoint) => {
                        env.push_str(&format!("AWS_DYNAMODB_ENDPOINT={}\n", endpoint))
                    }
                };
                env.push_str(&format!("AWS_DYNAMODB_TABLE={}\n", dynamodb_table));

                match s3_region {
                    S3Region::Region(region) => {
                        env.push_str(&format!("AWS_S3_REGION={}\n", region))
                    }
                    S3Region::Endpoint(endpoint) => {
                        env.push_str(&format!("AWS_S3_ENDPOINT={}\n", endpoint))
                    }
                };
                env.push_str(&format!("AWS_S3_BUCKET={}\n", s3_bucket));

                if let Some(encryption) = &self.encryption {
                    env.push_str(&format!("\n\nENCRYPTION_SECRET={}", encryption));
                }

                env
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub name: String,
    pub bot_version: String,
    pub engine_version: String,
    pub default_flow: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    pub commands: Vec<Vec<String>>,
}

impl Manifest {
    fn default(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            bot_version: "0.1.0".to_owned(),
            engine_version: "1.6.3".to_owned(), // get the current version automatically
            default_flow: "default".to_owned(),
            authors: None,
            description: None,
            repository: None,
            license: None,
            commands: vec![],
        }
    }
}

fn gen_manifest(name: &str) -> serde_yaml::Value {
    serde_yaml::to_value(Manifest::default(name)).unwrap()
}

struct IgnoreList {
    /// git like formatted entries
    ignore: Vec<String>,
}

impl IgnoreList {
    /// constructor to build a new ignore file
    fn new() -> IgnoreList {
        IgnoreList { ignore: Vec::new() }
    }

    /// Add a new entry to the ignore list. Requires three arguments with the
    /// entry in possibly three different formats. One for "git style" entries,
    /// one for "mercurial style" entries and one for "fossil style" entries.
    fn push(&mut self, ignore: &str) {
        self.ignore.push(ignore.to_string());
    }

    /// format_existing is used to format the IgnoreList when the ignore file
    /// already exists. It reads the contents of the given `BufRead` and
    /// checks if the contents of the ignore list are already existing in the
    /// file.
    fn format_existing<T: BufRead>(&self, existing: T) -> String {
        // TODO: is unwrap safe?
        let existing_items = existing.lines().collect::<Result<Vec<_>, _>>().unwrap();

        let ignore_items = &self.ignore;

        let mut out = String::new();

        for item in ignore_items {
            if existing_items.contains(item) {
                out.push('#');
            }
            out.push_str(item);
            out.push('\n');
        }

        out
    }
}

/// file entries are filtered out.
fn write_ignore_file<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let mut ignore = IgnoreList::new();
    ignore.push(".env");
    ignore.push("metadata.yml");

    let mut gitignore = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&path)?;

    // let mut gitignore = File::create(&path)?;
    let ignore: String = match File::open(&path) {
        Err(err) => return Err(err),
        Ok(file) => ignore.format_existing(BufReader::new(file)),
    };

    gitignore.write_all(ignore.as_bytes())?;
    Ok(())
}

pub fn init_git_repo<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    git2::Repository::init(path).unwrap();
    Ok(())
}

pub fn init_with_env(env_info: Env) -> std::io::Result<()> {
    // error: destination `/Users/amerelo/CSML/csml_cli/demo` already exists

    create_dir_all(&env_info.bot_name)?;

    let mut env = File::create(&format!("{}/.env", &env_info.bot_name))?; //&path.join("/.env")
                                                                          // env.write_all(DEFAULT_ENV)?;
    env.write_all(env_info.to_text().as_bytes())?;

    let manifest = File::create(&format!("{}/manifest.yaml", &env_info.bot_name))?; //&path.join("/manifest.yaml")
    let manifest_yaml = gen_manifest(&env_info.bot_name);
    serde_yaml::to_writer(manifest, &manifest_yaml).unwrap();

    // init metadata.yaml
    let metadata = File::create(&format!("{}/metadata.yaml", &env_info.bot_name))?;
    serde_yaml::to_writer(metadata, &serde_json::json!({})).unwrap();

    // init .git
    init_git_repo(&format!("{}/.git", &env_info.bot_name)).unwrap();

    // init .gitignore
    write_ignore_file(&format!("{}/.gitignore", &env_info.bot_name)).unwrap(); //&path

    create_dir_all(&format!("{}/src", &env_info.bot_name))?;
    let mut default_flow = File::create(&format!("{}/src/default.csml", &env_info.bot_name))?; //&path.join("/src/default.csml")
    default_flow.write_all(DEFAULT_FLOW)?;

    Ok(())
}

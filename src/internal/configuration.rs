use config::{self, Config, ConfigError};
use serde::{Deserialize, Serialize};
use std::{env, fmt};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    // read from env or set mannual
    pub version: String,
    pub file_path: String,
    pub threads: usize,
    pub release: bool,

    // load from yaml file or use default
    pub database: Database,
    pub jwt: Jwt,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Jwt {
    pub key: String,
    pub alive_mins: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Database {
    pub conn: String,
    pub db: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            file_path: "".to_string(),
            threads: 1,
            release: false,

            database: Database::default(),
            jwt: Jwt { key: "".to_string(), alive_mins: 0 },
        }
    }
}

impl fmt::Display for Database {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "{}/{}", &self.conn, &self.db)
    }
}

pub fn load_config(fp: &str) -> Result<Configuration, ConfigError> {
    let mut builder = Config::builder();

    builder = builder
        .set_default("version", "0.1.0")?
        .set_override("version", env!("CARGO_PKG_VERSION"))?
        .set_default("file_path", "")?
        .set_default("threads", "1")?
        .set_default("release", "false")?
        .add_source(config::File::new(fp, config::FileFormat::Yaml));

    builder.build()?.try_deserialize::<Configuration>()
}

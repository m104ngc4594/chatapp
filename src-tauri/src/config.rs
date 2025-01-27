use crate::utils::config_dir;
use anyhow::Result;
use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub chat: String,
    pub notification: String,
}

/// app config
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn try_new() -> Result<Self> {
        let config_file = config_dir().join("app.yml");
        let default_config = include_str!("./fixtures/config.default.yml");
        let config = Config::builder()
            .add_source(File::from_str(default_config, FileFormat::Yaml))
            .add_source(File::with_name(&config_file.to_string_lossy()).required(false))
            .build()?;

        Ok(config.try_deserialize()?)
    }
}

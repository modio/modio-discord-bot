use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::Result;

pub const DBL_OVERRIDE_BOT_ID: &str = "DBL_OVERRIDE_BOT_ID";

const DEFAULT_MODIO_HOST: &str = "https://api.mod.io/v1";

#[derive(Deserialize)]
pub struct Config {
    pub bot: BotConfig,
    pub modio: ModioConfig,
}

#[derive(Deserialize)]
pub struct BotConfig {
    pub token: String,
    pub dbl_token: Option<String>,
    pub database_url: String,
}

#[derive(Deserialize)]
pub struct ModioConfig {
    #[serde(default = "default_modio_host")]
    pub host: String,
    pub api_key: String,
    pub token: Option<String>,
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Config> {
    let data = fs::read_to_string(path)?;
    Ok(toml::from_str(&data)?)
}

fn default_modio_host() -> String {
    DEFAULT_MODIO_HOST.to_owned()
}

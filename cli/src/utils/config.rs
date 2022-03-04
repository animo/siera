use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    pub name: String,
    pub endpoint: String,
    pub api_key: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Configurations {
    pub configurations: Vec<Configuration>,
}

pub fn get_config_from_path(config_path: PathBuf) -> Result<Configurations> {
    let out: Result<String> =
        std::fs::read_to_string(config_path).map_err(|_| Error::InvalidConfigurationPath.into());
    serde_yaml::from_str::<Configurations>(&out?)
        .map_err(|_| Error::InvalidConfigurationStructure.into())
}

pub fn get_config_path() -> Result<PathBuf> {
    if cfg!(windows) {
        let home = "C:\\Program Files\\Common Files";
        Ok(Path::new(home).join("aries-cli\\config.ini"))
    } else if cfg!(unix) {
        let home = option_env!("HOME").ok_or_else(|| Error::HomeNotFound);
        Ok(Path::new(&home?).join(".config/aries-cli/config.ini"))
    } else {
        Err(Error::OsUnknown.into())
    }
}

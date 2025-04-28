use crate::error::ReportError;
use crate::error::Result;
use homedir::my_home;
use serde::Deserialize;
use std::str::FromStr;

/// The hledger-report configuration.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Config {
    #[serde(default)]
    pub hledger: HledgerConfig,
}

impl Config {
    fn path() -> Result<std::path::PathBuf> {
        let env_path = std::env::var("HLEDGER_REPORT_CONFIG");
        match env_path {
            Ok(env) => std::path::PathBuf::from_str(&env).map_err(|_| ReportError::ConfigPath),
            Err(_) => match my_home() {
                Ok(home) => match home {
                    Some(home) => {
                        let mut path = home.into_os_string();
                        path.push("/.config/hledger-report/config.toml");
                        Ok(path.into())
                    }
                    None => Err(ReportError::ConfigPath),
                },
                Err(_) => Err(ReportError::ConfigPath),
            },
        }
    }

    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        let config = std::fs::read_to_string(&path).map_err(|_| ReportError::ConfigRead(path))?;

        toml::from_str::<Config>(&config).map_err(ReportError::ConfigParse)
    }
}

/// Configuration options regarding the interactions with the `hledger` process.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct HledgerConfig {
    pub path: String,
}

impl Default for HledgerConfig {
    fn default() -> Self {
        Self {
            path: "hledger".to_owned(),
        }
    }
}

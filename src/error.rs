use thiserror::Error;

/// Enumeration of all possible application errors, that can be reported to the user.
#[derive(Debug, Error)]
pub enum ReportError {
    #[error("hledger execution failed: {0}")]
    HledgerExecution(#[from] std::io::Error),
    #[error("Encountered non UTF-8 encoding or string conversion failed: {0}")]
    StringConversion(#[from] std::str::Utf8Error),
    #[error("The hledger output could not be parsed as valid JSON: {0}")]
    HledgerOutputJsonInvalid(#[from] serde_json::error::Error),
    #[error(
        "Failed to provide the path to the configruation file. Please provide the path to the configuration file in the environment variable \"HLEDGER_REPORT_CONFIG\" to fix this error."
    )]
    ConfigPath,
    #[error("Failed to read configuration file \"{0}\"")]
    ConfigRead(std::path::PathBuf),
    #[error("Failed to parse configuration file: {0}")]
    ConfigParse(#[from] toml::de::Error),
}

pub type Result<T> = std::result::Result<T, ReportError>;

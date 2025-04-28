use std::process::Command;

use crate::error::*;

/// Executes the given hledger binary and returns the JSON output as given type *T*.
/// The argument `-O=json` is added to the *arguments*, if it is not contained.
///
/// This function returns an `ReportError::HledgerOutputJsonInvalid` if the hledger output is not valid JSON or if the output can not be transformed to *T*.
pub fn execute_hledger<T: serde::de::DeserializeOwned>(
    hledger_binary_path: &str,
    arguments: Vec<String>,
) -> Result<T> {
    let actual_arguments = if !arguments.iter().any(|arg| arg.starts_with("-O")) {
        let mut arguments = arguments.clone();
        arguments.push("-O=json".to_owned());
        arguments
    } else {
        arguments
    };

    let output = Command::new(hledger_binary_path)
        .args(actual_arguments)
        .output()
        .map_err(ReportError::HledgerExecution)?;

    let json = std::str::from_utf8(&output.stdout)?;

    serde_json::from_str::<T>(json).map_err(ReportError::HledgerOutputJsonInvalid)
}

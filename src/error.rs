use thiserror::Error;

/// enumeration of all possible application errors
#[derive(Debug, Error)]
pub enum ReportError {
    #[error("This is a test")]
    SampleError,
}

pub type Result<T> = std::result::Result<T, ReportError>;

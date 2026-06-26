use thiserror::Error;

/// Unified error type for the twinex crate.
#[derive(Error, Debug)]
pub enum TwinexError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error at {path}:{line}: {message}")]
    Parse {
        path: String,
        line: usize,
        message: String,
    },

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Format error: {0}")]
    Format(String),

    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Nothing to generate: the resulting file would be empty")]
    NothingToGenerate,

    #[error("Validation error:\n{0}")]
    Validation(String),
}

pub type Result<T> = std::result::Result<T, TwinexError>;

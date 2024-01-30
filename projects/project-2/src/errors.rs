use thiserror::Error;

pub type Result<T> = std::result::Result<T, KvsError>;

#[derive(Debug, Error)]
pub enum KvsError {
    #[error("Failed to open log file")]
    OpenFile(#[source] std::io::Error),

    #[error("Failed to append to log")]
    AppendToLog(#[source] bincode::Error),

    #[error("Failed to read from log")]
    ReadFromLog(#[source] bincode::Error),

    #[error("Key not found")]
    KeyNotFound,

    #[error("An unexpected I/O error occurred")]
    IoError(#[from] std::io::Error),
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LeviathanError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
}

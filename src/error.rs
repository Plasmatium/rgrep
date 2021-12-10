use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileMatcherError {
    #[error("invalid line block: missing medium")]
    InvalidLineBlockMissingMedium,
    #[error("invalid around lines: around lines length must > overflowed lines")]
    InvalidAroundLines,
    #[error("invalid line block: missing after")]
    InvalidLineBlockMissingAfter,
}

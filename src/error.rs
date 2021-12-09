use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileMatcherError {
    #[error("invalid line block: missing medium")]
    InvalidLineBlock,
}

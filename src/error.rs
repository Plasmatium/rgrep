use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileMatcherError {
    #[error("invalid line block: missing medium")]
    InvalidLineBlockMissingMedium,
}


#[derive(Error, Debug)]
pub enum ArgsError<'a> {
    #[error("invalid arg --{arg:?} (expected positive integer, found {found:?})")]
    InvalidParseNumberArg { arg: &'a str, found: &'a str}
}
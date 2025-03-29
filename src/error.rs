use thiserror::Error;

#[derive(Error, Debug)]
pub enum RadmlError {
    #[error("undefined")]
    Undefined,
}

pub type Result<T> = core::result::Result<T, RadmlError>;

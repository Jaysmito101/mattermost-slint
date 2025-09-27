use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Generic Error {0}")]
    GenericError(String),
    #[error("Invalid Param Error {0}")]
    InvalidParamError(String),
    #[error("Slint Error: {0}")]
    SlintError(slint::PlatformError),
}

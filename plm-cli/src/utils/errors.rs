use std::io;
pub type PlmResult<T> = Result<T, PlmError>;

#[derive(thiserror::Error, Debug)]
pub enum PlmError {
    #[error("Some internal error occured: {0}")]
    InternalError(String),

    #[error("Error on file system operation: {0:?}")]
    FileSystemError(io::Error),

    #[error("Error on serialization operation: {0:?}")]
    SerializationError(io::Error),
}

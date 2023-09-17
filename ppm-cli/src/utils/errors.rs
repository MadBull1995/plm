pub type PPMResult<T> = Result<T, PpmError>;

#[derive(thiserror::Error, Debug)]
pub enum PpmError {
    #[error("Some internal error occured")]
    InternalError
}
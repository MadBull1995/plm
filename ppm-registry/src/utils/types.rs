use super::error::RegistryError;

pub type RegistryResult<T> = Result<T, RegistryError>;
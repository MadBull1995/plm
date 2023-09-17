#[derive(thiserror::Error, Debug)]
pub enum RegistryError {
    #[error("Invalid config setup: {0:?}")]
    InvalidConfigSetup(String),
}
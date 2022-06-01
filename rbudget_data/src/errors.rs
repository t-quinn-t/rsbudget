
/// Convert different error types to a custom error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)] 
    DBError(#[from] sqlx::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Failed reading .env values")]
    EVError(#[from] std::env::VarError)
}
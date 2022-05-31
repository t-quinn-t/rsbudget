
/// Convert different error types to a custom error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database Error Occurred.")] 
    DBError(#[from] sqlx::Error),
    #[error("Environment Variables Loading Failed.")]
    EVError(#[from] std::io::Error),
}
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(clippy::module_name_repetitions)]
pub enum LoginError {
    #[error("invalid credentials provided")]
    InvalidCredentials,

    #[error(transparent)]
    Database(#[from] mongodb::error::Error),

    #[error(transparent)]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("User queried without ID. This error is supposed to be unreachable.")]
    UserWithoutId,
}

#[derive(Debug, Error)]
#[allow(clippy::module_name_repetitions)]
pub enum RegisterError {
    #[error(transparent)]
    Database(#[from] mongodb::error::Error),

    #[error(transparent)]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("user with this name already exists")]
    UserExists,
}

#[derive(Debug, Error)]
#[allow(clippy::module_name_repetitions)]
pub enum VerifyError {
    #[error(transparent)]
    Database(#[from] mongodb::error::Error),

    #[error("token expired")]
    Expired,

    #[error("token invalid")]
    Invalid,
}

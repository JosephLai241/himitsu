//! Contains errors that may be raised in the application.

use argon2;
use thiserror::Error;

/// Contains variants for errors that may be raised throughout this program.
#[derive(Debug, Error)]
pub enum SkeletonsError {
    /// An error occurred while performing Argon2 tasks.
    #[error("Argon2 error: {0}")]
    Argon2Error(#[from] argon2::Error),
}

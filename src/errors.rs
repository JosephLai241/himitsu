//! Contains errors that may be raised in the application.

use argon2;
use inquire;
use serde_json;
use thiserror::Error;

use std::io;

/// Contains variants for errors that may be raised throughout this program.
#[derive(Debug, Error)]
pub enum SkeletonsError {
    /// An error occurred while performing Argon2 tasks.
    #[error("Argon2 error: {0}")]
    Argon2Error(#[from] argon2::Error),

    /// An error occurred while attempting to access application-specific directories.
    #[error("Could not access system application directories!")]
    ApplicationError,

    /// The user failed to log in within 3 tries.
    #[error("Goodbye.")]
    FailedToLogin,

    /// An error occurred while attempting to process `inquire` prompts.
    #[error("Inquire error: {0}")]
    InquireError(#[from] inquire::error::InquireError),

    /// An error occurred while performing any IO tasks.
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),

    /// An error occurred while performing any path-related tasks.
    #[error("Path error: {0}")]
    PathError(String),

    /// An error occurred while performing any serde_json tasks.
    #[error("Serde JSON error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

//! Contains errors that may be raised in the application.

use anyhow;
use argon2;
use inquire;
use regex;
use serde_json;
use thiserror::Error;

use std::{io, string::FromUtf8Error};

/// Contains variants for errors that may be raised throughout this program.
#[derive(Debug, Error)]
pub enum SkeletonsError {
    /// An error occurred while performing Argon2 tasks.
    #[error("Argon2 error: {0}")]
    Argon2Error(#[from] argon2::Error),

    /// An error occurred while attempting to access application-specific directories.
    #[error("Could not access system application directories!")]
    ApplicationError,

    /// An error occurred while attempting to dencrypt something.
    #[error("AEAD decryption error: {0}")]
    AEADDencryptionError(String),

    /// An error occurred while attempting to encrypt something.
    #[error("AEAD encryption error: {0}")]
    AEADEncryptionError(String),

    /// An error occurred while attempting to get a handle to the system clipboard.
    #[error("Clipboard context error: {0}")]
    ClipboardContextError(String),

    /// An error occurred while attempting to manipulate the system clipboard's contents.
    #[error("Clipboard error: {0}")]
    ClipboardContentsError(String),

    /// The user failed to log in within 3 tries.
    #[error("Goodbye.")]
    FailedToLogin,

    /// An error occurred while attempting to convert bytes to a string.
    #[error("FromUtf8Error: {0}")]
    FromUtf8Error(#[from] FromUtf8Error),

    /// An error occurred while attempting to process `inquire` prompts.
    #[error("Inquire error: {0}")]
    InquireError(#[from] inquire::error::InquireError),

    /// An error occurred while performing any IO tasks.
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),

    /// An error occurred while traversing the lookup table.
    #[error("Lookup table error: {0}")]
    LookupError(String),

    /// An error occurred while performing any path-related tasks.
    #[error("Path error: {0}")]
    PathError(String),

    /// An error occurred while executing a regex expression.
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),

    /// An error occurred while performing any serde_json tasks.
    #[error("Serde JSON error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    /// AN error occurred while attempting to store the lookup table.
    #[error("Store lookup table error: {0}")]
    StoreLookupTableError(String),

    /// An error occurred while attempting to store a nonce.
    #[error("Store nonce error: {0}")]
    StoreNonceError(String),

    /// An error occurred while attempting to store a secret.
    #[error("Store secret error: {0}")]
    StoreSecretError(String),

    /// Catch any other errors that may arise, such as `bail!`s returned via `Anyhow`.
    #[error(transparent)]
    Transparent(#[from] anyhow::Error),
}

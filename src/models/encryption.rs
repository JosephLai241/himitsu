//! Contains structs used for encryption/decryption.

use serde::{Deserialize, Serialize};

/// Contains the password_hash and salt used for encryption/decryption.
#[derive(Debug, Deserialize, Serialize)]
pub struct Encryption {
    /// The Argon2 password hash.
    pub password_hash: Vec<u8>,
    /// The salt associated with the master password.
    pub salt: String,
}

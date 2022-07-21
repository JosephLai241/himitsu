//! Contains utilities used for authentication.

use argon2::{Config, ThreadMode::Parallel, Variant::Argon2id};

use crate::{errors::SkeletonsError, models::encryption::Encryption};

/// Returns the Argon2 configuration object. This object contains the parameters
/// used to generate a secure password.
///
/// The settings that are specified in `Config` are as follows:
/// * Generate a 32 byte key
/// * Use 1 degree of parallellism
/// * Use 15MB of memory
/// * Use 2 passes
/// * Use the Argon2id variant
///
pub fn get_argon2_config<'a>() -> Config<'a> {
    Config {
        hash_length: 32,
        lanes: 1,
        mem_cost: 15000,
        thread_mode: Parallel,
        time_cost: 2,
        variant: Argon2id,
        ..Default::default()
    }
}

/// Generate a new hash using Argon2. See [`get_argon2_config`]'s docstring for
/// Argon2's hash generation configuration settings.
pub fn generate_raw_hash(password: &str, salt: &[u8; 32]) -> Result<Vec<u8>, SkeletonsError> {
    let argon2_config = get_argon2_config();

    Ok(argon2::hash_raw(password.as_bytes(), salt, &argon2_config)?)
}

/// Verify the password against the stored Argon2 password hash.
pub fn check_authorization(
    encryption_values: &Encryption,
    password: &str,
) -> Result<bool, SkeletonsError> {
    argon2::verify_raw(
        password.as_bytes(),
        &encryption_values.salt,
        &encryption_values.password_hash,
        &get_argon2_config(),
    )
    .map_or_else(|error| Err(SkeletonsError::Argon2Error(error)), Ok)
}

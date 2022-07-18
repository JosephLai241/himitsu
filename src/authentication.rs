//! Contains utilities used for authentication.

use argon2::{hash_encoded, Config, ThreadMode::Parallel, Variant::Argon2id};

use crate::errors::SkeletonsError;

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
fn get_argon2_config<'a>() -> Config<'a> {
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
pub fn generate_hash(password: &str, salt: &str) -> Result<String, SkeletonsError> {
    let argon2_config = get_argon2_config();

    Ok(hash_encoded(
        password.as_bytes(),
        salt.as_bytes(),
        &argon2_config,
    )?)
}

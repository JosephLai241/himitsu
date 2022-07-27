//! Contains functions pertaining to initial setup for the lookup table.

use std::fs;

use chacha20poly1305::{
    aead::{Aead, NewAead},
    Key, XChaCha20Poly1305, XNonce,
};
use rand::{self, rngs::OsRng, RngCore};

use crate::{authentication, errors::HimitsuError, models::metadata::LookupTable};

use super::utils;

/// Create a new lookup table, salt, and nonce, then write the values to individual files in the
/// `lookup` directory.
pub fn create_lookup(password: &str) -> Result<(), HimitsuError> {
    let lookup_dir_path = utils::get_lookup_dir_path()?;

    let mut lookup_salt = [0u8; 32];
    let mut nonce = [0u8; 24];
    OsRng.fill_bytes(&mut lookup_salt);
    OsRng.fill_bytes(&mut nonce);

    let argon2_config = authentication::get_argon2_config();
    let key = argon2::hash_raw(password.as_bytes(), &lookup_salt, &argon2_config)?;

    let cipher = XChaCha20Poly1305::new(Key::from_slice(&key));

    let lookup_nonce = XNonce::from_slice(&nonce);

    let lookup_table = LookupTable::new();

    match cipher.encrypt(
        &lookup_nonce,
        serde_json::to_string(&lookup_table)?.as_bytes(),
    ) {
        Ok(encrypted_lookup_table) => {
            if !lookup_dir_path.exists() {
                fs::create_dir_all(&lookup_dir_path)?;
            }

            if let Err(error) = fs::write(lookup_dir_path.join("salt"), lookup_salt) {
                return Err(HimitsuError::StoreSaltError(format!(
                    "Lookup table salt: {error}"
                )));
            }
            if let Err(error) = fs::write(lookup_dir_path.join("nonce"), lookup_nonce) {
                return Err(HimitsuError::StoreNonceError(format!(
                    "Lookup table nonce: {error}"
                )));
            }
            if let Err(error) = fs::write(lookup_dir_path.join("table"), encrypted_lookup_table) {
                return Err(HimitsuError::StoreLookupTableError(error.to_string()));
            }

            Ok(())
        }
        Err(error) => Err(HimitsuError::AEADEncryptionError(format!(
            "Lookup table encryption error: {error}"
        ))),
    }
}

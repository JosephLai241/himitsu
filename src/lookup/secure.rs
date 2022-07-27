//! Contains functions for securely encrypting/decrypting the lookup table.

use std::fs;

use argon2;
use chacha20poly1305::{
    aead::{Aead, NewAead},
    Key, XChaCha20Poly1305, XNonce,
};

use crate::{authentication, errors::HimitsuError, models::metadata::LookupTable};

use super::utils;

/// Encrypt the lookup table and write the table to the `"table"` file.
pub fn encrypt_lookup_table(
    password: &str,
    updated_lookup: &mut LookupTable,
) -> Result<(), HimitsuError> {
    let lookup_table_path = utils::get_lookup_dir_path()?.join("table");

    let lookup_salt = utils::get_lookup_salt()?;

    let argon2_config = authentication::get_argon2_config();
    let key = argon2::hash_raw(password.as_bytes(), &lookup_salt, &argon2_config)?;

    let cipher = XChaCha20Poly1305::new(Key::from_slice(&key));

    let lookup_nonce = utils::get_lookup_nonce()?;
    let nonce = XNonce::from_slice(&lookup_nonce);

    match cipher.encrypt(nonce, serde_json::to_string(updated_lookup)?.as_bytes()) {
        Ok(encrypted_lookup_table) => {
            if let Err(error) = fs::write(lookup_table_path, encrypted_lookup_table) {
                return Err(HimitsuError::StoreLookupTableError(error.to_string()));
            }

            Ok(())
        }
        Err(error) => Err(HimitsuError::AEADEncryptionError(format!(
            "Lookup table encryption error: {error}"
        ))),
    }
}

/// Decrypt the lookup table and return its contents.
pub fn decrypt_lookup_table(password: &str) -> Result<LookupTable, HimitsuError> {
    let lookup_salt = utils::get_lookup_salt()?;
    let lookup_nonce = utils::get_lookup_nonce()?;
    let nonce = XNonce::from_slice(&lookup_nonce);

    let argon2_config = authentication::get_argon2_config();
    let key = argon2::hash_raw(password.as_bytes(), &lookup_salt, &argon2_config)?;

    let cipher = XChaCha20Poly1305::new(Key::from_slice(&key));

    let raw_lookup_data = &utils::get_lookup_table()?[..];

    match cipher.decrypt(nonce, raw_lookup_data) {
        Ok(data) => match String::from_utf8(data) {
            Ok(stringified_data) => Ok(serde_json::from_str(&stringified_data)?),
            Err(error) => Err(HimitsuError::FromUtf8Error(error)),
        },
        Err(error) => Err(HimitsuError::AEADDencryptionError(format!(
            "Lookup table decryption error: {}",
            error.to_string()
        ))),
    }
}

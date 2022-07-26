//! Contains functions for securely encrypting/decrypting the lookup table.

use std::fs;

use chacha20poly1305::{
    aead::{Aead, NewAead},
    Key, XChaCha20Poly1305, XNonce,
};

use crate::{
    errors::SkeletonsError,
    models::{encryption::Encryption, metadata::LookupTable},
};

use super::utils::{get_lookup_dir_path, get_lookup_nonce, get_lookup_table};

/// Encrypt the lookup table and write the table to the `"table"` file.
pub fn encrypt_lookup_table(
    encryption_data: &Encryption,
    updated_lookup: &mut LookupTable,
) -> Result<(), SkeletonsError> {
    let lookup_table_path = get_lookup_dir_path()?.join("table");

    let cipher = XChaCha20Poly1305::new(Key::from_slice(&encryption_data.password_hash));

    let lookup_nonce = get_lookup_nonce()?;
    let nonce = XNonce::from_slice(&lookup_nonce);

    match cipher.encrypt(nonce, serde_json::to_string(updated_lookup)?.as_bytes()) {
        Ok(encrypted_lookup_table) => {
            if let Err(error) = fs::write(lookup_table_path, encrypted_lookup_table) {
                return Err(SkeletonsError::StoreLookupTableError(error.to_string()));
            }

            Ok(())
        }
        Err(error) => Err(SkeletonsError::AEADEncryptionError(format!(
            "Lookup table encryption error: {error}"
        ))),
    }
}

/// Decrypt the lookup table and return its contents.
pub fn decrypt_lookup_table(encryption_data: &Encryption) -> Result<LookupTable, SkeletonsError> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(&encryption_data.password_hash));

    let lookup_nonce = get_lookup_nonce()?;
    let nonce = XNonce::from_slice(&lookup_nonce);

    let raw_lookup_data = &get_lookup_table()?[..];

    match cipher.decrypt(nonce, raw_lookup_data) {
        Ok(data) => match String::from_utf8(data) {
            Ok(stringified_data) => Ok(serde_json::from_str(&stringified_data)?),
            Err(error) => Err(SkeletonsError::FromUtf8Error(error)),
        },
        Err(error) => Err(SkeletonsError::AEADDencryptionError(format!(
            "Lookup table decryption error: {}",
            error.to_string()
        ))),
    }
}

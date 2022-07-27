//! Contains utilities for storing secrets.

use std::fs;

use chacha20poly1305::aead::{consts::U24, generic_array::GenericArray};
use directories::ProjectDirs;

use crate::errors::HimitsuError;

/// Store the secret onto the machine.
pub fn store_secret(
    ciphertext: Vec<u8>,
    nonce: &GenericArray<u8, U24>,
    salt: [u8; 32],
    secret_hash: &str,
) -> Result<(), HimitsuError> {
    match ProjectDirs::from("", "", "himitsu") {
        Some(project_directory) => {
            let closet_path = project_directory
                .data_dir()
                .join("closet")
                .join(secret_hash);

            fs::create_dir_all(&closet_path)?;

            if let Err(error) = fs::write(closet_path.join("skeleton"), ciphertext) {
                return Err(HimitsuError::StoreSecretError(error.to_string()));
            }
            if let Err(error) = fs::write(closet_path.join("salt"), salt) {
                return Err(HimitsuError::StoreSaltError(error.to_string()));
            }
            if let Err(error) = fs::write(closet_path.join("nonce"), nonce) {
                return Err(HimitsuError::StoreNonceError(error.to_string()));
            }

            Ok(())
        }
        None => Err(HimitsuError::PathError(
            "Could not get the path to the himitsu application directory!".to_string(),
        )),
    }
}

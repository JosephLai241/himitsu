//! Contains utilities for storing secrets.

use std::fs;

use chacha20poly1305::aead::{consts::U24, generic_array::GenericArray};
use directories::ProjectDirs;

use crate::errors::SkeletonsError;

/// Store the secret onto the machine.
pub fn store_secret(
    ciphertext: Vec<u8>,
    nonce: &GenericArray<u8, U24>,
    secret_hash: &str,
) -> Result<(), SkeletonsError> {
    match ProjectDirs::from("", "", "skeletons") {
        Some(project_directory) => {
            let closet_path = project_directory
                .data_dir()
                .join("closet")
                .join(secret_hash);

            fs::create_dir_all(&closet_path)?;

            if let Err(error) = fs::write(closet_path.join("skeleton"), ciphertext) {
                return Err(SkeletonsError::StoreSecretError(error.to_string()));
            }
            if let Err(error) = fs::write(closet_path.join("nonce"), nonce) {
                return Err(SkeletonsError::StoreNonceError(error.to_string()));
            }

            Ok(())
        }
        None => Err(SkeletonsError::PathError(
            "Could not get the path to the skeletons application directory!".to_string(),
        )),
    }
}

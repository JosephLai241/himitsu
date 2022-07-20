//! Contains utilities for storing secrets.

use std::fs;

use chacha20poly1305::aead::{consts::U24, generic_array::GenericArray};
use data_encoding::HEXLOWER;
use directories::ProjectDirs;
use ring::digest::{Context, SHA256};

use crate::errors::SkeletonsError;

/// Generate a SHA256 hash for a new secret.
fn get_sha256_hash(ciphertext: &Vec<u8>, nonce: &GenericArray<u8, U24>) -> String {
    let mut hash_string = String::from_utf8_lossy(&ciphertext).to_string();
    hash_string.push_str(&String::from_utf8_lossy(&nonce));

    let mut context = Context::new(&SHA256);
    context.update(hash_string.as_bytes());

    HEXLOWER.encode(context.finish().as_ref())
}

/// Store the secret onto the machine.
pub fn store_secret(
    ciphertext: Vec<u8>,
    encrypted_anatomy: Vec<u8>,
    nonce: &GenericArray<u8, U24>,
) -> Result<(), SkeletonsError> {
    match ProjectDirs::from("", "", "skeletons") {
        Some(project_directory) => {
            let secret_hash = get_sha256_hash(&ciphertext, nonce);
            let secrets_path = project_directory
                .data_dir()
                .join("closet")
                .join(secret_hash);

            if !secrets_path.exists() {
                fs::create_dir_all(&secrets_path)?;
            }

            if let Err(error) = fs::write(secrets_path.join("secret.encrypted"), ciphertext) {
                return Err(SkeletonsError::StoreSecretError(error.to_string()));
            }
            if let Err(error) = fs::write(secrets_path.join("nonce"), nonce) {
                return Err(SkeletonsError::StoreNonceError(error.to_string()));
            }
            if let Err(error) = fs::write(secrets_path.join("anatomy"), encrypted_anatomy) {
                return Err(SkeletonsError::StoreAnatomyError(error.to_string()));
            }

            Ok(())
        }
        None => Err(SkeletonsError::PathError(
            "Could not get the path to the skeletons application directory!".to_string(),
        )),
    }
}

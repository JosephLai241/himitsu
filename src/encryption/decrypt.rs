//! Contains decryption functions for `skeletons`.

use std::{fs::File, io::Read, path::PathBuf};

use ansi_term::Color;
use argon2;
use chacha20poly1305::{
    aead::{Aead, NewAead},
    Key, XChaCha20Poly1305, XNonce,
};
use directories::ProjectDirs;
use spinners::{Spinner, Spinners};

use crate::{
    authentication, errors::SkeletonsError, models::encryption::Encryption,
    utils::clipboard::set_clipboard,
};

/// Decrypt a secret based on its SHA256 hash ID and copy it to the system clipboard.
pub fn decrypt_secret(encryption_data: &Encryption, hash_id: &str) -> Result<(), SkeletonsError> {
    let mut decryption_spinner =
        Spinner::new(Spinners::Aesthetic, "Decrypting the secret...".into());

    let hash_path = get_secret_hash_path(hash_id)?;

    let raw_nonce = get_secret_nonce(&hash_path)?;
    let nonce = XNonce::from_slice(&raw_nonce);

    let encrypted_secret = &get_secret(&hash_path)?[..];

    let argon2_config = authentication::get_argon2_config();
    let key = argon2::hash_raw(
        &encryption_data.password_hash,
        &encryption_data.salt,
        &argon2_config,
    )?;

    let cipher = XChaCha20Poly1305::new(Key::from_slice(&key));

    match cipher.decrypt(nonce, encrypted_secret) {
        Ok(decrypted_secret) => {
            decryption_spinner.stop_and_persist(
                "🔓",
                Color::Green
                    .bold()
                    .paint("Successfully decrypted your secret.")
                    .to_string(),
            );

            let secret = String::from_utf8_lossy(&decrypted_secret).to_string();
            match set_clipboard(secret) {
                Ok(()) => {
                    println!("📋 The secret is copied to your clipboard. Press \"<CTRL> + v\" to paste your secret.");
                    Ok(())
                }
                Err(error) => Err(error),
            }
        }
        Err(error) => {
            decryption_spinner.stop_and_persist(
                "❗️",
                Color::Red
                    .bold()
                    .paint("SECRET DECRYPTION FAILED!".to_string())
                    .to_string(),
            );
            Err(SkeletonsError::AEADDencryptionError(format!(
                "Secret decryption error: {}",
                error.to_string()
            )))
        }
    }
}

/// Get the secret's SHA256 hash directory path.
fn get_secret_hash_path(hash_id: &str) -> Result<PathBuf, SkeletonsError> {
    match ProjectDirs::from("", "", "skeletons") {
        Some(project_directory) => Ok(project_directory.data_dir().join("closet").join(hash_id)),
        None => Err(SkeletonsError::PathError(
            "Could not get the path to the skeletons application directory!".to_string(),
        )),
    }
}

/// Get the secret's encrypted nonce value.
fn get_secret_nonce(hash_path: &PathBuf) -> Result<[u8; 24], SkeletonsError> {
    let mut nonce_file = File::open(hash_path.join("nonce"))?;
    let mut raw_nonce = [0u8; 24];
    nonce_file.read_exact(&mut raw_nonce)?;

    Ok(raw_nonce)
}

/// Get the encrypted secret itself.
fn get_secret(hash_path: &PathBuf) -> Result<Vec<u8>, SkeletonsError> {
    let mut secret_file = File::open(hash_path.join("skeleton"))?;
    let mut encrypted_secret = Vec::new();
    secret_file.read_to_end(&mut encrypted_secret)?;

    Ok(encrypted_secret)
}

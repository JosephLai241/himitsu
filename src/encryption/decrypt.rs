//! Contains decryption functions for `himitsu`.

use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use ansi_term::Color;
use argon2;
use chacha20poly1305::{
    aead::{Aead, NewAead},
    Key, XChaCha20Poly1305, XNonce,
};
use directories::ProjectDirs;
use spinners::{Spinner, Spinners};

use crate::{authentication, errors::HimitsuError, utils::clipboard};

/// This enum contains variants for what should be done with the decrypted secret.
pub enum DecryptionMode {
    /// Decrypt and return the secret for editing if applicable.
    EditSecret,
    /// Decrypt and use the secret (copy it to the clipboard).
    UseSecret,
}

/// Decrypt a secret based on its SHA256 hash ID and copy it to the system clipboard.
pub fn decrypt_secret(
    decryption_mode: DecryptionMode,
    hash_id: &str,
    password: &str,
) -> Result<Option<String>, HimitsuError> {
    let mut decryption_spinner =
        Spinner::new(Spinners::Aesthetic, "Decrypting the secret...".into());

    let hash_path = get_secret_hash_path(hash_id)?;

    let salt = get_secret_salt(&hash_path)?;
    let raw_nonce = get_secret_nonce(&hash_path)?;
    let nonce = XNonce::from_slice(&raw_nonce);

    let encrypted_secret = &get_secret(&hash_path)?[..];

    let argon2_config = authentication::get_argon2_config();
    let key = argon2::hash_raw(password.as_bytes(), &salt, &argon2_config)?;

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

            match decryption_mode {
                DecryptionMode::EditSecret => Ok(Some(secret)),
                DecryptionMode::UseSecret => match clipboard::set_clipboard(secret) {
                    Ok(()) => {
                        println!(
                            "{}",
                            Color::Green
                                .bold()
                                .paint("\n📋 The secret is copied to your clipboard.")
                        );

                        Ok(None)
                    }
                    Err(error) => Err(error),
                },
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
            Err(HimitsuError::AEADDencryptionError(format!(
                "Secret decryption error: {}",
                error
            )))
        }
    }
}

/// Get the secret's SHA256 hash directory path.
fn get_secret_hash_path(hash_id: &str) -> Result<PathBuf, HimitsuError> {
    match ProjectDirs::from("", "", "himitsu") {
        Some(project_directory) => Ok(project_directory.data_dir().join("closet").join(hash_id)),
        None => Err(HimitsuError::PathError(
            "Could not get the path to the himitsu application directory!".to_string(),
        )),
    }
}

/// Get the secret's nonce value.
fn get_secret_nonce(hash_path: &Path) -> Result<[u8; 24], HimitsuError> {
    let mut nonce_file = File::open(hash_path.join("nonce"))?;
    let mut raw_nonce = [0u8; 24];
    nonce_file.read_exact(&mut raw_nonce)?;

    Ok(raw_nonce)
}

/// Get the secret's salt value.
fn get_secret_salt(hash_path: &Path) -> Result<[u8; 32], HimitsuError> {
    let mut nonce_file = File::open(hash_path.join("salt"))?;
    let mut salt = [0u8; 32];
    nonce_file.read_exact(&mut salt)?;

    Ok(salt)
}

/// Get the encrypted secret itself.
fn get_secret(hash_path: &Path) -> Result<Vec<u8>, HimitsuError> {
    let mut secret_file = File::open(hash_path.join("skeleton"))?;
    let mut encrypted_secret = Vec::new();
    secret_file.read_to_end(&mut encrypted_secret)?;

    Ok(encrypted_secret)
}

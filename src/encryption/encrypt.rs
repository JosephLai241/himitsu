//! Contains encryption functions for `skeletons`.

use ansi_term::Color;
use anyhow::bail;
use chacha20poly1305::{
    aead::{consts::U24, generic_array::GenericArray, Aead, NewAead},
    Key, XChaCha20Poly1305, XNonce,
};
use rand::{rngs::OsRng, RngCore};
use serde_json;
use spinners::{Spinner, Spinners};

use crate::{
    authentication::get_argon2_config,
    errors::SkeletonsError,
    models::{encryption::Encryption, metadata::Anatomy},
    utils::store,
};

/// Store the encrypted secret.
pub fn encrypt_secret(
    anatomy: &Anatomy,
    encryption_data: &Encryption,
    secret_value: &str,
) -> Result<(), SkeletonsError> {
    let mut encryption_spinner =
        Spinner::new(Spinners::Aesthetic, "Encrypting your secret...".into());

    let mut secret_salt = [0u8; 32];
    let mut secret_nonce = [0u8; 24];
    OsRng.fill_bytes(&mut secret_salt);
    OsRng.fill_bytes(&mut secret_nonce);

    // Generate a new hash for this particular secret.
    let argon2_config = get_argon2_config();
    let key = argon2::hash_raw(&encryption_data.password_hash, &secret_salt, &argon2_config)?;

    let cipher = XChaCha20Poly1305::new(Key::from_slice(&key));

    let nonce = XNonce::from_slice(&secret_nonce);

    let ciphertext = generate_ciphertext(&cipher, "Ciphertext", &nonce, secret_value.as_bytes())
        .map_or_else(
            |error| {
                encryption_spinner.stop_and_persist(
                    "❗️",
                    Color::Red
                        .bold()
                        .paint("SECRET ENCRYPTION FAILED.")
                        .to_string(),
                );
                bail!(error);
            },
            Ok,
        );
    let encrypted_anatomy = generate_ciphertext(
        &cipher,
        "Anatomy",
        &nonce,
        serde_json::to_string(anatomy)?.as_bytes(),
    )
    .map_or_else(
        |error| {
            encryption_spinner.stop_and_persist(
                "❗️",
                Color::Red
                    .bold()
                    .paint("ANATOMY ENCRYPTION FAILED.")
                    .to_string(),
            );
            bail!(error);
        },
        Ok,
    );

    encryption_spinner.stop_and_persist(
        "✅",
        Color::Green
            .bold()
            .paint("Successfully encrypted your secret.")
            .to_string(),
    );

    let mut write_spinner = Spinner::new(Spinners::Noise, "Storing your secret...".into());
    store::store_secret(ciphertext?, encrypted_anatomy?, nonce)?;

    write_spinner.stop_and_persist(
        "🔒",
        Color::Green
            .bold()
            .paint("Secret has been stored!")
            .to_string(),
    );

    Ok(())
}

/// Generate the ciphertext from the cipher, nonce, and secret.
fn generate_ciphertext(
    cipher: &XChaCha20Poly1305,
    item_type: &str,
    nonce: &GenericArray<u8, U24>,
    value: &[u8],
) -> Result<Vec<u8>, SkeletonsError> {
    cipher.encrypt(nonce, value).map_or_else(
        |error| {
            Err(SkeletonsError::AEADEncryptionError(format!(
                "{item_type} error: {}",
                error.to_string()
            )))
        },
        Ok,
    )
}
